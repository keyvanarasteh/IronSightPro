import React, { useState, useEffect, useMemo } from 'react';
import {
Shield,
Activity,
Cpu,
Network,
Database,
AlertTriangle,
Terminal,
Settings,
Search,
Eye,
Lock,
Unlock,
RefreshCw,
FileCode,
Zap,
CheckCircle2,
XCircle,
ChevronRight,
Filter
} from 'lucide-react';
import {
LineChart,
Line,
AreaChart,
Area,
XAxis,
YAxis,
CartesianGrid,
Tooltip,
ResponsiveContainer,
PieChart,
Pie,
Cell
} from 'recharts';

// --- Mock Data Generators (Reflecting Rust Logic) ---

const generateMockProcesses = (count = 50) => {
const names = ["systemd", "init", "sshd", "dbus-daemon", "nginx", "docker-proxy", "python3", "rust-analyzer", "code",
"firefox", "gnome-shell", "kernel_task", "malicious_sh", "hidden_miner"];
const paths = ["/usr/bin/", "/usr/sbin/", "/tmp/", "/home/user/.local/bin/", "/var/lib/"];

return Array.from({ length: count }, (_, i) => {
const pid = i === 0 ? 1 : Math.floor(Math.random() * 60000) + 1000;
const name = names[Math.floor(Math.random() * names.length)];
const cpu = parseFloat((Math.random() * (pid > 50000 ? 95 : 5)).toFixed(1));
const mem = Math.floor(Math.random() * 1024 * 1024 * 500);
const entropy = parseFloat((Math.random() * (pid > 50000 ? 2.5 : 1) + 4).toFixed(2));
const isSigned = Math.random() > 0.1;
const wxViolations = pid > 55000 ? Math.floor(Math.random() * 5) : 0;

// Heuristic Score Calculation (simplified version of Rust logic)
let score = 0;
if (entropy > 7.0) score += 30;
if (!isSigned) score += 20;
if (cpu > 90) score += 15;
if (wxViolations > 0) score += 40;
if (name.includes("miner") || name.includes("sh")) score += 50;

let level = "Low";
if (score > 70) level = "Critical";
else if (score > 40) level = "High";
else if (score > 20) level = "Medium";

return {
pid,
name,
exe: paths[Math.floor(Math.random() * paths.length)] + name,
cpu,
mem,
entropy,
isSigned,
wxViolations,
score,
level,
connections: Math.floor(Math.random() * 10),
suspicious_path: name.includes("tmp") || name.includes("hidden")
};
});
};

const INITIAL_PROCESSES = generateMockProcesses(40);

const COLORS = {
Critical: '#ef4444',
High: '#f97316',
Medium: '#facc15',
Low: '#22c55e',
};

