import React, { useState, useEffect, useMemo } from 'react';
import {
    Shield,
    Activity,
    Terminal,
    AlertTriangle,
    Cpu,
    HardDrive,
    Zap,
    Search,
    MoreVertical,
    Skull,
    Pause,
    Download,
    ShieldAlert,
    ChevronRight,
    ChevronDown,
    Settings,
    Bell,
    Sun,
    Moon,
    Files,
    Vibrate,
    Globe,
    Database,
    Code,
    X,
    Info,
    CheckCircle,
    Filter
} from 'lucide-react';

// --- Mock Data ---
const MOCK_PROCESSES = [
    { pid: 1204, name: 'systemd', cpu: 0.1, mem: '12MB', score: 5, status: 'Running', user: 'root', path: '/usr/lib/systemd/systemd', hash: 'e3b0c442...', entropy: 4.2, signed: true },
    { pid: 4492, name: 'chrome.exe', cpu: 4.2, mem: '450MB', score: 12, status: 'Running', user: 'user', path: 'C:\\Program Files\\Google\\Chrome\\chrome.exe', hash: '8f3a1c...', entropy: 5.1, signed: true },
    { pid: 8812, name: 'svchost.exe', cpu: 0.5, mem: '24MB', score: 45, status: 'Running', user: 'SYSTEM', path: 'C:\\Windows\\System32\\svchost.exe', hash: 'a1b2c3...', entropy: 6.8, signed: false },
    { pid: 9901, name: 'unknown_agent', cpu: 12.5, mem: '120MB', score: 88, status: 'Running', user: 'user', path: '/tmp/ldr_stage2', hash: 'f2e3d4...', entropy: 7.9, signed: false, sockets: ['192.168.1.5:4444 (TCP)', '45.12.33.1:80 (TCP)'] },
];

const MOCK_ALERTS = [
    { id: 1, time: '10:04:22', type: 'KERNEL', msg: 'VirtualProtect(RX) detected in PID 9901', severity: 'high', code: 'E_MEM_01' },
    { id: 2, time: '10:04:25', type: 'STRINGS', msg: 'Suspicious URL found in PID 9901 heap', severity: 'critical', code: 'E_STR_09' },
    { id: 3, time: '10:05:01', type: 'AUTH', msg: 'Unsigned binary started: temp_installer.exe', severity: 'medium', code: 'W_SIG_02' },
];

const MOCK_SOCKETS = [
    { pid: 9901, process: 'unknown_agent', local: '192.168.1.5:49211', remote: '45.12.33.1:80', state: 'ESTABLISHED', proto: 'TCP' },
    { pid: 4492, process: 'chrome.exe', local: '192.168.1.5:55201', remote: '142.250.190.46:443', state: 'ESTABLISHED', proto: 'TCP' },
    { pid: 8812, process: 'svchost.exe', local: '0.0.0.0:135', remote: '*:*', state: 'LISTENING', proto: 'TCP' },
];

