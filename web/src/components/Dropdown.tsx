import { useState, useRef, useEffect } from 'react';
import { ChevronDown, Check } from 'lucide-react';

interface DropdownProps {
    options: string[];
    value: string | null;
    onChange: (value: string) => void;
    placeholder?: string;
}

export function Dropdown({ options, value, onChange, placeholder = "Select..." }: DropdownProps) {
    const [isOpen, setIsOpen] = useState(false);
    const dropdownRef = useRef<HTMLDivElement>(null);

    // Close dropdown when clicking outside
    useEffect(() => {
        function handleClickOutside(event: MouseEvent) {
            if (dropdownRef.current && !dropdownRef.current.contains(event.target as Node)) {
                setIsOpen(false);
            }
        }
        document.addEventListener("mousedown", handleClickOutside);
        return () => document.removeEventListener("mousedown", handleClickOutside);
    }, []);

    const handleSelect = (option: string) => {
        onChange(option);
        setIsOpen(false);
    };

    return (
        <div className="relative" ref={dropdownRef}>
            <button
                onClick={() => setIsOpen(!isOpen)}
                className={`
                    flex items-center justify-between gap-2 px-3 py-1.5 
                    bg-gray-50 border border-gray-200 rounded-md 
                    text-sm font-medium text-gray-700 
                    hover:bg-gray-100 hover:border-gray-300 transition-all
                    min-w-[160px]
                    ${isOpen ? 'ring-2 ring-blue-100 border-blue-400' : ''}
                `}
            >
                <span className="truncate">{value || placeholder}</span>
                <ChevronDown size={14} className={`text-gray-500 transition-transform duration-200 ${isOpen ? 'rotate-180' : ''}`} />
            </button>

            {isOpen && (
                <div className="absolute top-full left-0 mt-1 w-full min-w-[200px] bg-white border border-gray-200 rounded-md shadow-lg z-50 overflow-hidden animate-in fade-in zoom-in-95 duration-100">
                    <div className="py-1">
                        {options.map((option) => (
                            <div
                                key={option}
                                onClick={() => handleSelect(option)}
                                className="
                                    px-4 py-2 text-sm text-gray-700 cursor-pointer
                                    hover:bg-blue-50 hover:text-blue-700
                                    flex items-center justify-between
                                "
                            >
                                <span>{option}</span>
                                {value === option && <Check size={14} className="text-blue-600" />}
                            </div>
                        ))}
                    </div>
                </div>
            )}
        </div>
    );
}