const Dashboard = () => {
const [activeTab, setActiveTab] = useState('overview');
const [processes, setProcesses] = useState(INITIAL_PROCESSES);
const [search, setSearch] = useState('');
const [isScanning, setIsScanning] = useState(false);
const [lastScan, setLastScan] = useState(new Date().toLocaleTimeString());
const [history, setHistory] = useState(
Array.from({ length: 20 }, (_, i) => ({
time: i,
threats: 5 + Math.floor(Math.random() * 5),
avgScore: 10 + Math.random() * 10
}))
);

// Stats
const stats = useMemo(() => {
const critical = processes.filter(p => p.level === 'Critical').length;
const high = processes.filter(p => p.level === 'High').length;
const medium = processes.filter(p => p.level === 'Medium').length;
return { critical, high, medium, total: processes.length };
}, [processes]);

const filteredProcesses = useMemo(() => {
return processes
.filter(p =>
p.name.toLowerCase().includes(search.toLowerCase()) ||
p.pid.toString().includes(search)
)
.sort((a, b) => b.score - a.score);
}, [processes, search]);

const runScan = () => {
setIsScanning(true);
setTimeout(() => {
setProcesses(generateMockProcesses(40));
setLastScan(new Date().toLocaleTimeString());
setIsScanning(false);
setHistory(prev => [...prev.slice(1), {
time: prev.length,
threats: stats.critical + stats.high,
avgScore: processes.reduce((acc, curr) => acc + curr.score, 0) / processes.length
}]);
}, 1500);
};

useEffect(() => {
const interval = setInterval(runScan, 10000);
return () => clearInterval(interval);
}, [stats]);

return (
<div className="flex h-screen bg-slate-950 text-slate-200 overflow-hidden font-sans">
    {/* Sidebar */}
    <aside className="w-64 bg-slate-900 border-r border-slate-800 flex flex-col">
        <div className="p-6 border-b border-slate-800 flex items-center gap-3">
            <div className="bg-emerald-500 p-2 rounded-lg">
                <Shield className="w-6 h-6 text-slate-950" strokeWidth={2.5} />
            </div>
            <div>
                <h1 className="font-bold text-lg tracking-tight text-white">IronSight</h1>
                <p className="text-xs text-slate-500 font-mono">v0.1.0-alpha</p>
            </div>
        </div>

        <nav className="flex-1 p-4 space-y-2">
            <NavItem active={activeTab==='overview' } onClick={()=> setActiveTab('overview')} icon={
                <Activity size={18} />} label="Security Overview" />
                <NavItem active={activeTab==='processes' } onClick={()=> setActiveTab('processes')} icon={
                    <Cpu size={18} />} label="Process Explorer" />
                    <NavItem active={activeTab==='network' } onClick={()=> setActiveTab('network')} icon={
                        <Network size={18} />} label="Network Audit" />
                        <NavItem active={activeTab==='memory' } onClick={()=> setActiveTab('memory')} icon={
                            <Database size={18} />} label="Memory Watch" />
                            <NavItem active={activeTab==='config' } onClick={()=> setActiveTab('config')} icon={
                                <Settings size={18} />} label="Configuration" />
        </nav>

        <div className="p-4 border-t border-slate-800">
            <div className="flex items-center justify-between mb-2">
                <span className="text-xs font-semibold text-slate-500 uppercase tracking-wider">Watchdog Status</span>
                <span className="flex h-2 w-2 rounded-full bg-emerald-500 animate-pulse"></span>
            </div>
            <div className="bg-slate-950 rounded p-3 flex items-center gap-3">
                <CheckCircle2 size={16} className="text-emerald-500" />
                <span className="text-sm font-medium">Sentinel Active</span>
            </div>
        </div>
    </aside>

    {/* Main Content */}
    <main className="flex-1 overflow-y-auto relative">
        {/* Header */}
        <header
            className="sticky top-0 z-10 bg-slate-950/80 backdrop-blur-md border-b border-slate-800 px-8 py-4 flex items-center justify-between">
            <div className="flex items-center gap-6">
                <h2 className="text-xl font-semibold capitalize text-white">{activeTab.replace('_', ' ')}</h2>
                <div className="flex items-center gap-2 bg-slate-900 px-3 py-1 rounded-full border border-slate-800">
                    <span className="text-[10px] font-bold text-slate-500 uppercase tracking-widest">UID: 0</span>
                    <Lock size={12} className="text-emerald-500" />
                    <span className="text-[10px] font-bold text-emerald-500 uppercase tracking-widest">ROOT
                        PRIVILEGES</span>
                </div>
            </div>

            <div className="flex items-center gap-4">
                <div className="text-right">
                    <p className="text-[10px] text-slate-500 font-bold uppercase tracking-widest">Last Scan</p>
                    <p className="text-sm font-mono text-slate-300">{lastScan}</p>
                </div>
                <button onClick={runScan} disabled={isScanning} className={`p-2.5 rounded-xl border transition-all
                    ${isScanning ? 'bg-slate-800 border-slate-700'
                    : 'bg-emerald-500/10 border-emerald-500/20 hover:bg-emerald-500/20 text-emerald-500' }`}>
                    <RefreshCw size={20} className={isScanning ? 'animate-spin' : '' } />
                </button>
            </div>
        </header>

        <div className="p-8 space-y-8">
            {activeTab === 'overview' && (
            <>
                {/* Stats Grid */}
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                    <StatCard label="Critical Threats" value={stats.critical} icon={<AlertTriangle
                        className="text-red-500" />} color="border-red-500/50" trend="+0% vs last hour" />
                    <StatCard label="High Risk" value={stats.high} icon={<AlertTriangle className="text-orange-500" />}
                    color="border-orange-500/50" trend="+2 new detections" />
                    <StatCard label="W^X Violations" value={processes.reduce((a, b)=> a + b.wxViolations, 0)} icon={
                        <Database className="text-amber-500" />} color="border-amber-500/50" />
                        <StatCard label="Total Monitored" value={stats.total} icon={<Activity
                            className="text-blue-500" />} color="border-blue-500/50" trend="System stable" />
                </div>

                {/* Charts Row */}
                <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
                    <div className="lg:col-span-2 bg-slate-900/50 border border-slate-800 rounded-2xl p-6">
                        <h3
                            className="text-sm font-bold text-slate-400 uppercase tracking-widest mb-6 flex items-center gap-2">
                            <Zap size={16} /> Threat Score History
                        </h3>
                        <div className="h-[280px]">
                            <ResponsiveContainer width="100%" height="100%">
                                <AreaChart data={history}>
                                    <defs>
                                        <linearGradient id="colorScore" x1="0" y1="0" x2="0" y2="1">
                                            <stop offset="5%" stopColor="#10b981" stopOpacity={0.3} />
                                            <stop offset="95%" stopColor="#10b981" stopOpacity={0} />
                                        </linearGradient>
                                    </defs>
                                    <CartesianGrid strokeDasharray="3 3" stroke="#1e293b" vertical={false} />
                                    <XAxis dataKey="time" hide />
                                    <YAxis stroke="#475569" fontSize={12} tickLine={false} axisLine={false} />
                                    <Tooltip contentStyle={{ backgroundColor: '#0f172a' , border: '1px solid #1e293b' ,
                                        borderRadius: '8px' }} itemStyle={{ color: '#10b981' }} />
                                    <Area type="monotone" dataKey="avgScore" stroke="#10b981" fillOpacity={1}
                                        fill="url(#colorScore)" strokeWidth={2} />
                                </AreaChart>
                            </ResponsiveContainer>
                        </div>
                    </div>

                    <div className="bg-slate-900/50 border border-slate-800 rounded-2xl p-6">
                        <h3 className="text-sm font-bold text-slate-400 uppercase tracking-widest mb-6">Threat
                            Distribution</h3>
                        <div className="h-[280px] flex items-center justify-center">
                            <ResponsiveContainer width="100%" height="100%">
                                <PieChart>
                                    <Pie data={[ { name: 'Critical' , value: stats.critical }, { name: 'High' , value:
                                        stats.high }, { name: 'Medium' , value: stats.medium }, { name: 'Low' , value:
                                        stats.total - (stats.critical + stats.high + stats.medium) }, ]}
                                        innerRadius={60} outerRadius={80} paddingAngle={5} dataKey="value">
                                        <Cell fill={COLORS.Critical} />
                                        <Cell fill={COLORS.High} />
                                        <Cell fill={COLORS.Medium} />
                                        <Cell fill={COLORS.Low} />
                                    </Pie>
                                    <Tooltip contentStyle={{ backgroundColor: '#0f172a' , border: '1px solid #1e293b' ,
                                        borderRadius: '8px' }} />
                                </PieChart>
                            </ResponsiveContainer>
                        </div>
                    </div>
                </div>

                {/* Recent Activity Table */}
                <div className="bg-slate-900/50 border border-slate-800 rounded-2xl overflow-hidden">
                    <div className="px-6 py-4 border-b border-slate-800 flex items-center justify-between">
                        <h3 className="text-sm font-bold text-slate-400 uppercase tracking-widest">Active Incident
                            Watchlist</h3>
                        <button onClick={()=> setActiveTab('processes')} className="text-xs text-blue-400
                            hover:underline flex items-center gap-1">
                            View Full Snapshot
                            <ChevronRight size={14} />
                        </button>
                    </div>
                    <div className="overflow-x-auto">
                        <table className="w-full text-left">
                            <thead
                                className="bg-slate-950/50 text-slate-500 text-[10px] font-bold uppercase tracking-widest">
                                <tr>
                                    <th className="px-6 py-3">PID</th>
                                    <th className="px-6 py-3">Process</th>
                                    <th className="px-6 py-3">Entropy</th>
                                    <th className="px-6 py-3">W^X</th>
                                    <th className="px-6 py-3">Risk Level</th>
                                    <th className="px-6 py-3">Score</th>
                                    <th className="px-6 py-3 text-right">Action</th>
                                </tr>
                            </thead>
                            <tbody className="divide-y divide-slate-800">
                                {filteredProcesses.slice(0, 8).map((proc) => (
                                <tr key={proc.pid} className="hover:bg-slate-800/40 transition-colors group">
                                    <td className="px-6 py-4 font-mono text-xs text-slate-400">{proc.pid}</td>
                                    <td className="px-6 py-4">
                                        <div className="flex flex-col">
                                            <span className="text-sm font-semibold text-white">{proc.name}</span>
                                            <span
                                                className="text-[10px] text-slate-500 font-mono truncate max-w-[150px]">{proc.exe}</span>
                                        </div>
                                    </td>
                                    <td className="px-6 py-4 font-mono text-sm">
                                        <span className={proc.entropy> 7.5 ? 'text-red-400' : 'text-slate-300'}>
                                            {proc.entropy.toFixed(2)}
                                        </span>
                                    </td>
                                    <td className="px-6 py-4">
                                        {proc.wxViolations > 0 ? (
                                        <span
                                            className="bg-red-500/10 text-red-500 text-[10px] px-2 py-0.5 rounded font-bold border border-red-500/20">
                                            {proc.wxViolations} REGIONS
                                        </span>
                                        ) : (
                                        <span className="text-slate-600 text-[10px]">None</span>
                                        )}
                                    </td>
                                    <td className="px-6 py-4">
                                        <div className="flex items-center gap-2">
                                            <div className="w-2 h-2 rounded-full" style={{ backgroundColor:
                                                COLORS[proc.level] }}></div>
                                            <span className="text-xs font-bold uppercase" style={{ color:
                                                COLORS[proc.level] }}>{proc.level}</span>
                                        </div>
                                    </td>
                                    <td className="px-6 py-4">
                                        <div className="w-full bg-slate-800 rounded-full h-1.5 w-16 overflow-hidden">
                                            <div className="h-full rounded-full" style={{ width: `${Math.min(proc.score,
                                                100)}%`, backgroundColor: COLORS[proc.level] }}></div>
                                        </div>
                                    </td>
                                    <td className="px-6 py-4 text-right">
                                        <button className="p-2 text-slate-500 hover:text-white transition-colors">
                                            <Terminal size={16} />
                                        </button>
                                    </td>
                                </tr>
                                ))}
                            </tbody>
                        </table>
                    </div>
                </div>
            </>
            )}

            {activeTab === 'processes' && (
            <div className="space-y-6">
                <div className="flex items-center gap-4">
                    <div className="relative flex-1">
                        <Search className="absolute left-3 top-1/2 -translate-y-1/2 text-slate-500" size={18} />
                        <input type="text" placeholder="Search processes by name, PID, or path..."
                            className="w-full bg-slate-900 border border-slate-800 rounded-xl py-3 pl-11 pr-4 focus:outline-none focus:ring-2 focus:ring-emerald-500/50 text-white"
                            value={search} onChange={(e)=> setSearch(e.target.value)}
                        />
                    </div>
                    <button
                        className="flex items-center gap-2 bg-slate-900 border border-slate-800 px-4 py-3 rounded-xl text-slate-400 hover:text-white transition-colors">
                        <Filter size={18} />
                        <span>Filters</span>
                    </button>
                </div>

                <div className="bg-slate-900/50 border border-slate-800 rounded-2xl overflow-hidden">
                    <table className="w-full text-left">
                        <thead
                            className="bg-slate-950/50 text-slate-500 text-[10px] font-bold uppercase tracking-widest">
                            <tr>
                                <th className="px-6 py-4">Status</th>
                                <th className="px-6 py-4">PID</th>
                                <th className="px-6 py-4">Name</th>
                                <th className="px-6 py-4">CPU %</th>
                                <th className="px-6 py-4">Signature</th>
                                <th className="px-6 py-4">Entropy</th>
                                <th className="px-6 py-4">Score</th>
                                <th className="px-6 py-4 text-right">Detail</th>
                            </tr>
                        </thead>
                        <tbody className="divide-y divide-slate-800">
                            {filteredProcesses.map((proc) => (
                            <tr key={proc.pid} className="hover:bg-slate-800/40 transition-colors">
                                <td className="px-6 py-4">
                                    <div className={`w-2 h-2 rounded-full ${proc.score> 20 ? 'animate-pulse' : ''}`}
                                        style={{ backgroundColor: COLORS[proc.level] }}></div>
                                </td>
                                <td className="px-6 py-4 font-mono text-xs text-slate-400">{proc.pid}</td>
                                <td className="px-6 py-4 font-semibold text-white">{proc.name}</td>
                                <td className="px-6 py-4 font-mono text-sm">{proc.cpu}%</td>
                                <td className="px-6 py-4">
                                    {proc.isSigned ? (
                                    <span
                                        className="flex items-center gap-1.5 text-emerald-500 text-xs font-bold uppercase">
                                        <CheckCircle2 size={14} /> Trusted
                                    </span>
                                    ) : (
                                    <span
                                        className="flex items-center gap-1.5 text-red-400 text-xs font-bold uppercase">
                                        <XCircle size={14} /> Unsigned
                                    </span>
                                    )}
                                </td>
                                <td className="px-6 py-4 font-mono text-sm">{proc.entropy.toFixed(2)}</td>
                                <td className="px-6 py-4">
                                    <span className={`px-2 py-0.5 rounded text-[10px] font-bold border ${proc.score> 50
                                        ? 'bg-red-500/10 border-red-500/20 text-red-500' : 'bg-slate-800
                                        border-slate-700 text-slate-400'}`}>
                                        {proc.score.toFixed(1)}
                                    </span>
                                </td>
                                <td className="px-6 py-4 text-right">
                                    <button className="text-slate-500 hover:text-white">
                                        <Eye size={16} />
                                    </button>
                                </td>
                            </tr>
                            ))}
                        </tbody>
                    </table>
                </div>
            </div>
            )}

            {activeTab === 'config' && (
            <div className="max-w-4xl mx-auto space-y-6">
                <div className="bg-slate-900/50 border border-slate-800 rounded-2xl p-8">
                    <div className="flex items-center justify-between mb-8">
                        <div className="flex items-center gap-4">
                            <div className="bg-blue-500/10 p-3 rounded-2xl">
                                <Settings className="w-6 h-6 text-blue-500" />
                            </div>
                            <div>
                                <h3 className="text-xl font-bold text-white">Runtime Configuration</h3>
                                <p className="text-sm text-slate-500">Manage IronSight engine settings and scan
                                    thresholds.</p>
                            </div>
                        </div>
                        <button
                            className="bg-blue-600 hover:bg-blue-500 text-white px-6 py-2.5 rounded-xl font-semibold transition-all">
                            Save Changes
                        </button>
                    </div>

                    <div className="space-y-8">
                        <ConfigSection title="General Settings">
                            <ConfigField label="Scan Interval (Seconds)" value="5" />
                            <ConfigField label="Log Level" value="Info" type="select" options={['Trace', 'Debug'
                                , 'Info' , 'Warn' , 'Error' ]} />
                            <ConfigField label="Report Directory" value="/tmp/ironsight-reports" />
                        </ConfigSection>

                        <ConfigSection title="Threat Thresholds">
                            <ConfigField label="Entropy Alert Threshold" value="7.0" />
                            <ConfigField label="CPU Spike Detection (%)" value="90" />
                            <ConfigField label="Auto-Response Action" value="Log Only" type="select" options={['Log
                                Only', 'Suspend Process' , 'Kill Process' ]} />
                        </ConfigSection>

                        <ConfigSection title="Exclusions">
                            <div className="space-y-2">
                                <div className="flex flex-wrap gap-2">
                                    {['systemd', 'init', 'sshd', 'dbus-daemon'].map(tag => (
                                    <span key={tag}
                                        className="bg-slate-800 px-3 py-1 rounded-lg text-xs font-mono flex items-center gap-2">
                                        {tag}
                                        <XCircle size={12} className="text-slate-500 cursor-pointer" />
                                    </span>
                                    ))}
                                    <button
                                        className="px-3 py-1 rounded-lg border border-dashed border-slate-700 text-xs text-slate-500">+
                                        Add Process</button>
                                </div>
                            </div>
                        </ConfigSection>
                    </div>
                </div>

                <div
                    className="bg-slate-900/30 border border-slate-800 border-dashed rounded-2xl p-6 flex flex-col items-center justify-center text-center space-y-3">
                    <FileCode className="text-slate-600" size={32} />
                    <div>
                        <h4 className="text-slate-400 font-semibold">TOML Export</h4>
                        <p className="text-xs text-slate-600">Download the generated configuration file for your
                            `ironsight.toml`</p>
                    </div>
                    <button
                        className="text-xs font-bold text-blue-500 uppercase tracking-widest hover:text-blue-400 transition-colors">Download
                        config.toml</button>
                </div>
            </div>
            )}

            {(activeTab === 'network' || activeTab === 'memory') && (
            <div className="h-[60vh] flex flex-col items-center justify-center space-y-4 text-center">
                <div className="p-6 bg-slate-900 rounded-full border border-slate-800 shadow-xl shadow-slate-950">
                    <Zap size={48} className="text-slate-700 animate-pulse" />
                </div>
                <div>
                    <h3 className="text-xl font-bold text-slate-300">Analysis Module Initializing</h3>
                    <p className="text-slate-500 max-w-sm mx-auto mt-2">Connecting to IronSight backend via Unix Domain
                        Socket for live telemetry stream...</p>
                </div>
                <div className="flex gap-2">
                    {[0, 1, 2].map(i => (
                    <div key={i} className="w-1.5 h-1.5 rounded-full bg-slate-700 animate-bounce" style={{
                        animationDelay: `${i * 0.2}s` }}></div>
                    ))}
                </div>
            </div>
            )}
        </div>
    </main>
</div>
);
};

// --- Subcomponents ---

const NavItem = ({ active, icon, label, onClick }) => (
<button onClick={onClick} className={`w-full flex items-center gap-3 px-4 py-3 rounded-xl transition-all ${active
    ? 'bg-emerald-500/10 text-emerald-500 font-semibold shadow-sm shadow-emerald-500/10'
    : 'text-slate-400 hover:bg-slate-800 hover:text-slate-200' }`}>
    {icon}
    <span className="text-sm">{label}</span>
</button>
);

const StatCard = ({ label, value, icon, color, trend }) => (
<div className={`bg-slate-900/50 border border-slate-800 border-l-4 ${color} rounded-2xl p-5 hover:bg-slate-900
    transition-all cursor-default`}>
    <div className="flex items-center justify-between mb-3">
        <span className="text-[10px] font-bold text-slate-500 uppercase tracking-widest">{label}</span>
        {icon}
    </div>
    <div className="flex items-baseline gap-2">
        <span className="text-3xl font-bold text-white">{value}</span>
        {trend && <span className="text-[10px] text-slate-500 font-medium whitespace-nowrap">{trend}</span>}
    </div>
</div>
);

const ConfigSection = ({ title, children }) => (
<div className="space-y-4">
    <h4 className="text-xs font-bold text-slate-500 uppercase tracking-widest pb-2 border-b border-slate-800">{title}
    </h4>
    <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        {children}
    </div>
</div>
);

const ConfigField = ({ label, value, type = 'text', options = [] }) => (
<div className="space-y-1.5">
    <label className="text-xs text-slate-400 font-medium ml-1">{label}</label>
    {type === 'select' ? (
    <select
        className="w-full bg-slate-950 border border-slate-800 rounded-lg px-3 py-2 text-sm text-slate-300 focus:outline-none focus:ring-1 focus:ring-blue-500">
        {options.map(o => <option key={o} value={o}>{o}</option>)}
    </select>
    ) : (
    <input type="text" defaultValue={value}
        className="w-full bg-slate-950 border border-slate-800 rounded-lg px-3 py-2 text-sm text-slate-300 focus:outline-none focus:ring-1 focus:ring-blue-500" />
    )}
</div>
);

export default Dashboard;