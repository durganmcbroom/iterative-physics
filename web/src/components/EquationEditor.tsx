import React from 'react';
import type {Equation} from '../types';

interface EquationEditorProps {
    equations: Equation[];
    setEquations: React.Dispatch<React.SetStateAction<Equation[]>>;
}

export const EquationEditor: React.FC<EquationEditorProps> = ({ equations, setEquations }) => {
    const handleEquationChange = (id: number, newText: string) => {
        setEquations(prev => {
            const updated = prev.map(eq => eq.id === id ? { ...eq, text: newText } : eq);
            // If we typed in the last box, add a new ghost box
            const lastEq = updated[updated.length - 1];
            if (lastEq.text.trim() !== "") {
                return [...updated, { id: Date.now() + Math.random(), text: "" }];
            }
            return updated;
        });
    };

    const handleEquationBlur = (id: number) => {
        setEquations(prev => {
            if (prev.length <= 1) return prev;
            const index = prev.findIndex(eq => eq.id === id);
            const isLast = index === prev.length - 1;
            // If it's empty and NOT the last box, remove it
            if (!isLast && prev[index].text.trim() === "") {
                return prev.filter(eq => eq.id !== id);
            }
            return prev;
        });
    };

    return (
        <div className="p-2 flex flex-col h-full">
            <h2 className="text-xs font-bold text-gray-400 uppercase mb-2 mt-2 px-2 tracking-wider">Kinematics</h2>
            <div className="overflow-y-auto space-y-0.5 flex-col flex mb-2">
                {equations.map((eq, index) => (
                    <div key={eq.id} className="w-full px-1">
                        <input
                            type="text"
                            value={eq.text}
                            onChange={(e) => handleEquationChange(eq.id, e.target.value)}
                            onBlur={() => handleEquationBlur(eq.id)}
                            placeholder={index === equations.length - 1 ? "Add new equation..." : ""}
                            className={`
                                w-full border rounded px-3 py-3 text-base font-mono text-gray-700 focus:outline-none focus:ring-1 focus:ring-blue-100 transition-colors
                                ${index === equations.length - 1 && eq.text === "" ? "border-dashed border-gray-300 bg-gray-50/50 text-gray-400" : "bg-white border-gray-200 focus:border-blue-400"}
                            `}
                        />
                    </div>
                ))}
            </div>
            <div className="mt-auto p-2 text-[10px] text-gray-400 text-center">
                Durgan McBroom. AI was used in the development of this frontend. See <a className={"border-b-1 border-gray-300"} href={"https://github.com"}>Github for more info.</a>
            </div>
        </div>
    );
};