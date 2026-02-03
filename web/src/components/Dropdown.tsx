import { useState, useRef, useEffect } from 'react';
import { createPortal } from 'react-dom';
import { ChevronDown, Check } from 'lucide-react';

interface DropdownProps {
    options: string[];
    value: string | null;
    onChange: (value: string) => void;
    placeholder?: string;
}

export function Dropdown({ options, value, onChange, placeholder = "Select..." }: DropdownProps) {
    const [isOpen, setIsOpen] = useState(false);
    const [coords, setCoords] = useState({ top: 0, left: 0, width: 0 });

    // We need refs for both the trigger button and the portal menu
    // to correctly handle the "click outside" logic across the DOM split.
    const triggerRef = useRef<HTMLButtonElement>(null);
    const menuRef = useRef<HTMLDivElement>(null);

    // Update position when opening
    const toggleOpen = () => {
        if (!isOpen && triggerRef.current) {
            const rect = triggerRef.current.getBoundingClientRect();
            setCoords({
                top: rect.bottom + 4, // 4px gap
                left: rect.left,
                width: rect.width
            });
        }
        setIsOpen(!isOpen);
    };

    const handleSelect = (option: string) => {
        onChange(option);
        setIsOpen(false);
    };

    // Close dropdown when clicking outside
    useEffect(() => {
        function handleClickOutside(event: MouseEvent) {
            const target = event.target as Node;
            // Check if click is outside BOTH the trigger button and the portal menu
            if (
                isOpen &&
                triggerRef.current && !triggerRef.current.contains(target) &&
                menuRef.current && !menuRef.current.contains(target)
            ) {
                setIsOpen(false);
            }
        }

        // Handle window resize to prevent floating menu if window moves
        function handleResize() {
            if (isOpen) setIsOpen(false);
        }

        document.addEventListener("mousedown", handleClickOutside);
        window.addEventListener("resize", handleResize);

        return () => {
            document.removeEventListener("mousedown", handleClickOutside);
            window.removeEventListener("resize", handleResize);
        };
    }, [isOpen]);

    return (
        <>
            <button
                ref={triggerRef}
                onClick={toggleOpen}
                className={`
                    flex items-center justify-between gap-2 px-3 py-1.5 
                    bg-gray-50 border border-gray-200 rounded-md 
                    text-sm font-medium text-gray-700 
                    hover:bg-gray-100 hover:border-gray-300 transition-all
                    min-w-[120px] w-full
                    ${isOpen ? 'ring-2 ring-blue-100 border-blue-400' : ''}
                `}
            >
                <span className="truncate">{value || placeholder}</span>
                <ChevronDown size={14} className={`text-gray-500 transition-transform duration-200 shrink-0 ${isOpen ? 'rotate-180' : ''}`} />
            </button>

            {isOpen && createPortal(
                <div
                    ref={menuRef}
                    style={{
                        top: coords.top,
                        left: coords.left,
                        width: coords.width
                    }}
                    className="fixed z-[9999] bg-white border border-gray-200 rounded-md shadow-lg overflow-hidden animate-in fade-in zoom-in-95 duration-100"
                >
                    <div className="py-1 max-h-[300px] overflow-y-auto">
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
                </div>,
                document.body
            )}
        </>
    );
}