export default function App() {
    const [selectedPid, setSelectedPid] = useState(null);
    const [theme, setTheme] = useState('dark');
    const [activeActivity, setActiveActivity] = useState('explorer'); // explorer, search, security, heuristics
    const [activeView, setActiveView] = useState('Process_Tree');
    const [bottomPanelTab, setBottomPanelTab] = useState('telemetry'); // telemetry, problems, output
    const [modal, setModal] = useState(null); // { type: 'kill' | 'dump', pid: number }
    const [notifications, setNotifications] = useState([]);
    const [searchQuery, setSearchQuery] = useState('');

    const toggleTheme = () => setTheme(theme === 'dark' ? 'light' : 'dark');
    const addNotification = (type, msg) => {
        const id = Date.now();
        setNotifications(prev => [...prev, { id, type, msg }]);
        setTimeout(() => setNotifications(prev => prev.filter(n => n.id !== id)), 5000);
    };

    const selectedProcess = MOCK_PROCESSES.find(p => p.pid === selectedPid);

    const themeClasses = theme === 'dark'
        ? 'bg-[#1e1e1e] text-[#cccccc] border-[#2b2b2b]'
        : 'bg-[#ffffff] text-[#333333] border-[#e1e1e1]';

    const sidebarBg = theme === 'dark' ? 'bg-[#252526]' : 'bg-[#f3f3f3]';
    const activityBarBg = theme === 'dark' ? 'bg-[#333333]' : 'bg-[#2c2c2c]';

    return (
        <div className={`flex h-screen w-full font-sans text-sm selection:bg-blue-500/30 transition-colors duration-200 ${theme === 'dark' ? 'dark' : ''} ${themeClasses}`}>

            {/* 1. Activity Bar */}
            <div className={`w-12 flex flex-col items-center py-2 space-y-4 z-30 ${activityBarBg} text-white/60`}>
                <div className="p-2 text-white cursor-pointer hover:scale-110 transition-transform"><Shield size={24} /></div>
                <ActivityIcon icon={<Files size={22} />} active={activeActivity === 'explorer'} onClick={() => setActiveActivity('explorer')} />
                <ActivityIcon icon={<Search size={22} />} active={activeActivity === 'search'} onClick={() => setActiveActivity('search')} />
                <ActivityIcon icon={<ShieldAlert size={22} />} active={activeActivity === 'security'} onClick={() => setActiveActivity('security')} />
                <ActivityIcon icon={<Zap size={22} />} active={activeActivity === 'heuristics'} onClick={() => setActiveActivity('heuristics')} />
                <div className="flex-grow" />
                <ActivityIcon icon={<Settings size={22} />} />
                <ActivityIcon icon={theme === 'dark' ? <Sun size={20} /> : <Moon size={20} />} onClick={toggleTheme} />
            </div>

            {/* 2. Side Bar (Explorer/Search/Security) */}
            <div className={`w-64 flex flex-col border-r ${sidebarBg} ${theme === 'dark' ? 'border-[#2b2b2b]' : 'border-[#e1e1e1]'}`}>
                <div className="px-4 py-2 flex justify-between items-center uppercase text-[11px] font-bold tracking-wider opacity-60">
                    <span>{activeActivity === 'explorer' ? 'Explorer' : activeActivity.toUpperCase()}</span>
                    <MoreVertical size={14} className="cursor-pointer hover:opacity-100" />
                </div>

                <div className="flex-grow overflow-auto">
                    {activeActivity === 'explorer' && (
                        <>
                            <CollapsibleSection title="Active Workspace">
                                <SidebarItem icon={<ChevronDown size={14} />} label="Live_Telemetry" active />
                                <div className="ml-4 space-y-0.5">
                                    <SidebarItem icon={<Activity size={14} />} label="Process_Tree" activeSub={activeView === 'Process_Tree'} onClick={() => setActiveView('Process_Tree')} />
                                    <SidebarItem icon={<Terminal size={14} />} label="Kernel_Logs" activeSub={activeView === 'Kernel_Logs'} onClick={() => setActiveView('Kernel_Logs')} />
                                    <SidebarItem icon={<Globe size={14} />} label="Network_Sockets" activeSub={activeView === 'Network_Sockets'} onClick={() => setActiveView('Network_Sockets')} />
                                </div>
                            </CollapsibleSection>
                            <CollapsibleSection title="Memory Analysis" collapsed>
                                <SidebarItem icon={<ChevronRight size={14} />} label="Heap_Scan" />
                                <SidebarItem icon={<ChevronRight size={14} />} label="Stack_Trace" />
                            </CollapsibleSection>
                        </>
                    )}

                    {activeActivity === 'search' && (
                        <div className="p-4 space-y-4">
                            <div className="bg-black/10 dark:bg-white/5 border border-inherit rounded flex items-center px-2 py-1">
                                <Search size={14} className="opacity-40 mr-2" />
                                <input
                                    autoFocus
                                    className="bg-transparent border-none outline-none w-full text-xs"
                                    placeholder="Search processes..."
                                    value={searchQuery}
                                    onChange={(e) => setSearchQuery(e.target.value)}
                                />
                            </div>
                            <div className="text-[11px] opacity-40 uppercase font-bold tracking-tighter">Results</div>
                            {MOCK_PROCESSES.filter(p => p.name.includes(searchQuery)).map(p => (
                                <div key={p.pid} onClick={() => { setSelectedPid(p.pid); setActiveView('Process_Tree'); }} className="flex items-center gap-2 px-2 py-1 hover:bg-blue-500/10 cursor-pointer rounded">
                                    <Activity size={12} className="text-blue-500" />
                                    <span className="truncate">{p.name}</span>
                                    <span className="ml-auto opacity-30">{p.pid}</span>
                                </div>
                            ))}
                        </div>
                    )}

                    {activeActivity === 'security' && (
                        <div className="p-0">
                            <div className="px-4 py-2 border-b border-inherit bg-orange-500/5 flex items-center justify-between">
                                <span className="text-[10px] font-bold text-orange-500 uppercase">Risk Level: Elevated</span>
                                <Filter size={12} className="opacity-40" />
                            </div>
                            {MOCK_ALERTS.map(alert => (
                                <div key={alert.id} className="p-3 border-b border-inherit hover:bg-white/5 cursor-pointer group">
                                    <div className="flex items-center gap-2 mb-1">
                                        <ShieldAlert size={14} className={alert.severity === 'critical' ? 'text-red-500' : 'text-orange-500'} />
                                        <span className="text-[11px] font-bold">{alert.type}</span>
                                        <span className="ml-auto text-[9px] opacity-40 group-hover:opacity-100">{alert.time}</span>
                                    </div>
                                    <div className="text-[11px] leading-relaxed opacity-80 mb-2">{alert.msg}</div>
                                    <div className="text-[9px] font-mono opacity-30">ERR_CODE: {alert.code}</div>
                                </div>
                            ))}
                        </div>
                    )}
                </div>
            </div>

            {/* 3. Editor Area */}
            <div className="flex-grow flex flex-col min-w-0 relative">
                <div className="flex-grow flex flex-col min-h-0 overflow-hidden">
                    <div className={`flex border-b h-9 items-center ${theme === 'dark' ? 'bg-[#252526] border-[#2b2b2b]' : 'bg-[#f3f3f3] border-[#e1e1e1]'}`}>
                        <EditorTab label={`${activeView}.rs`} active icon={<Code size={12} />} />
                        <EditorTab label="Threat_Heuristics.json" icon={<Database size={12} />} />
                    </div>

                    <div className="flex-grow flex overflow-hidden">
                        <div className="flex-grow flex flex-col overflow-hidden">
                            <div className="overflow-auto flex-grow">
                                {activeView === 'Process_Tree' && <ProcessListView selectedPid={selectedPid} setSelectedPid={setSelectedPid} theme={theme} />}
                                {activeView === 'Kernel_Logs' && <KernelLogView theme={theme} />}
                                {activeView === 'Network_Sockets' && <SocketListView theme={theme} />}
                            </div>
                        </div>

                        {/* Inspection Sidebar */}
                        <div className={`w-80 border-l flex flex-col ${theme === 'dark' ? 'bg-[#1e1e1e] border-[#2b2b2b]' : 'bg-white border-[#e1e1e1]'}`}>
                            <div className={`px-4 py-2 border-b flex justify-between items-center text-[11px] font-bold uppercase opacity-60 ${theme === 'dark' ? 'bg-[#252526] border-[#2b2b2b]' : 'bg-[#f3f3f3] border-[#e1e1e1]'}`}>
                                <span>Properties</span>
                                <Settings size={14} />
                            </div>
                            <div className="p-4 overflow-auto space-y-6">
                                {!selectedProcess ? (
                                    <div className="h-full flex flex-col items-center justify-center opacity-30 italic">
                                        Select a process to inspect metadata
                                    </div>
                                ) : (
                                    <>
                                        <InspectionSection title="Identification">
                                            <MetaRow label="Path" value={selectedProcess.path} />
                                            <MetaRow label="SHA-256" value={selectedProcess.hash} />
                                            <MetaRow label="Signature" value={selectedProcess.signed ? "Valid (Microsoft)" : "Unsigned / Unknown"} highlight={!selectedProcess.signed} />
                                        </InspectionSection>

                                        <InspectionSection title="Live Analysis">
                                            <MetaRow label="Entropy" value={`${selectedProcess.entropy} (High)`} />
                                            <MetaRow label="Status" value={selectedProcess.status} />
                                            <MetaRow label="Memory Usage" value={selectedProcess.mem} />
                                        </InspectionSection>

                                        <div className="space-y-2 pt-4">
                                            <ActionButton icon={<Pause size={14} />} label="Suspend Thread" onClick={() => addNotification('warning', `Process ${selectedProcess.pid} suspended.`)} />
                                            <ActionButton icon={<Download size={14} />} label="Extract Minidump" onClick={() => setModal({ type: 'dump', pid: selectedProcess.pid })} />
                                            <ActionButton icon={<Skull size={14} />} label="Kill Process" danger onClick={() => setModal({ type: 'kill', pid: selectedProcess.pid })} />
                                        </div>
                                    </>
                                )}
                            </div>
                        </div>
                    </div>
                </div>

                {/* Bottom Panel */}
                <div className={`h-48 border-t flex flex-col ${theme === 'dark' ? 'bg-[#1e1e1e] border-[#2b2b2b]' : 'bg-white border-[#e1e1e1]'}`}>
                    <div className="flex px-4 border-b border-inherit h-9 items-center space-x-6 text-[11px] uppercase font-bold tracking-tight opacity-60">
                        <PanelTab label="Telemetry" active={bottomPanelTab === 'telemetry'} onClick={() => setBottomPanelTab('telemetry')} />
                        <PanelTab label="Problems" active={bottomPanelTab === 'problems'} count={MOCK_ALERTS.length} onClick={() => setBottomPanelTab('problems')} />
                        <PanelTab label="Output" active={bottomPanelTab === 'output'} onClick={() => setBottomPanelTab('output')} />
                    </div>
                    <div className="flex-grow p-3 font-mono text-[12px] overflow-auto space-y-1 bg-black/5 dark:bg-black/20">
                        {bottomPanelTab === 'telemetry' && (
                            <>
                                {MOCK_ALERTS.map(a => (
                                    <div key={a.id} className="flex gap-4">
                                        <span className="opacity-30">[{a.time}]</span>
                                        <span className={a.severity === 'critical' ? 'text-red-500 font-bold' : 'text-yellow-500'}>[{a.type}]</span>
                                        <span>{a.msg}</span>
                                    </div>
                                ))}
                                <div className="text-blue-500 animate-pulse">_ system_monitoring_active: listening for syscalls...</div>
                            </>
                        )}
                        {bottomPanelTab === 'problems' && (
                            <div className="space-y-2">
                                {MOCK_ALERTS.map(a => (
                                    <div key={a.id} className="flex items-start gap-3 p-1 hover:bg-white/5 rounded">
                                        <AlertTriangle size={14} className={a.severity === 'critical' ? 'text-red-500' : 'text-orange-500'} />
                                        <div className="flex flex-col">
                                            <div className="flex gap-4">
                                                <span className="font-bold">{a.msg}</span>
                                                <span className="opacity-40">{a.code}</span>
                                            </div>
                                            <span className="opacity-30 italic text-[10px]">Location: PID {9901} &gt; heap_buffer_x04</span>
                                        </div>
                                    </div>
                                ))}
                            </div>
                        )}
                        {bottomPanelTab === 'output' && (
                            <div className="opacity-60 italic">-- Initializing eBPF probe manager... OK<br />-- Attaching to sys_enter_mprotect... OK<br />-- Ring buffer initialized at 0x4f22...</div>
                        )}
                    </div>
                </div>

                {/* Status Bar */}
                <div className="h-6 flex items-center px-3 justify-between text-white text-[11px] bg-[#007acc]">
                    <div className="flex items-center space-x-4">
                        <div className="flex items-center gap-1.5 bg-white/10 px-2 h-full">
                            <Shield size={12} />
                            <span>Ready</span>
                        </div>
                        <span>master*</span>
                        <div className="flex items-center gap-1 cursor-pointer hover:bg-white/10 px-1" onClick={() => setBottomPanelTab('problems')}>
                            <AlertTriangle size={12} />
                            <span>{MOCK_ALERTS.length}</span>
                        </div>
                    </div>
                    <div className="flex items-center space-x-4">
                        <span>UTF-8</span>
                        <span>Rust (IronVigil)</span>
                        <div className="flex items-center gap-1">
                            <Activity size={12} />
                            <span>Live Telemetry</span>
                        </div>
                    </div>
                </div>

                {/* --- MODAL SYSTEM --- */}
                {modal && (
                    <div className="absolute inset-0 bg-black/60 backdrop-blur-sm z-50 flex items-center justify-center p-4">
                        <div className={`w-full max-w-sm border rounded-lg shadow-2xl p-6 ${theme === 'dark' ? 'bg-[#252526] border-[#333]' : 'bg-white border-[#ccc]'}`}>
                            <div className="flex items-center gap-3 mb-4">
                                <div className={`p-2 rounded-full ${modal.type === 'kill' ? 'bg-red-500/20 text-red-500' : 'bg-blue-500/20 text-blue-500'}`}>
                                    {modal.type === 'kill' ? <Skull size={24} /> : <Download size={24} />}
                                </div>
                                <h3 className="text-lg font-bold uppercase tracking-tight">
                                    {modal.type === 'kill' ? 'Neutralize Target?' : 'Generate Forensic Dump?'}
                                </h3>
                            </div>
                            <p className="opacity-60 mb-6 leading-relaxed">
                                {modal.type === 'kill'
                                    ? `Are you sure you want to terminate PID ${modal.pid}? This action is immediate and will stop all execution threads.`
                                    : `Generate a full minidump of PID ${modal.pid}. This will capture all heap and stack data for offline analysis.`}
                            </p>
                            <div className="flex justify-end gap-3">
                                <button
                                    onClick={() => setModal(null)}
                                    className="px-4 py-2 rounded text-xs hover:bg-white/5 transition-colors border border-transparent hover:border-inherit"
                                >
                                    Cancel
                                </button>
                                <button
                                    onClick={() => {
                                        addNotification(modal.type === 'kill' ? 'error' : 'success', `Action executed for PID ${modal.pid}`);
                                        setModal(null);
                                    }}
                                    className={`px-4 py-2 rounded text-xs font-bold transition-all ${modal.type === 'kill'
                                        ? 'bg-red-500 hover:bg-red-600 text-white shadow-lg shadow-red-500/20'
                                        : 'bg-blue-500 hover:bg-blue-600 text-white shadow-lg shadow-blue-500/20'
                                        }`}
                                >
                                    Confirm Action
                                </button>
                            </div>
                        </div>
                    </div>
                )}

                {/* --- NOTIFICATION STACK --- */}
                <div className="absolute top-4 right-4 z-[60] flex flex-col gap-2 pointer-events-none">
                    {notifications.map(n => (
                        <div key={n.id} className={`pointer-events-auto min-w-[240px] p-3 rounded-lg border shadow-xl flex items-center gap-3 animate-in slide-in-from-right duration-300 ${n.type === 'error' ? 'bg-red-950/90 border-red-500 text-red-100' :
                            n.type === 'warning' ? 'bg-orange-950/90 border-orange-500 text-orange-100' :
                                'bg-emerald-950/90 border-emerald-500 text-emerald-100'
                            }`}>
                            {n.type === 'error' ? <X size={18} /> : n.type === 'warning' ? <AlertTriangle size={18} /> : <CheckCircle size={18} />}
                            <span className="text-xs font-medium">{n.msg}</span>
                            <button onClick={() => setNotifications(prev => prev.filter(x => x.id !== n.id))} className="ml-auto opacity-40 hover:opacity-100">
                                <X size={14} />
                            </button>
                        </div>
                    ))}
                </div>

            </div>
        </div>
    );
}

