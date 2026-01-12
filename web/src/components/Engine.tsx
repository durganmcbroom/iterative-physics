// import React, { useState, useEffect, useRef } from 'react';
// import { Plus, X, Box, ArrowLeft, ZoomIn, Move, Save, Trash2 } from 'lucide-react';
//
// // --- Types & Interfaces ---
//
// interface Vector {
//     x: number;
//     y: number;
// }
//
// interface BodyState<T> {
//     displacement: T;
//     velocity: T;
// }
//
// interface BodyProperties {
//     mass: number;
//     moi: number;
// }
//
// interface Shape {
//     type: 'Rectangle';
//     width: number;
//     height: number;
// }
//
// interface Body {
//     name: string;
//     color: string;
//     shape: Shape;
//     properties: BodyProperties;
//     linear: BodyState<Vector>;
//     angular: BodyState<number>;
// }
//
// interface Equation {
//     id: number;
//     text: string;
// }
//
// interface BodyFormState {
//     name: string;
//     color: string;
//     mass: number;
//     width: number;
//     height: number;
//     posX: number;
//     posY: number;
//     velX: number;
//     velY: number;
// }
//
// interface ViewTransform {
//     x: number;
//     y: number;
//     scale: number;
// }
//
// // --- Helper Functions ---
// const createBodyState = <T,>(displacement: T, velocity: T): BodyState<T> => ({
//     displacement,
//     velocity,
//     // acceleration,
// });
//
// const calculateRectProperties = (mass: number, height: number, width: number): BodyProperties => {
//     const moi = (mass / 12.0) * (Math.pow(height, 2) + Math.pow(width, 2));
//     return { mass, moi };
// };
//
// export default function App() {
//     // --- State ---
//     const [leftOpen, setLeftOpen] = useState<boolean>(true);
//     const [rightOpen, setRightOpen] = useState<boolean>(false);
//
//     // rightSidebarView: 'list' | 'create' | 'edit'
//     const [rightSidebarView, setRightSidebarView] = useState<'list' | 'create' | 'edit'>('list');
//     const [editingId, setEditingId] = useState<string | null>(null);
//
//     // Equations State: Always start with one empty "ghost" box
//     const [equations, setEquations] = useState<Equation[]>([{ id: 1, text: "" }]);
//
//     // Scene State
//     const [bodies, setBodies] = useState<Body[]>([
//         {
//             name: "Demo_block",
//             color: "#3b82f6", // Default blue
//             shape: { type: 'Rectangle', width: 50, height: 50 },
//             properties: calculateRectProperties(10, 50, 50),
//             linear: createBodyState<Vector>({ x: 0, y: 0 }, { x: 0, y: 0 }),
//             angular: createBodyState<number>(0, 0),
//         }
//     ]);
//
//     // Form State (Shared for Create and Edit)
//     const [bodyForm, setBodyForm] = useState<BodyFormState>({
//         name: "New Body",
//         color: "#3b82f6",
//         mass: 1,
//         width: 40,
//         height: 40,
//         posX: 0,
//         posY: 0,
//         velX: 10,
//         velY: 5,
//     });
//
//     // Canvas Refs
//     const canvasRef = useRef<HTMLCanvasElement | null>(null);
//     const containerRef = useRef<HTMLDivElement | null>(null);
//
//     // Viewport Transform Refs
//     const viewTransform = useRef<ViewTransform>({ x: 300, y: 300, scale: 1 });
//     const isDragging = useRef<boolean>(false);
//     const lastMousePos = useRef<Vector>({ x: 0, y: 0 });
//
//     // --- Handlers ---
//
//     const toggleRightSidebar = () => {
//         if (!rightOpen) {
//             setRightOpen(true);
//             setLeftOpen(false);
//             setRightSidebarView('list');
//         } else {
//             setRightOpen(false);
//             setLeftOpen(true);
//         }
//     };
//
//     const toggleLeftSidebar = () => {
//         if (!leftOpen) {
//             setLeftOpen(true);
//             setRightOpen(false);
//         } else {
//             setLeftOpen(false);
//         }
//     };
//
//     // Equation Handlers
//     const handleEquationChange = (id: number, newText: string) => {
//         setEquations(prev => {
//             const updated = prev.map(eq => eq.id === id ? { ...eq, text: newText } : eq);
//
//             // If we typed in the last box, add a new ghost box
//             const lastEq = updated[updated.length - 1];
//             if (lastEq.text.trim() !== "") {
//                 return [...updated, { id: Date.now() + Math.random(), text: "" }];
//             }
//             return updated;
//         });
//     };
//
//     const handleEquationBlur = (id: number) => {
//         setEquations(prev => {
//             // Never remove the only remaining box
//             if (prev.length <= 1) return prev;
//
//             const index = prev.findIndex(eq => eq.id === id);
//             const isLast = index === prev.length - 1;
//
//             // If it's empty and NOT the last box (ghost box), remove it
//             if (!isLast && prev[index].text.trim() === "") {
//                 return prev.filter(eq => eq.id !== id);
//             }
//             return prev;
//         });
//     };
//
//     // Body Form Handlers
//     const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
//         const { name, value } = e.target;
//         setBodyForm(prev => ({
//             ...prev,
//             [name]: (name === 'name' || name === 'color') ? value : parseFloat(value) || 0
//         }));
//     };
//
//     const startCreateBody = () => {
//         setBodyForm({
//             name: "New Body",
//             color: "#3b82f6",
//             mass: 1,
//             width: 40,
//             height: 40,
//             posX: 0,
//             posY: 0,
//             velX: 10,
//             velY: 5,
//         });
//         setRightSidebarView('create');
//     };
//
//     const startEditBody = (body: Body) => {
//         setEditingId(body.name);
//         setBodyForm({
//             name: body.name,
//             color: body.color || "#3b82f6",
//             mass: body.properties.mass,
//             width: body.shape.width,
//             height: body.shape.height,
//             posX: body.linear.displacement.x,
//             posY: body.linear.displacement.y,
//             velX: body.linear.velocity.x,
//             velY: body.linear.velocity.y,
//         });
//         setRightSidebarView('edit');
//     };
//
//     const saveBody = () => {
//         const props = calculateRectProperties(bodyForm.mass, bodyForm.height, bodyForm.width);
//         const shape: Shape = { type: 'Rectangle', width: bodyForm.width, height: bodyForm.height };
//         const linear = createBodyState<Vector>(
//             { x: bodyForm.posX, y: bodyForm.posY },
//             { x: bodyForm.velX, y: bodyForm.velY },
//         );
//         const angular = createBodyState<number>(0, 0);
//
//         if (rightSidebarView === 'create') {
//             const newBody: Body = {
//                 // id: Date.now(),
//                 name: bodyForm.name,
//                 color: bodyForm.color,
//                 shape,
//                 properties: props,
//                 linear,
//                 angular,
//             };
//             setBodies([...bodies, newBody]);
//         } else if (rightSidebarView === 'edit' && editingId !== null) {
//             setBodies(prev => prev.map(b => b.name === editingId ? {
//                 ...b,
//                 name: bodyForm.name,
//                 color: bodyForm.color,
//                 shape,
//                 properties: props,
//                 linear: { ...b.linear, displacement: linear.displacement, velocity: linear.velocity },
//             } : b));
//         }
//         setRightSidebarView('list');
//     };
//
//     const deleteBody = () => {
//         if (editingId) {
//             setBodies(prev => prev.filter(b => b.name !== editingId));
//             setRightSidebarView('list');
//         }
//     }
//
//     // Canvas Interaction Handlers
//     const handleWheel = (e: React.WheelEvent) => {
//         // Prevent default isn't strictly necessary on synthetic events unless we want to stop propagation,
//         // but in canvas it's often good practice if we were blocking page scroll.
//         // e.preventDefault(); // React synthetic events might not need this for internal logic, but for scrolling.
//
//         const scaleSensitivity = 0.001;
//         const delta = -e.deltaY * scaleSensitivity;
//         const newScale = Math.min(Math.max(0.1, viewTransform.current.scale + delta), 5);
//         viewTransform.current.scale = newScale;
//     };
//
//     const handleMouseDown = (e: React.MouseEvent) => {
//         isDragging.current = true;
//         lastMousePos.current = { x: e.clientX, y: e.clientY };
//     };
//
//     const handleMouseMove = (e: React.MouseEvent) => {
//         if (!isDragging.current) return;
//         const dx = e.clientX - lastMousePos.current.x;
//         const dy = e.clientY - lastMousePos.current.y;
//         viewTransform.current.x += dx;
//         viewTransform.current.y += dy;
//         lastMousePos.current = { x: e.clientX, y: e.clientY };
//     };
//
//     const handleMouseUp = () => {
//         isDragging.current = false;
//     };
//
//     // --- Canvas Rendering Loop ---
//     useEffect(() => {
//         const canvas = canvasRef.current;
//         if (!canvas) return;
//         const ctx = canvas.getContext('2d');
//         if (!ctx) return; // TS Check
//
//         let animationFrameId: number;
//
//         const resizeCanvas = () => {
//             if (containerRef.current && canvas) {
//                 canvas.width = containerRef.current.clientWidth;
//                 canvas.height = containerRef.current.clientHeight;
//             }
//         };
//
//         window.addEventListener('resize', resizeCanvas);
//         resizeCanvas();
//
//         const render = () => {
//             const { x: transX, y: transY, scale } = viewTransform.current;
//
//             // Clear
//             ctx.fillStyle = '#f9fafb';
//             ctx.fillRect(0, 0, canvas.width, canvas.height);
//
//             ctx.save();
//             // Apply Pan and Zoom
//             ctx.translate(transX, transY);
//             ctx.scale(scale, scale);
//
//             // Draw Grid
//             ctx.strokeStyle = '#e5e7eb';
//             ctx.lineWidth = 1 / scale;
//             const gridSize = 50;
//             const bigRange = 4000;
//             ctx.beginPath();
//             for (let x = -bigRange; x <= bigRange; x += gridSize) {
//                 ctx.moveTo(x, -bigRange);
//                 ctx.lineTo(x, bigRange);
//             }
//             for (let y = -bigRange; y <= bigRange; y += gridSize) {
//                 ctx.moveTo(-bigRange, y);
//                 ctx.lineTo(bigRange, y);
//             }
//             ctx.stroke();
//
//             // Origin Marker
//             ctx.strokeStyle = '#9ca3af';
//             ctx.lineWidth = 2 / scale;
//             ctx.beginPath();
//             ctx.moveTo(-20, 0); ctx.lineTo(20, 0);
//             ctx.moveTo(0, -20); ctx.lineTo(0, 20);
//             ctx.stroke();
//
//             // Draw Bodies
//             bodies.forEach(body => {
//                 if (body.shape.type === 'Rectangle') {
//                     const { x, y } = body.linear.displacement;
//                     const rotation = body.angular.displacement;
//                     const { width, height } = body.shape;
//                     const color = body.color || '#3b82f6';
//
//                     ctx.save();
//                     ctx.translate(x, y);
//                     ctx.rotate(rotation);
//
//                     ctx.fillStyle = color;
//                     ctx.shadowColor = 'rgba(0, 0, 0, 0.1)';
//                     ctx.shadowBlur = 10;
//                     ctx.shadowOffsetY = 4;
//                     ctx.fillRect(-width / 2, -height / 2, width, height);
//
//                     // Use a dark overlay for border to ensure visibility regardless of color
//                     ctx.strokeStyle = 'rgba(0,0,0,0.2)';
//                     ctx.lineWidth = 2 / scale;
//                     ctx.strokeRect(-width / 2, -height / 2, width, height);
//
//                     ctx.fillStyle = 'rgba(255,255,255,0.8)';
//                     ctx.beginPath();
//                     ctx.arc(0, 0, 2 / scale, 0, Math.PI * 2);
//                     ctx.fill();
//
//                     ctx.restore();
//
//                     // Label
//                     ctx.save();
//                     ctx.translate(x, y - height/2 - 10);
//                     ctx.scale(1 / scale, 1 / scale);
//                     ctx.fillStyle = '#1f2937';
//                     ctx.font = '12px sans-serif';
//                     ctx.textAlign = 'center';
//                     ctx.fillText(body.name, 0, 0);
//                     ctx.restore();
//                 }
//             });
//
//             ctx.restore();
//             animationFrameId = requestAnimationFrame(render);
//         };
//
//         render();
//         return () => {
//             window.removeEventListener('resize', resizeCanvas);
//             cancelAnimationFrame(animationFrameId);
//         };
//     }, [bodies]);
//
//     // Reusable Body Form Component
//     const renderBodyForm = (mode: 'create' | 'edit') => (
//         <div className="flex flex-col h-full">
//             <div className="p-4 border-b border-gray-100 flex items-center justify-between bg-gray-50/50">
//                 <div className="flex items-center gap-3">
//                     <button onClick={() => setRightSidebarView('list')} className="p-1 hover:bg-gray-200 rounded text-gray-500">
//                         <ArrowLeft size={16} />
//                     </button>
//                     <h2 className="text-sm font-semibold text-gray-800">{mode === 'create' ? 'Create Body' : 'Edit Body'}</h2>
//                 </div>
//                 {mode === 'edit' && (
//                     <button onClick={deleteBody} className="text-red-400 hover:text-red-600 p-1 hover:bg-red-50 rounded">
//                         <Trash2 size={16} />
//                     </button>
//                 )}
//             </div>
//
//             <div className="p-5 overflow-y-auto flex-1">
//                 <div className="space-y-4">
//                     {/* Identity */}
//                     <div className="grid grid-cols-4 gap-3">
//                         <div className="col-span-3">
//                             <label className="block text-xs font-medium text-gray-500 mb-1 uppercase">Name</label>
//                             <input
//                                 type="text"
//                                 name="name"
//                                 value={bodyForm.name}
//                                 onChange={handleInputChange}
//                                 className="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded focus:outline-none focus:border-blue-500 text-sm"
//                             />
//                         </div>
//                         <div className="col-span-1">
//                             <label className="block text-xs font-medium text-gray-500 mb-1 uppercase">Color</label>
//                             <div className="flex items-center h-[38px]">
//                                 <input
//                                     type="color"
//                                     name="color"
//                                     value={bodyForm.color}
//                                     onChange={handleInputChange}
//                                     className="w-full h-full p-0 border-0 rounded cursor-pointer"
//                                 />
//                             </div>
//                         </div>
//                     </div>
//
//                     {/* Physical Properties */}
//                     <div className="grid grid-cols-2 gap-3">
//                         <div>
//                             <label className="block text-xs font-medium text-gray-500 mb-1 uppercase">Mass (kg)</label>
//                             <input
//                                 type="number"
//                                 name="mass"
//                                 value={bodyForm.mass}
//                                 onChange={handleInputChange}
//                                 className="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded focus:outline-none focus:border-blue-500 text-sm font-mono"
//                             />
//                         </div>
//                         <div>
//                             <label className="block text-xs font-medium text-gray-400 mb-1 uppercase">MOI</label>
//                             <div className="w-full px-3 py-2 bg-gray-100 border border-transparent rounded text-sm font-mono text-gray-500">
//                                 {(bodyForm.mass / 12 * (Math.pow(bodyForm.width, 2) + Math.pow(bodyForm.height, 2))).toFixed(1)}
//                             </div>
//                         </div>
//                     </div>
//
//                     {/* Shape */}
//                     <div>
//                         <label className="block text-xs font-medium text-gray-500 mb-2 uppercase border-b border-gray-100 pb-1">Shape: Rectangle</label>
//                         <div className="grid grid-cols-2 gap-3">
//                             <div>
//                                 <label className="text-xs text-gray-400">Width</label>
//                                 <input
//                                     type="number"
//                                     name="width"
//                                     value={bodyForm.width}
//                                     onChange={handleInputChange}
//                                     className="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded focus:outline-none focus:border-blue-500 text-sm font-mono"
//                                 />
//                             </div>
//                             <div>
//                                 <label className="text-xs text-gray-400">Height</label>
//                                 <input
//                                     type="number"
//                                     name="height"
//                                     value={bodyForm.height}
//                                     onChange={handleInputChange}
//                                     className="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded focus:outline-none focus:border-blue-500 text-sm font-mono"
//                                 />
//                             </div>
//                         </div>
//                     </div>
//
//                     {/* Initial State */}
//                     <div>
//                         <label className="block text-xs font-medium text-gray-500 mb-2 uppercase border-b border-gray-100 pb-1">State Vectors</label>
//                         <div className="grid grid-cols-2 gap-3 mb-3">
//                             <div>
//                                 <label className="text-xs text-gray-400">Pos X</label>
//                                 <input
//                                     type="number"
//                                     name="posX"
//                                     value={bodyForm.posX}
//                                     onChange={handleInputChange}
//                                     className="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded focus:outline-none focus:border-blue-500 text-sm font-mono"
//                                 />
//                             </div>
//                             <div>
//                                 <label className="text-xs text-gray-400">Pos Y</label>
//                                 <input
//                                     type="number"
//                                     name="posY"
//                                     value={bodyForm.posY}
//                                     onChange={handleInputChange}
//                                     className="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded focus:outline-none focus:border-blue-500 text-sm font-mono"
//                                 />
//                             </div>
//                         </div>
//                         <div className="grid grid-cols-2 gap-3">
//                             <div>
//                                 <label className="text-xs text-gray-400">Vel X</label>
//                                 <input
//                                     type="number"
//                                     name="velX"
//                                     value={bodyForm.velX}
//                                     onChange={handleInputChange}
//                                     className="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded focus:outline-none focus:border-blue-500 text-sm font-mono"
//                                 />
//                             </div>
//                             <div>
//                                 <label className="text-xs text-gray-400">Vel Y</label>
//                                 <input
//                                     type="number"
//                                     name="velY"
//                                     value={bodyForm.velY}
//                                     onChange={handleInputChange}
//                                     className="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded focus:outline-none focus:border-blue-500 text-sm font-mono"
//                                 />
//                             </div>
//                         </div>
//                     </div>
//                 </div>
//             </div>
//
//             <div className="p-4 border-t border-gray-100 bg-gray-50">
//                 <button
//                     onClick={saveBody}
//                     className="w-full py-2 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded shadow-sm flex items-center justify-center gap-2 transition-colors"
//                 >
//                     {mode === 'create' ? <Plus size={16} /> : <Save size={16} />}
//                     {mode === 'create' ? 'Create' : 'Save Changes'}
//                 </button>
//             </div>
//         </div>
//     );
//
//     return (
//         <div className="flex flex-col h-screen w-full bg-white text-gray-800 font-sans overflow-hidden">
//
//             {/* --- Minimal Header --- */}
//             <header className="h-12 border-b border-gray-200 flex items-center justify-between px-4 bg-white z-10 shrink-0">
//                 <div className="flex items-center gap-2">
//                     <div className="p-1 bg-gray-100 rounded cursor-pointer hover:bg-gray-200" onClick={toggleLeftSidebar}>
//                         <Box size={16} className="text-gray-600" />
//                     </div>
//                     <h1 className="text-sm font-semibold tracking-wide text-gray-700 uppercase">Kinematics <span className="text-blue-500">Simulation</span></h1>
//                 </div>
//
//                 <button
//                     onClick={toggleRightSidebar}
//                     className={`px-3 py-1.5 rounded-md text-sm font-medium transition-colors flex items-center gap-2
//             ${rightOpen ? 'bg-blue-100 text-blue-700' : 'bg-gray-100 text-gray-700 hover:bg-gray-200'}
//           `}
//                 >
//                     {rightOpen ? <X size={16} /> : null}
//                     <span>Bodies</span>
//                 </button>
//             </header>
//
//             {/* --- Main Workspace --- */}
//             <div className="flex-1 flex overflow-hidden relative">
//
//                 {/* --- Left Sidebar: Kinematics Equations --- */}
//                 <div
//                     className={`
//             bg-gray-50 border-r border-gray-200 flex flex-col transition-all duration-300 ease-in-out
//             ${leftOpen ? 'w-80 opacity-100' : 'w-0 opacity-0 overflow-hidden'}
//           `}
//                 >
//                     <div className="p-2 flex flex-col h-full">
//                         <h2 className="text-xs font-bold text-gray-400 uppercase mb-2 mt-2 px-2 tracking-wider">Kinematics</h2>
//
//                         {/* Scrollable list of equations */}
//                         <div className="overflow-y-auto space-y-0.5 flex-col flex mb-2">
//                             {equations.map((eq, index) => (
//                                 <div key={eq.id} className="w-full px-1">
//                                     <input
//                                         type="text"
//                                         value={eq.text}
//                                         onChange={(e) => handleEquationChange(eq.id, e.target.value)}
//                                         onBlur={() => handleEquationBlur(eq.id)}
//                                         placeholder={index === equations.length - 1 ? "Add new equation..." : ""}
//                                         className={`
//                         w-full border rounded px-3 py-3 text-base font-mono text-gray-700 focus:outline-none focus:ring-1 focus:ring-blue-100 transition-colors
//                         ${index === equations.length - 1 && eq.text === "" ? "border-dashed border-gray-300 bg-gray-50/50 text-gray-400" : "bg-white border-gray-200 focus:border-blue-400"}
//                     `}
//                                     />
//                                 </div>
//                             ))}
//                         </div>
//
//                         <div className="mt-auto p-2 text-[10px] text-gray-400 text-center">
//                             Simple text editor for physics formulas
//                         </div>
//                     </div>
//                 </div>
//
//                 {/* --- Center: Canvas --- */}
//                 <div
//                     ref={containerRef}
//                     className="flex-1 relative bg-gray-50 overflow-hidden cursor-move"
//                     onMouseDown={handleMouseDown}
//                     onMouseMove={handleMouseMove}
//                     onMouseUp={handleMouseUp}
//                     onMouseLeave={handleMouseUp}
//                     onWheel={handleWheel}
//                 >
//                     <canvas ref={canvasRef} className="block touch-none" />
//
//                     <div className="absolute top-4 left-4 pointer-events-none">
//                         <div className="bg-white/90 backdrop-blur px-2 py-1 rounded border border-gray-200 text-[10px] text-gray-400 font-mono flex gap-3">
//                             <span className="flex items-center gap-1"><Move size={10}/> Pan</span>
//                             <span className="flex items-center gap-1"><ZoomIn size={10}/> Scroll to Zoom</span>
//                         </div>
//                     </div>
//                 </div>
//
//                 {/* --- Right Sidebar: Bodies Manager --- */}
//                 <div
//                     className={`
//             bg-white border-l border-gray-200 flex flex-col transition-all duration-300 ease-in-out shadow-xl z-20
//             ${rightOpen ? 'w-80 translate-x-0' : 'w-0 translate-x-full overflow-hidden absolute right-0 h-full'}
//           `}
//                 >
//                     {/* List View */}
//                     {rightSidebarView === 'list' && (
//                         <div className="flex flex-col h-full">
//                             <div className="p-4 border-b border-gray-100 bg-gray-50/50">
//                                 <h2 className="text-sm font-semibold text-gray-800">Scene Objects</h2>
//                                 <p className="text-xs text-gray-500">{bodies.length} active bodies</p>
//                             </div>
//
//                             <div className="flex-1 overflow-y-auto p-2">
//                                 {bodies.length === 0 ? (
//                                     <div className="text-center py-10 text-gray-400 text-sm italic">No bodies in scene</div>
//                                 ) : (
//                                     <div className="space-y-2">
//                                         {bodies.map(body => (
//                                             <div
//                                                 key={body.name}
//                                                 onClick={() => startEditBody(body)}
//                                                 className="p-3 border border-gray-100 rounded bg-white hover:border-blue-400 hover:shadow-md cursor-pointer transition-all flex justify-between items-center group"
//                                             >
//                                                 <div>
//                                                     <div className="font-medium text-sm text-gray-700">{body.name}</div>
//                                                     <div className="text-[10px] text-gray-400 font-mono mt-0.5">
//                                                         Mass: {body.properties.mass}kg • {body.shape.type}
//                                                     </div>
//                                                 </div>
//                                                 <div
//                                                     className="w-4 h-4 rounded-full border border-gray-200 shadow-sm group-hover:scale-125 transition-transform"
//                                                     style={{ backgroundColor: body.color || '#3b82f6' }}
//                                                 ></div>
//                                             </div>
//                                         ))}
//                                     </div>
//                                 )}
//                             </div>
//
//                             <div className="p-4 border-t border-gray-100 bg-gray-50">
//                                 <button
//                                     onClick={startCreateBody}
//                                     className="w-full py-2 bg-blue-600 hover:bg-blue-700 text-white rounded text-sm font-medium flex items-center justify-center gap-2 shadow-sm transition-colors"
//                                 >
//                                     <Plus size={16} />
//                                     Add New Body
//                                 </button>
//                             </div>
//                         </div>
//                     )}
//
//                     {/* Create & Edit Views */}
//                     {(rightSidebarView === 'create' || rightSidebarView === 'edit') && renderBodyForm(rightSidebarView)}
//
//                 </div>
//
//             </div>
//         </div>
//     );
// }