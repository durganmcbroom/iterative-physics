import { useRef } from 'react';
import type {Vector, ViewTransform} from '../types';

export function useCanvasTransform(initialScale: number = 1) {
    const viewTransform = useRef<ViewTransform>({ x: 300, y: 300, scale: initialScale });
    const isDragging = useRef<boolean>(false);
    const lastMousePos = useRef<Vector>({ x: 0, y: 0 });

    const handleWheel = (e: React.WheelEvent) => {
        const scaleSensitivity = 0.001;
        const delta = -e.deltaY * scaleSensitivity;
        const newScale = Math.min(Math.max(0.1, viewTransform.current.scale + delta), 5);
        viewTransform.current.scale = newScale;
    };

    const handleMouseDown = (e: React.MouseEvent) => {
        isDragging.current = true;
        lastMousePos.current = { x: e.clientX, y: e.clientY };
    };

    const handleMouseMove = (e: React.MouseEvent) => {
        if (!isDragging.current) return;
        const dx = e.clientX - lastMousePos.current.x;
        const dy = e.clientY - lastMousePos.current.y;
        viewTransform.current.x += dx;
        viewTransform.current.y += dy;
        lastMousePos.current = { x: e.clientX, y: e.clientY };
    };

    const handleMouseUp = () => {
        isDragging.current = false;
    };

    return {
        viewTransform,
        handlers: {
            onWheel: handleWheel,
            onMouseDown: handleMouseDown,
            onMouseMove: handleMouseMove,
            onMouseUp: handleMouseUp,
            onMouseLeave: handleMouseUp,
        }
    };
}