// --- Dynamic View Components ---

function ProcessListView({ selectedPid, setSelectedPid, theme }) {
    return (
        <table className="w-full text-left border-collapse border-spacing-0">
            <thead className={`sticky top-0 z-10 text-[11px] uppercase tracking-wider ${theme === 'dark' ? 'bg-[#1e1e1e] text-white/40 border-[#2b2b2b]' : 'bg-white text-black/40 border-[#e1e1e1]'}`}>
                <tr>
                    <th className="px-4 py-2 border-b font-medium border-inherit">PID</th>
                    <th className="px-4 py-2 border-b font-medium border-inherit">Process Name</th>
                    <th className="px-4 py-2 border-b font-medium border-inherit">CPU %</th>
                    <th className="px-4 py-2 border-b font-medium border-inherit">Risk Level</th>
                    <th className="px-4 py-2 border-b font-medium border-inherit">User</th>
                </tr>
            </thead>
            <tbody className="divide-y border-inherit">
                {MOCK_PROCESSES.map((p) => (
                    <tr
                        key={p.pid}
                        onClick={() => setSelectedPid(p.pid)}
                        className={`group cursor-pointer text-[13px] ${selectedPid === p.pid ? (theme === 'dark' ? 'bg-[#37373d]' : 'bg-[#e4e6f1]') : (theme === 'dark' ? 'hover:bg-[#2a2d2e]' : 'hover:bg-[#f0f0f0]')}`}
                    >
                        <td className="px-4 py-1.5 font-mono text-blue-400">{p.pid}</td>
                        <td className="px-4 py-1.5 flex items-center gap-2">
                            <Activity size={14} className={p.score > 50 ? 'text-red-500' : 'text-blue-500'} />
                            <span className="font-semibold">{p.name}</span>
                            {p.score > 70 && <AlertTriangle size={12} className="text-orange-500 animate-pulse" />}
                        </td>
                        <td className="px-4 py-1.5 opacity-80">{p.cpu}%</td>
                        <td className="px-4 py-1.5 text-[11px]">
                            <div className="flex items-center gap-2">
                                <div className="w-16 h-1 bg-black/20 rounded-full overflow-hidden">
                                    <div className={`h-full ${p.score > 70 ? 'bg-red-500' : 'bg-blue-500'}`} style={{ width: `${p.score}%` }} />
                                </div>
                                <span className={p.score > 70 ? 'text-red-500 font-bold' : ''}>{p.score}</span>
                            </div>
                        </td>
                        <td className="px-4 py-1.5 opacity-60">{p.user}</td>
                    </tr>
                ))}
            </tbody>
        </table>
    );
}

