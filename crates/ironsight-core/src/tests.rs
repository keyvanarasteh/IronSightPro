//! Unit tests for ironsight-core.

#[cfg(test)]
mod snapshot_tests {
    use crate::snapshot::build_snapshot;
    use crate::ProcessSnapshot;
    use sysinfo::System;

    fn take_snapshot() -> ProcessSnapshot {
        let mut sys = System::new_all();
        sys.refresh_all();
        // sysinfo needs a second refresh for accurate CPU readings
        std::thread::sleep(std::time::Duration::from_millis(200));
        sys.refresh_all();
        build_snapshot(&sys)
    }

    #[test]
    fn snapshot_has_processes() {
        let snap = take_snapshot();
        assert!(
            snap.processes.len() > 1,
            "Expected at least some processes, got {}",
            snap.processes.len()
        );
    }

    #[test]
    fn snapshot_has_system_info() {
        let snap = take_snapshot();
        assert!(snap.system_total_memory > 0);
        assert!(snap.cpu_count > 0);
    }

    #[test]
    fn snapshot_serialization_roundtrip() {
        let snap = take_snapshot();
        let json = serde_json::to_string(&snap).expect("Should serialize");
        let restored: ProcessSnapshot =
            serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(snap.processes.len(), restored.processes.len());
        assert_eq!(snap.cpu_count, restored.cpu_count);
    }

    #[test]
    fn by_cpu_is_descending() {
        let snap = take_snapshot();
        let sorted = snap.by_cpu();
        for window in sorted.windows(2) {
            assert!(
                window[0].cpu_percent >= window[1].cpu_percent,
                "CPU sort not descending: {} < {}",
                window[0].cpu_percent,
                window[1].cpu_percent
            );
        }
    }

    #[test]
    fn by_memory_is_descending() {
        let snap = take_snapshot();
        let sorted = snap.by_memory();
        for window in sorted.windows(2) {
            assert!(
                window[0].memory_bytes >= window[1].memory_bytes,
                "Memory sort not descending"
            );
        }
    }

    #[test]
    fn find_by_pid_returns_existing() {
        let snap = take_snapshot();
        if let Some((&pid, _)) = snap.processes.iter().next() {
            let found = snap.find_by_pid(pid);
            assert!(found.is_some(), "find_by_pid should find PID {pid}");
            assert_eq!(found.unwrap().pid, pid);
        }
    }

    #[test]
    fn find_by_pid_returns_none_for_missing() {
        let snap = take_snapshot();
        let found = snap.find_by_pid(u32::MAX);
        assert!(found.is_none(), "u32::MAX PID should not exist");
    }

    #[test]
    fn find_by_name_case_insensitive() {
        let snap = take_snapshot();
        if let Some(info) = snap.processes.values().next() {
            let name_upper = info.name.to_uppercase();
            let results = snap.find_by_name(&name_upper);
            assert!(
                !results.is_empty(),
                "Case-insensitive search for '{}' should find at least 1",
                name_upper
            );
        }
    }

    #[test]
    fn tree_builds_parent_child_map() {
        let snap = take_snapshot();
        let tree = snap.tree();
        let total_children: usize = tree.values().map(|v| v.len()).sum();
        assert!(
            total_children > 0,
            "Tree should have parent-child relationships"
        );
    }

    #[test]
    fn children_of_returns_correct_parent() {
        let snap = take_snapshot();
        for info in snap.processes.values() {
            if let Some(ppid) = info.parent_pid {
                let children = snap.children_of(ppid);
                let pids: Vec<u32> = children.iter().map(|c| c.pid).collect();
                assert!(
                    pids.contains(&info.pid),
                    "PID {} should be in children_of({})",
                    info.pid,
                    ppid
                );
                break;
            }
        }
    }

    #[test]
    fn total_memory_is_sum_of_parts() {
        let snap = take_snapshot();
        let manual_sum: u64 = snap.processes.values().map(|p| p.memory_bytes).sum();
        assert_eq!(snap.total_memory_bytes(), manual_sum);
    }
}

#[cfg(test)]
mod diff_tests {
    use std::collections::HashMap;

    use chrono::Utc;

    use crate::diff::ProcessDiff;
    use crate::process_info::{ProcStatus, ProcessInfo};
    use crate::snapshot::ProcessSnapshot;

