import React, {useState, useEffect} from 'react';
import {Plus, ArrowLeft, Save, Trash2, Pi} from 'lucide-react';
import type {Body, BodyFormState} from '../types';

interface BodyFormProps {
    mode: 'create' | 'edit';
    initialBody?: Body | null;
    onSave: (formState: BodyFormState) => void;
    onCancel: () => void;
    onDelete: () => void;
}

const DEFAULT_FORM: BodyFormState = {
    name: "B", color: "#3b82f6", mass: 1, width: 40, height: 40,
    posX: 0, posY: 0, velX: 0, velY: 0, rot: 0
};

export const BodyForm: React.FC<BodyFormProps> = ({mode, initialBody, onSave, onCancel, onDelete}) => {
    const [form, setForm] = useState<BodyFormState>(DEFAULT_FORM);

    useEffect(() => {
        if (mode === 'edit' && initialBody) {
            setForm({
                name: initialBody.name,
                color: initialBody.color || "#3b82f6",
                mass: initialBody.properties.mass,
                width: initialBody.shape.width,
                height: initialBody.shape.height,
                posX: initialBody.linear.displacement.x,
                posY: initialBody.linear.displacement.y,
                velX: initialBody.linear.velocity.x,
                velY: initialBody.linear.velocity.y,
                rot: initialBody.angular.displacement,
            });
        } else {
            setForm(DEFAULT_FORM);
        }
    }, [mode, initialBody]);

    const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>, transform: (t: any) => any = (t) => parseFloat(t)) => {
        const {name, value} = e.target;

        setForm(prev => ({
            ...prev,
            [name]: transform(value)
        }));

        console.log(form)
    };

    return (
        <div className="flex flex-col h-full">
            <div className="p-4 border-b border-gray-100 flex items-center justify-between bg-gray-50/50">
                <div className="flex items-center gap-3">
                    <button onClick={onCancel} className="p-1 hover:bg-gray-200 rounded text-gray-500">
                        <ArrowLeft size={16}/>
                    </button>
                    <h2 className="text-sm font-semibold text-gray-800">{mode === 'create' ? 'Create Body' : 'Edit Body'}</h2>
                </div>
                {mode === 'edit' && (
                    <button onClick={onDelete} className="text-red-400 hover:text-red-600 p-1 hover:bg-red-50 rounded">
                        <Trash2 size={16}/>
                    </button>
                )}
            </div>

            <div className="p-5 overflow-y-auto flex-1">
                {/* ... (The inputs are identical to original, just referencing `form` state) ... */}
                {/* For brevity, I am abbreviating the repetitive HTML structure here,
                     but in real code you would paste the exact JSX from the original file
                     replacing `bodyForm` with `form`. */}

                <div className="space-y-4">
                    {/* Identity */}
                    <div className="grid grid-cols-4 gap-3">
                        <div className="col-span-3">
                            <label className="block text-xs font-medium text-gray-500 mb-1 uppercase">Name</label>
                            <input
                                type="text"
                                name="name"
                                value={form.name}
                                onChange={(e) => handleInputChange(e, t => t)}
                                className="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded focus:outline-none focus:border-blue-500 text-sm"
                            />
                        </div>
                        <div className="col-span-1">
                            <label className="block text-xs font-medium text-gray-500 mb-1 uppercase">Color</label>
                            <div className="flex items-center h-[38px]">
                                <input
                                    type="color"
                                    name="color"
                                    value={form.color}
                                    onChange={(e) => handleInputChange(e, t => t)}
                                    className="w-full h-full p-0 border-0 rounded cursor-pointer"
                                />
                            </div>
                        </div>
                    </div>

                    {/* Physical Properties */}
                    <div className="grid grid-cols-2 gap-3">
                        <div>
                            <label className="block text-xs font-medium text-gray-500 mb-1 uppercase">Mass (kg)</label>
                            <input
                                type="number"
                                name="mass"
                                value={form.mass}
                                onChange={handleInputChange}
                                className="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded focus:outline-none focus:border-blue-500 text-sm font-mono"
                            />
                        </div>
                        <div>
                            <label className="block text-xs font-medium text-gray-400 mb-1 uppercase">MOI</label>
                            <div
                                className="w-full px-3 py-2 bg-gray-100 border border-transparent rounded text-sm font-mono text-gray-500">
                                {(form.mass / 12 * (Math.pow(form.width, 2) + Math.pow(form.height, 2))).toFixed(1)}
                            </div>
                        </div>
                    </div>

                    {/* Shape */}
                    <div>
                        <label
                            className="block text-xs font-medium text-gray-500 mb-2 uppercase border-b border-gray-100 pb-1">Shape:
                            Rectangle</label>
                        <div className="grid grid-cols-2 gap-3">
                            <div>
                                <label className="text-xs text-gray-400">Width</label>
                                <input
                                    type="number"
                                    name="width"
                                    value={form.width}
                                    onChange={handleInputChange}
                                    className="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded focus:outline-none focus:border-blue-500 text-sm font-mono"
                                />
                            </div>
                            <div>
                                <label className="text-xs text-gray-400">Height</label>
                                <input
                                    type="number"
                                    name="height"
                                    value={form.height}
                                    onChange={handleInputChange}
                                    className="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded focus:outline-none focus:border-blue-500 text-sm font-mono"
                                />
                            </div>
                        </div>
                    </div>

                    {/* Initial State */}
                    <div>
                        <label
                            className="block text-xs font-medium text-gray-500 mb-2 uppercase border-b border-gray-100 pb-1">State
                            Vectors</label>
                        <div className="grid grid-cols-2 gap-3 mb-3">
                            <div>
                                <label className="text-xs text-gray-400">Pos X</label>
                                <input
                                    type="number"
                                    name="posX"
                                    value={form.posX}
                                    onChange={handleInputChange}
                                    className="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded focus:outline-none focus:border-blue-500 text-sm font-mono"
                                />
                            </div>
                            <div>
                                <label className="text-xs text-gray-400">Pos Y</label>
                                <input
                                    type="number"
                                    name="posY"
                                    value={form.posY}
                                    onChange={handleInputChange}
                                    className="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded focus:outline-none focus:border-blue-500 text-sm font-mono"
                                />
                            </div>
                        </div>
                        <div className="grid grid-cols-2 gap-3">
                            <div>
                                <label className="text-xs text-gray-400">Vel X</label>
                                <input
                                    type="number"
                                    name="velX"
                                    value={form.velX}
                                    onChange={handleInputChange}
                                    className="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded focus:outline-none focus:border-blue-500 text-sm font-mono"
                                />
                            </div>
                            <div>
                                <label className="text-xs text-gray-400">Vel Y</label>
                                <input
                                    type="number"
                                    name="velY"
                                    value={form.velY}
                                    onChange={handleInputChange}
                                    className="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded focus:outline-none focus:border-blue-500 text-sm font-mono"
                                />
                            </div>
                        </div>
                        <div className="">
                            <label className="text-xs text-gray-400">Rotation</label>
                            <div className="mt-1">
                                <div className="flex items-center gap-2">
                                    <input
                                        type="number"
                                        name="rot"
                                        value={form.rot / Math.PI}
                                        onChange={(e)=>handleInputChange(e, (t) => t*Math.PI)}
                                        className="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded focus:outline-none focus:border-blue-500 text-sm font-mono"
                                    />
                                    <Pi strokeWidth={3} className="text-gray-600"/>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

            </div>

            <div className="p-4 border-t border-gray-100 bg-gray-50">
                <button
                    onClick={() => onSave(form)}
                    className="w-full py-2 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded shadow-sm flex items-center justify-center gap-2 transition-colors"
                >
                    {mode === 'create' ? <Plus size={16}/> : <Save size={16}/>}
                    {mode === 'create' ? 'Create' : 'Save Changes'}
                </button>
            </div>
        </div>
    );
};