function KernelLogView({ theme }) {
    return (
        <div className="p-4 space-y-2 font-mono text-xs">
            <div className="opacity-40 border-b border-inherit pb-2 mb-4">-- Kernel Syscall Audit Log (eBPF Stream) --</div>
            {Array.from({ length: 15 }).map((_, i) => (
                <div key={i} className="flex gap-4 hover:bg-white/5 p-1 rounded transition-colors group">
                    <span className="text-blue-500/50">0x0042F{i}</span>
                    <span className="text-emerald-500 font-bold group-hover:underline cursor-pointer">mmap</span>
                    <span className="opacity-60 text-emerald-500/80">addr=0x7fff.. len=4096 prot=PROT_READ|PROT_EXEC</span>
                    <span className="ml-auto opacity-30 italic">PID: {9000 + i}</span>
                </div>
            ))}
        </div>
    )
}

function SocketListView({ theme }) {
    return (
        <table className="w-full text-left border-collapse border-spacing-0">
            <thead className={`sticky top-0 z-10 text-[11px] uppercase tracking-wider ${theme === 'dark' ? 'bg-[#1e1e1e] text-white/40 border-[#2b2b2b]' : 'bg-white text-black/40 border-[#e1e1e1]'}`}>
                <tr>
                    <th className="px-4 py-2 border-b font-medium border-inherit">PID</th>
                    <th className="px-4 py-2 border-b font-medium border-inherit">Local Address</th>
                    <th className="px-4 py-2 border-b font-medium border-inherit">Remote Address</th>
                    <th className="px-4 py-2 border-b font-medium border-inherit">State</th>
                    <th className="px-4 py-2 border-b font-medium border-inherit">Proto</th>
                </tr>
            </thead>
            <tbody className="divide-y border-inherit text-[12px] font-mono">
                {MOCK_SOCKETS.map((s, i) => (
                    <tr key={i} className={theme === 'dark' ? 'hover:bg-[#2a2d2e]' : 'hover:bg-[#f0f0f0]'}>
                        <td className="px-4 py-2 text-blue-400 font-bold">{s.pid}</td>
                        <td className="px-4 py-2">{s.local}</td>
                        <td className="px-4 py-2 text-emerald-500 font-bold">{s.remote}</td>
                        <td className={`px-4 py-2 font-bold ${s.state === 'ESTABLISHED' ? 'text-emerald-500' : 'text-yellow-500'}`}>{s.state}</td>
                        <td className="px-4 py-2 opacity-50">{s.proto}</td>
                    </tr>
                ))}
            </tbody>
        </table>
    )
}