    fn make_process(pid: u32, name: &str, cpu: f32, mem: u64) -> ProcessInfo {
        ProcessInfo {
            pid,
            parent_pid: None,
            name: name.to_string(),
            exe: None,
            cmd: vec![],
            cwd: None,
            status: ProcStatus::Running,
            cpu_percent: cpu,
            memory_bytes: mem,
            virtual_memory_bytes: 0,
            start_time: 0,
            run_time_secs: 0,
            threads: None,
            uid: None,
            gid: None,
            captured_at: Some(Utc::now()),
        }
    }

    fn make_snapshot(procs: Vec<ProcessInfo>) -> ProcessSnapshot {
        let mut map = HashMap::new();
        for p in procs {
            map.insert(p.pid, p);
        }
        ProcessSnapshot {
            processes: map,
            taken_at: Utc::now(),
            system_total_memory: 1024 * 1024 * 1024,
            system_used_memory: 512 * 1024 * 1024,
            cpu_count: 4,
        }
    }

    #[test]
    fn detects_spawned_processes() {
        let old = make_snapshot(vec![make_process(1, "init", 0.0, 1000)]);
        let new = make_snapshot(vec![
            make_process(1, "init", 0.0, 1000),
            make_process(42, "malware", 50.0, 99999),
        ]);

        let diff = ProcessDiff::compute(&old, &new);
        assert_eq!(diff.spawned.len(), 1);
        assert_eq!(diff.spawned[0].pid, 42);
        assert_eq!(diff.spawned[0].name, "malware");
    }

    #[test]
    fn detects_exited_processes() {
        let old = make_snapshot(vec![
            make_process(1, "init", 0.0, 1000),
            make_process(99, "temp_proc", 5.0, 5000),
        ]);
        let new = make_snapshot(vec![make_process(1, "init", 0.0, 1000)]);

        let diff = ProcessDiff::compute(&old, &new);
        assert_eq!(diff.exited.len(), 1);
        assert_eq!(diff.exited[0].pid, 99);
    }

    #[test]
    fn detects_cpu_change_above_threshold() {
        let old = make_snapshot(vec![make_process(1, "app", 5.0, 1000)]);
        let new = make_snapshot(vec![make_process(1, "app", 50.0, 1000)]);

        let diff = ProcessDiff::compute(&old, &new);
        assert_eq!(diff.changed.len(), 1);
        assert!((diff.changed[0].cpu_delta - 45.0).abs() < 0.01);
    }

    #[test]
    fn ignores_small_cpu_changes() {
        let old = make_snapshot(vec![make_process(1, "app", 5.0, 1000)]);
        let new = make_snapshot(vec![make_process(1, "app", 5.5, 1000)]);

        let diff = ProcessDiff::compute(&old, &new);
        assert!(
            diff.changed.is_empty(),
            "CPU delta of 0.5% should be below threshold"
        );
    }

    #[test]
    fn detects_memory_change_above_threshold() {
        let mib = 1024 * 1024;
        let old = make_snapshot(vec![make_process(1, "app", 0.0, 10 * mib)]);
        let new = make_snapshot(vec![make_process(1, "app", 0.0, 20 * mib)]);

        let diff = ProcessDiff::compute(&old, &new);
        assert_eq!(diff.changed.len(), 1);
        assert_eq!(diff.changed[0].memory_delta_bytes, 10 * mib as i64);
    }

    #[test]
    fn detects_status_change() {
        let mut old_proc = make_process(1, "app", 0.0, 1000);
        old_proc.status = ProcStatus::Running;
        let mut new_proc = make_process(1, "app", 0.0, 1000);
        new_proc.status = ProcStatus::Zombie;

        let old = make_snapshot(vec![old_proc]);
        let new = make_snapshot(vec![new_proc]);

        let diff = ProcessDiff::compute(&old, &new);
        assert_eq!(diff.changed.len(), 1);
        assert!(diff.changed[0].status_changed);
    }

    #[test]
    fn is_empty_when_nothing_changed() {
        let procs = vec![make_process(1, "init", 0.0, 1000)];
        let old = make_snapshot(procs.clone());
        let new = make_snapshot(procs);

        let diff = ProcessDiff::compute(&old, &new);
        assert!(diff.is_empty());
    }
}

#[cfg(test)]
mod filter_tests {
    use std::collections::HashMap;

    use chrono::Utc;

    use crate::filter::ProcessFilter;
    use crate::process_info::{ProcStatus, ProcessInfo};
    use crate::snapshot::ProcessSnapshot;

