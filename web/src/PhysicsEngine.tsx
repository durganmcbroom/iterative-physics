import {useEffect, useState} from 'react';
import {Plus, X, Box, Square, Play, GithubIcon, Menu, ChevronLeft} from 'lucide-react';
import type {Body, BodyFormState, Equation, Shape, Vector} from './types';
import {createBodyState, calculateRectProperties} from './utils/physics';

import {SimulationCanvas} from './components/SimulationCanvas';
import {EquationEditor} from './components/EquationEditor';
import {BodyForm} from './components/BodyForm';
import {useNotifications} from "./components/Notification.tsx";
import {Dropdown} from "./components/Dropdown.tsx";
import {Templates} from "./templates.ts";
import {useWasm} from "./hooks/useWasm.ts";

export default function PhysicsEngine() {
    useWasm()

    // --- Layout State ---
    const [leftOpen, setLeftOpen] = useState(true);
    const [rightOpen, setRightOpen] = useState(false);
    const [rightSidebarView, setRightSidebarView] = useState<'list' | 'create' | 'edit'>('list');
    const [template, setTemplate] = useState(Templates[0].name);
    const [track, setTrack] = useState<string | null>(null);

    // --- Domain State ---
    const [editingId, setEditingId] = useState<string | null>(null);
    const [equations, setEquations] = useState<Equation[]>([{id: 1, text: ""}]);
    const [bodies, setBodies] = useState<Body[]>([
        {
            name: "A",
            color: "#3b82f6",
            shape: {type: 'Rectangle', width: 50, height: 50},
            properties: calculateRectProperties(10, 50, 50),
            linear: createBodyState<Vector>({x: 0, y: 0}, {x: 0, y: 0}),
            angular: createBodyState<number>(0, 0),
        }
    ]);

    const [running, setRunning] = useState(false);

    const notifications = useNotifications();

    useEffect(() => {
        handleTemplateChange(Templates[0].name)
    }, [])

    // --- Actions ---

    const toggleRightSidebar = () => {
        if (!rightOpen) {
            setRightOpen(true);
            setLeftOpen(false); // Close left when opening right on mobile
            setRightSidebarView('list');
        } else {
            setRightOpen(false);
        }
    };

    const toggleLeftSidebar = () => {
        if (!leftOpen) {
            setLeftOpen(true);
            setRightOpen(false); // Close right when opening left
        } else {
            setLeftOpen(false);
        }
    };

    const handleSaveBody = (formData: BodyFormState) => {
        const props = calculateRectProperties(formData.mass, formData.height, formData.width);
        const shape: Shape = {type: 'Rectangle', width: formData.width, height: formData.height};
        const linear = createBodyState<Vector>(
            {x: formData.posX, y: formData.posY},
            {x: formData.velX, y: formData.velY},
        );
        const angular = createBodyState<number>(formData.rot, 0);

        if (rightSidebarView === 'create') {
            const newBody: Body = {
                name: formData.name,
                color: formData.color,
                shape,
                properties: props,
                linear,
                angular,
            };
            setBodies([...bodies, newBody]);
        } else if (rightSidebarView === 'edit' && editingId !== null) {
            setBodies(prev => prev.map(b => b.name === editingId ? {
                ...b,
                name: formData.name,
                color: formData.color,
                shape,
                properties: props,
                linear: {...b.linear, displacement: linear.displacement, velocity: linear.velocity},
                angular: angular
            } : b));
        }
        setRightSidebarView('list');
        // Optional: Close sidebar on mobile after save
        if (window.innerWidth < 768) setRightOpen(false);
    };

    const handleDeleteBody = () => {
        if (editingId) {
            setBodies(prev => prev.filter(b => b.name !== editingId));
            setRightSidebarView('list');
        }
    };

    const handleTemplateChange = (template: string) => {
        setTemplate(template);
        let find = Templates.find((e) => e.name == template)!.state;
        setBodies(find.bodies as Body[]);

        setEquations([...find.equations.map((eq, i) => {
            return {
                id: i,
                text: eq
            }
        }), {id: find.equations.length + 1, text: ""}])
    }

    const closeSidebars = () => {
        setLeftOpen(false);
        setRightOpen(false);
    };

    return (
        <div className="flex flex-col h-[100dvh] w-full bg-white text-gray-800 font-sans overflow-hidden">

            {/* --- Header --- */}
            <header className="
                min-h-14 border-b border-gray-200 flex flex-nowrap
                items-center justify-between px-2 sm:px-4 py-2 gap-2
                bg-white z-20 shrink-0 relative shadow-sm"
            >
                {/* Left Section: Menu & Template */}
                <div className="flex items-center gap-2 flex-1 min-w-0">
                    <button
                        className={`p-2 rounded shrink-0 cursor-pointer transition-colors ${leftOpen ? 'bg-gray-200' : 'bg-gray-100 hover:bg-gray-200'}`}
                        onClick={toggleLeftSidebar}
                        aria-label="Toggle Equations"
                    >
                        <Box size={20} className="text-gray-600"/>
                    </button>

                    <div className="hidden sm:block p-2 shrink-0 bg-gray-100 rounded cursor-pointer hover:bg-gray-200">
                        <a href={"https://github.com/durganmcbroom/iterative-physics"}>
                            <GithubIcon size={20} className="text-gray-600"/>
                        </a>
                    </div>

                    <div className="flex-1 max-w-[150px] sm:max-w-[200px]">
                        <Dropdown
                            options={Templates.map((e) => e.name)}
                            value={template}
                            onChange={handleTemplateChange}
                            placeholder={"Template"}
                        />
                    </div>
                </div>

                {/* Center Section: Title */}
                {/* Changed to lg:flex to ensure it hides on medium screens to prevent overlap */}
                <div className="hidden lg:flex items-center justify-center px-4 shrink-0">
                    <h1 className="text-sm font-semibold tracking-wide text-gray-700 uppercase">
                        Iterative
                        <span className="ms-1 text-blue-500">Physics</span>
                    </h1>
                </div>

                {/* Right Section: Controls & Bodies Toggle */}
                <div className="flex items-center gap-2 justify-end flex-1 min-w-fit">
                    <div className=" hidden sm:block">
                        <Dropdown
                            options={[...bodies.map((n) => n.name), "None"]}
                            value={track}
                            onChange={(it) => setTrack(it === "None" ? null : it)}
                            placeholder={"Follow"}
                        />
                    </div>

                    <button
                        className={`${running ? "bg-orange-600 hover:bg-orange-700" : "bg-lime-600 hover:bg-lime-700"} text-white rounded-md p-2 transition-colors shadow-sm shrink-0`}
                        onClick={() => setRunning(!running)}
                    >
                        {running ? <Square size={20} fill="currentColor"/> : <Play size={20} fill="currentColor"/>}
                    </button>

                    <button
                        onClick={toggleRightSidebar}
                        className={`px-3 py-2 shrink-0 rounded-md text-sm font-medium transition-colors flex items-center gap-2 ${rightOpen ? 'bg-blue-100 text-blue-700' : 'bg-gray-100 text-gray-700 hover:bg-gray-200'}`}
                    >
                        <span className="hidden sm:inline">Bodies</span>
                        {rightOpen ? <X size={20}/> : <Menu size={20}/>}
                    </button>
                </div>
            </header>

            {/* --- Main Content Area --- */}
            <div className="flex-1 flex overflow-hidden relative w-full">

                {/* Mobile Backdrop */}
                {(leftOpen || rightOpen) && (
                    <div
                        className="fixed inset-0 bg-black/20 backdrop-blur-sm z-30 md:hidden transition-opacity"
                        onClick={closeSidebars}
                    />
                )}

                {/* --- Left Sidebar (Equations) --- */}
                <div
                    className={`
                        fixed inset-y-0 left-0 z-40 bg-gray-50 border-r border-gray-200 flex flex-col 
                        transition-all duration-300 ease-in-out h-full shadow-xl md:shadow-none
                        w-[85vw] sm:w-80 md:relative md:h-full
                        ${leftOpen ? 'translate-x-0' : '-translate-x-full md:w-0 md:translate-x-0 md:opacity-0 md:overflow-hidden'}
                    `}
                >
                    <div className="md:hidden flex items-center justify-between p-4 border-b bg-white">
                        <span className="font-semibold text-gray-700">Equations</span>
                        <button onClick={() => setLeftOpen(false)} className="p-1 hover:bg-gray-100 rounded">
                            <ChevronLeft size={20} />
                        </button>
                    </div>

                    <div className="flex-1 overflow-y-auto">
                        <EquationEditor equations={equations} setEquations={setEquations}/>
                    </div>
                </div>

                {/* --- Center Canvas --- */}
                {/* Wrapper div removed as requested */}
                <SimulationCanvas
                    running={running}
                    bodies={bodies}
                    equations={equations.map((x) => x.text)}
                    onError={(msg) => {
                        notifications.error(msg);
                        setRunning(false);
                    }}
                    track={track}
                />

                {/* --- Right Sidebar (Bodies) --- */}
                <div
                    className={`
                        fixed inset-y-0 right-0 z-40 bg-white border-l border-gray-200 flex flex-col 
                        transition-all duration-300 ease-in-out h-full shadow-xl md:shadow-none
                        w-[85vw] sm:w-80 md:relative md:h-full
                        ${rightOpen ? 'translate-x-0' : 'translate-x-full md:w-0 md:translate-x-0 md:opacity-0 md:overflow-hidden'}
                    `}
                >
                    <div className="md:hidden flex items-center justify-between p-4 border-b bg-gray-50">
                        <span className="font-semibold text-gray-700">
                             {rightSidebarView === 'list' ? 'Scene Objects' : 'Edit Body'}
                        </span>
                        <button onClick={() => setRightOpen(false)} className="p-1 hover:bg-gray-200 rounded">
                            <X size={20} />
                        </button>
                    </div>

                    {rightSidebarView === 'list' && (
                        <div className="flex flex-col h-full overflow-hidden">
                            <div className="hidden md:block p-4 border-b border-gray-100 bg-gray-50/50">
                                <h2 className="text-sm font-semibold text-gray-800">Scene Objects</h2>
                                <p className="text-xs text-gray-500">{bodies.length} active bodies</p>
                            </div>
                            <div className="flex-1 overflow-y-auto p-2">
                                {bodies.map(body => (
                                    <div
                                        key={body.name}
                                        onClick={() => {
                                            setEditingId(body.name);
                                            setRightSidebarView('edit');
                                        }}
                                        className="p-3 border border-gray-100 rounded bg-white hover:border-blue-400 hover:shadow-md cursor-pointer transition-all flex justify-between items-center group mb-2 active:scale-95 transform"
                                    >
                                        <div>
                                            <div className="font-medium text-sm text-gray-700">{body.name}</div>
                                            <div className="text-[10px] text-gray-400 font-mono mt-0.5">
                                                Mass: {body.properties.mass}kg
                                            </div>
                                        </div>
                                        <div className="w-5 h-5 rounded-full border border-gray-200 shadow-sm"
                                             style={{backgroundColor: body.color || '#3b82f6'}}></div>
                                    </div>
                                ))}
                            </div>
                            <div className="p-4 border-t border-gray-100 bg-gray-50 safe-area-pb">
                                <button onClick={() => setRightSidebarView('create')}
                                        className="w-full py-3 bg-blue-600 hover:bg-blue-700 text-white rounded-lg text-sm font-medium flex items-center justify-center gap-2 shadow-sm active:bg-blue-800">
                                    <Plus size={18}/> Add New Body
                                </button>
                            </div>
                        </div>
                    )}

                    {(rightSidebarView === 'create' || rightSidebarView === 'edit') && (
                        <div className="flex-1 overflow-y-auto">
                            <BodyForm
                                mode={rightSidebarView}
                                initialBody={rightSidebarView === 'edit' ? bodies.find(b => b.name === editingId) : null}
                                onSave={handleSaveBody}
                                onCancel={() => setRightSidebarView('list')}
                                onDelete={handleDeleteBody}
                            />
                        </div>
                    )}
                </div>
            </div>
        </div>
    );
}