// --- Visual Subcomponents ---

function ActivityIcon({ icon, active, onClick }) {
    return (
        <div
            onClick={onClick}
            className={`p-2 cursor-pointer transition-all border-l-2 hover:scale-105 active:scale-95 ${active
                ? 'text-white border-white'
                : 'text-white/40 border-transparent hover:text-white/80'
                }`}>
            {icon}
        </div>
    );
}

function SidebarItem({ icon, label, activeSub = false, onClick }) {
    return (
        <div
            onClick={onClick}
            className={`flex items-center gap-2 px-4 py-1 cursor-pointer text-[13px] transition-colors ${activeSub ? 'bg-blue-500/10 text-blue-400 border-r-2 border-blue-500' : 'hover:bg-black/5 dark:hover:bg-white/5'
                }`}>
            {icon}
            <span>{label}</span>
        </div>
    );
}

function PanelTab({ label, active, count, onClick }) {
    return (
        <div
            onClick={onClick}
            className={`cursor-pointer h-full flex items-center px-2 gap-2 transition-all ${active
                ? 'border-b-2 border-blue-500 text-blue-500'
                : 'hover:opacity-100'
                }`}
        >
            {label}
            {count !== undefined && (
                <span className="bg-blue-500 text-white rounded-full px-1 text-[8px] min-w-[12px] text-center">
                    {count}
                </span>
            )}
        </div>
    );
}