    fn make_process(pid: u32, name: &str, cpu: f32, mem_mib: f64) -> ProcessInfo {
        ProcessInfo {
            pid,
            parent_pid: Some(1),
            name: name.to_string(),
            exe: None,
            cmd: vec![],
            cwd: None,
            status: ProcStatus::Running,
            cpu_percent: cpu,
            memory_bytes: (mem_mib * 1024.0 * 1024.0) as u64,
            virtual_memory_bytes: 0,
            start_time: 0,
            run_time_secs: 0,
            threads: None,
            uid: None,
            gid: None,
            captured_at: Some(Utc::now()),
        }
    }

    fn test_snapshot() -> ProcessSnapshot {
        let procs = vec![
            make_process(1, "init", 0.1, 2.0),
            make_process(10, "chrome", 25.0, 500.0),
            make_process(11, "chrome-helper", 8.0, 200.0),
            make_process(20, "firefox", 15.0, 350.0),
            make_process(30, "code", 5.0, 120.0),
            make_process(40, "bash", 0.2, 5.0),
        ];
        let mut map = HashMap::new();
        for p in procs {
            map.insert(p.pid, p);
        }
        ProcessSnapshot {
            processes: map,
            taken_at: Utc::now(),
            system_total_memory: 16 * 1024 * 1024 * 1024,
            system_used_memory: 8 * 1024 * 1024 * 1024,
            cpu_count: 8,
        }
    }

    #[test]
    fn filter_by_name() {
        let snap = test_snapshot();
        let results = ProcessFilter::new(&snap).name_contains("chrome").collect();
        assert_eq!(results.len(), 2); // chrome + chrome-helper
    }

