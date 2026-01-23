import {useEffect, useState} from 'react';
import {Plus, X, Box, Square, Play, Settings2} from 'lucide-react';
import type {Body, BodyFormState, Equation, Shape, Vector} from './types';
import {createBodyState, calculateRectProperties} from './utils/physics';

import {SimulationCanvas} from './components/SimulationCanvas';
import {EquationEditor} from './components/EquationEditor';
import {BodyForm} from './components/BodyForm';
import {useNotifications} from "./components/Notification.tsx";
import {Dropdown} from "./components/Dropdown.tsx";
import {Templates} from "./templates.ts";

export default function App() {
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
            setLeftOpen(false);
            setRightSidebarView('list');
        } else {
            setRightOpen(false);
            setLeftOpen(true);
        }
    };

    const toggleLeftSidebar = () => {
        if (!leftOpen) {
            setLeftOpen(true);
            setRightOpen(false);
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

    return (
        <div className="flex flex-col h-screen w-full bg-white text-gray-800 font-sans overflow-hidden">
            <header className="
                h-12 border-b border-gray-200 flex
                items-center justify-between px-4
                bg-white z-10 shrink-0"
            >
                <div>
                    <div className="m-auto flex gap-8">
                        <div className="flex items-center gap-2">
                            <div className="p-1 bg-gray-100 rounded cursor-pointer hover:bg-gray-200"
                                 onClick={toggleLeftSidebar}>
                                <Box size={16} className="text-gray-600"/>
                            </div>
                            <div className="flex items-center gap-2">
                                <h2 className={""}></h2>
                                <Dropdown options={Templates.map((e) => e.name)} value={template}
                                          onChange={handleTemplateChange}
                                          placeholder={"Template:"}/>
                            </div>
                        </div>
                    </div>
                </div>
                <div>
                    <div className="m-auto flex gap-8">
                        <div className="flex items-center gap-2">
                            <h1 className="text-sm font-semibold tracking-wide text-gray-700 uppercase">
                                Iterative
                                <span className="ms-1 text-blue-500">Physics</span>
                            </h1>
                        </div>
                    </div>
                </div>
                <div
                    className={`flex items-center gap-4`}
                >
                    <Dropdown options={[...bodies.map((n) => n.name),"None"]} value={track}
                              onChange={(it) => {
                                  if (it == "None") {
                                      setTrack(null);
                                  } else {
                                      setTrack(it);
                                  }
                              }}
                              placeholder={"Follow:"}/>
                    <button
                        className={`${running ? "bg-orange-600 hover:bg-orange-700" : "bg-lime-600 hover:bg-lime-700"} white rounded-md p-1`}
                        onClick={() => setRunning(!running)}
                    >
                        {running ? <Square color={"white"} size={24}/> : <Play color={"white"} size={24}/>}
                    </button>
                    <button
                        onClick={toggleRightSidebar}
                        className={`px-3 py-1.5 rounded-md text-sm font-medium transition-colors flex items-center gap-2 ${rightOpen ? 'bg-blue-100 text-blue-700' : 'bg-gray-100 text-gray-700 hover:bg-gray-200'}`}
                    >
                        {rightOpen ? <X size={16}/> : null}
                        <span>Bodies</span>
                    </button>
                    <button
                        className={`rounded-md p-1 hover:bg-gray-100`}
                    >
                        <Settings2 size={24}/>
                    </button>
                </div>
            </header>
            <div className="flex-1 flex overflow-hidden relative">
                {/* Left Sidebar */}
                <div
                    className={`bg-gray-50 border-r border-gray-200 flex flex-col transition-all duration-300 ease-in-out ${leftOpen ? 'w-80 opacity-100' : 'w-0 opacity-0 overflow-hidden'}`}>
                    <EquationEditor equations={equations} setEquations={setEquations}/>
                </div>

                {/* Center Canvas */}
                <SimulationCanvas running={running} bodies={bodies} equations={equations.map((x) => x.text)}
                                  onError={(msg) => {
                                      notifications.error(msg);
                                      setRunning(false);
                                  }}
                                  track={track}
                />

                {/* Right Sidebar */}
                <div
                    className={`bg-white border-l border-gray-200 flex flex-col transition-all duration-300 ease-in-out shadow-xl z-20 ${rightOpen ? 'w-80 translate-x-0' : 'w-0 translate-x-full overflow-hidden absolute right-0 h-full'}`}>
                    {rightSidebarView === 'list' && (
                        <div className="flex flex-col h-full">
                            <div className="p-4 border-b border-gray-100 bg-gray-50/50">
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
                                        className="p-3 border border-gray-100 rounded bg-white hover:border-blue-400 hover:shadow-md cursor-pointer transition-all flex justify-between items-center group mb-2"
                                    >
                                        <div>
                                            <div className="font-medium text-sm text-gray-700">{body.name}</div>
                                            <div
                                                className="text-[10px] text-gray-400 font-mono mt-0.5">Mass: {body.properties.mass}kg
                                            </div>
                                        </div>
                                        <div className="w-4 h-4 rounded-full border border-gray-200 shadow-sm"
                                             style={{backgroundColor: body.color || '#3b82f6'}}></div>
                                    </div>
                                ))}
                            </div>
                            <div className="p-4 border-t border-gray-100 bg-gray-50">
                                <button onClick={() => setRightSidebarView('create')}
                                        className="w-full py-2 bg-blue-600 hover:bg-blue-700 text-white rounded text-sm font-medium flex items-center justify-center gap-2 shadow-sm">
                                    <Plus size={16}/> Add New Body
                                </button>
                            </div>
                        </div>
                    )}

                    {(rightSidebarView === 'create' || rightSidebarView === 'edit') && (
                        <BodyForm
                            mode={rightSidebarView}
                            initialBody={rightSidebarView === 'edit' ? bodies.find(b => b.name === editingId) : null}
                            onSave={handleSaveBody}
                            onCancel={() => setRightSidebarView('list')}
                            onDelete={handleDeleteBody}
                        />
                    )}
                </div>
            </div>
        </div>
    );
}