function CollapsibleSection({ title, children, collapsed = false }) {
    return (
        <div className="mt-2">
            <div className="px-2 py-1 flex items-center gap-1 cursor-pointer uppercase text-[11px] font-bold opacity-80 hover:bg-white/5 transition-colors">
                {collapsed ? <ChevronRight size={14} /> : <ChevronDown size={14} />}
                {title}
            </div>
            {!collapsed && <div>{children}</div>}
        </div>
    );
}

function EditorTab({ label, active, icon }) {
    return (
        <div className={`px-4 h-full flex items-center gap-2 text-[12px] border-r cursor-pointer border-inherit transition-all ${active
            ? 'bg-[#1e1e1e] dark:bg-[#1e1e1e] text-blue-400 border-t-2 border-t-blue-500'
            : 'opacity-50 hover:bg-white/10'
            }`}>
            {icon}
            {label}
        </div>
    );
}

function InspectionSection({ title, children }) {
    return (
        <div className="space-y-2 border-b border-inherit pb-4 last:border-0">
            <h3 className="text-[10px] uppercase font-bold opacity-40 pb-1">{title}</h3>
            <div className="space-y-3">{children}</div>
        </div>
    );
}

function MetaRow({ label, value, highlight = false }) {
    return (
        <div>
            <div className="text-[10px] opacity-50 uppercase font-bold">{label}</div>
            <div className={`text-[12px] break-all font-mono leading-relaxed ${highlight ? 'text-red-500 font-bold' : ''}`}>
                {value}
            </div>
        </div>
    );
}

function ActionButton({ icon, label, danger = false, onClick }) {
    return (
        <button
            onClick={onClick}
            className={`w-full flex items-center justify-center gap-2 py-1.5 rounded text-[11px] font-bold border transition-all active:scale-95 ${danger
                ? 'bg-red-500/10 border-red-500/30 text-red-500 hover:bg-red-500/20 shadow-sm'
                : 'bg-blue-500/10 border-blue-500/30 text-blue-400 hover:bg-blue-500/20'
                }`}>
            {icon} {label.toUpperCase()}
        </button>
    );
}