    #[test]
    fn filter_by_name_case_insensitive() {
        let snap = test_snapshot();
        let results = ProcessFilter::new(&snap).name_contains("CHROME").collect();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn filter_by_cpu() {
        let snap = test_snapshot();
        let results = ProcessFilter::new(&snap).cpu_above(10.0).collect();
        // chrome (25.0) + firefox (15.0)
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn filter_by_memory() {
        let snap = test_snapshot();
        let results = ProcessFilter::new(&snap).memory_above_mib(300.0).collect();
        // chrome (500) + firefox (350)
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn filter_chained() {
        let snap = test_snapshot();
        let results = ProcessFilter::new(&snap)
            .name_contains("chrome")
            .cpu_above(10.0)
            .collect();
        // Only "chrome" (25.0 CPU), not "chrome-helper" (8.0 CPU)
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "chrome");
    }

    #[test]
    fn filter_no_matches() {
        let snap = test_snapshot();
        let results = ProcessFilter::new(&snap)
            .name_contains("nonexistent_app")
            .collect();
        assert!(results.is_empty());
    }

    #[test]
    fn filter_by_parent() {
        let snap = test_snapshot();
        let results = ProcessFilter::new(&snap).parent_pid(1).collect();
        assert_eq!(results.len(), 6);
    }

    #[test]
    fn filter_by_status() {
        let snap = test_snapshot();
        let results = ProcessFilter::new(&snap)
            .status(ProcStatus::Running)
            .collect();
        assert_eq!(results.len(), 6);
    }

    // ── New tests for sort + limit (Step 8) ──────────────────────────────

    #[test]
    fn filter_sort_by_cpu_descending() {
        let snap = test_snapshot();
        let results = ProcessFilter::new(&snap).sort_by_cpu().collect();
        for window in results.windows(2) {
            assert!(window[0].cpu_percent >= window[1].cpu_percent);
        }
    }

    #[test]
    fn filter_sort_by_memory_descending() {
        let snap = test_snapshot();
        let results = ProcessFilter::new(&snap).sort_by_memory().collect();
        for window in results.windows(2) {
            assert!(window[0].memory_bytes >= window[1].memory_bytes);
        }
    }

    #[test]
    fn filter_limit_truncates() {
        let snap = test_snapshot();
        let results = ProcessFilter::new(&snap)
            .sort_by_cpu()
            .limit(3)
            .collect();
        assert_eq!(results.len(), 3);
        // First should be highest CPU (chrome at 25.0)
        assert_eq!(results[0].name, "chrome");
    }

    #[test]
    fn filter_sort_and_limit_combined() {
        let snap = test_snapshot();
        let results = ProcessFilter::new(&snap)
            .cpu_above(1.0)
            .sort_by_memory()
            .limit(2)
            .collect();
        assert!(results.len() <= 2);
    }
}

#[cfg(test)]
mod process_info_tests {
    use crate::process_info::{ProcStatus, ProcessInfo};
    use chrono::Utc;
    use std::time::Duration;

    #[test]
    fn memory_mib_conversion() {
        let p = ProcessInfo {
            pid: 1,
            parent_pid: None,
            name: "test".into(),
            exe: None,
            cmd: vec![],
            cwd: None,
            status: ProcStatus::Running,
            cpu_percent: 0.0,
            memory_bytes: 100 * 1024 * 1024, // 100 MiB
            virtual_memory_bytes: 0,
            start_time: 0,
            run_time_secs: 0,
            threads: None,
            uid: None,
            gid: None,
            captured_at: Some(Utc::now()),
        };
        assert!((p.memory_mib() - 100.0).abs() < 0.01);
    }

    #[test]
    fn age_returns_correct_duration() {
        let p = ProcessInfo {
            pid: 1,
            parent_pid: None,
            name: "test".into(),
            exe: None,
            cmd: vec![],
            cwd: None,
            status: ProcStatus::Running,
            cpu_percent: 0.0,
            memory_bytes: 0,
            virtual_memory_bytes: 0,
            start_time: 0,
            run_time_secs: 3600,
            threads: None,
            uid: None,
            gid: None,
            captured_at: None,
        };
        assert_eq!(p.age(), Duration::from_secs(3600));
    }

    #[test]
    fn proc_status_equality() {
        assert_eq!(ProcStatus::Running, ProcStatus::Running);
        assert_ne!(ProcStatus::Running, ProcStatus::Zombie);
    }

    #[test]
    fn display_format_process() {
        let p = ProcessInfo {
            pid: 42,
            parent_pid: None,
            name: "test-app".into(),
            exe: None,
            cmd: vec![],
            cwd: None,
            status: ProcStatus::Running,
            cpu_percent: 12.5,
            memory_bytes: 50 * 1024 * 1024,
            virtual_memory_bytes: 0,
            start_time: 0,
            run_time_secs: 0,
            threads: None,
            uid: None,
            gid: None,
            captured_at: None,
        };
        let display = format!("{}", p);
        assert!(display.contains("test-app"));
        assert!(display.contains("42"));
        assert!(display.contains("12.5"));
    }

    #[test]
    fn display_format_status() {
        assert_eq!(format!("{}", ProcStatus::Running), "Running");
        assert_eq!(format!("{}", ProcStatus::Zombie), "Zombie");
    }
}

#[cfg(test)]
mod spy_tests {
    use crate::ProcessSpy;
    use std::time::Duration;

    #[test]
    fn spy_snapshot_returns_processes() {
        let spy = ProcessSpy::new();
        let snap = spy.snapshot();
        assert!(snap.processes.len() > 1);
    }

    #[test]
    fn spy_system_info_has_data() {
        let spy = ProcessSpy::new();
        let info = spy.system_info();
        assert!(info.cpu_count > 0);
        assert!(info.total_memory_bytes > 0);
        assert!(info.process_count > 0);
    }

    #[test]
    fn spy_env_vars_for_self() {
        let spy = ProcessSpy::new();
        let pid = std::process::id();
        let vars = spy.env_vars(pid);
        assert!(vars.is_ok());
        assert!(!vars.unwrap().is_empty());
    }

    #[test]
    fn spy_env_vars_for_missing_pid() {
        let spy = ProcessSpy::new();
        let result = spy.env_vars(u32::MAX);
        assert!(result.is_err());
    }

    #[test]
    fn spy_kill_missing_pid_returns_error() {
        let spy = ProcessSpy::new();
        let result = spy.kill(u32::MAX);
        assert!(result.is_err());
    }

    #[test]
    fn spy_monitor_start_stop() {
        let mut spy = ProcessSpy::new();
        assert!(spy.start_monitoring(Duration::from_millis(100)).is_ok());
        assert!(spy.is_monitoring());
        std::thread::sleep(Duration::from_millis(300));
        spy.stop_monitoring();
        assert!(!spy.is_monitoring());
    }

    #[test]
    fn spy_double_start_returns_error() {
        let mut spy = ProcessSpy::new();
        assert!(spy.start_monitoring(Duration::from_millis(100)).is_ok());
        assert!(spy.start_monitoring(Duration::from_millis(100)).is_err());
        spy.stop_monitoring();
    }

    #[test]
    fn spy_wait_for_exit_timeout() {
        let spy = ProcessSpy::new();
        // PID 1 (init) should not exit — should timeout
        let result = spy.wait_for_exit(1, Duration::from_millis(200));
        assert!(result.is_err());
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn spy_fd_count_for_self() {
        let spy = ProcessSpy::new();
        let pid = std::process::id();
        let count = spy.fd_count(pid);
        assert!(count.is_some());
        assert!(count.unwrap() > 0, "Our process should have open FDs");
    }
}
