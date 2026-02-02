import { useRef } from 'react';
import type { Vector, ViewTransform } from '../types';

export function useCanvasTransform(initialScale: number = 1) {
    const viewTransform = useRef<ViewTransform>({ x: 300, y: 300, scale: initialScale });

    // State for mouse/touch interaction
    const isDragging = useRef<boolean>(false);
    const lastMousePos = useRef<Vector>({ x: 0, y: 0 });
    const lastPinchDistance = useRef<number | null>(null);

    // --- Helper: Get relative coordinates ---
    // This ensures zooming works even if the canvas isn't at (0,0) on the page
    const getRelativePos = (e: { clientX: number, clientY: number }, target: HTMLElement) => {
        const rect = target.getBoundingClientRect();
        return {
            x: e.clientX - rect.left,
            y: e.clientY - rect.top
        };
    };

    // --- Helper: Distance between two fingers ---
    const getDistance = (touch1: React.Touch, touch2: React.Touch) => {
        const dx = touch1.clientX - touch2.clientX;
        const dy = touch1.clientY - touch2.clientY;
        return Math.sqrt(dx * dx + dy * dy);
    };

    // --- Helper: Midpoint between two fingers ---
    const getMidpoint = (touch1: React.Touch, touch2: React.Touch, target: HTMLElement) => {
        const rect = target.getBoundingClientRect();
        return {
            x: ((touch1.clientX + touch2.clientX) / 2) - rect.left,
            y: ((touch1.clientY + touch2.clientY) / 2) - rect.top
        };
    };

    // --- Core Logic: Zoom towards a specific point ---
    const zoomToPoint = (newScale: number, center: Vector) => {
        const currentScale = viewTransform.current.scale;

        // Clamp scale limits
        const safeScale = Math.min(Math.max(0.1, newScale), 5);

        if (safeScale === currentScale) return;

        // Calculate the ratio of the change
        const scaleRatio = safeScale / currentScale;

        // The math to keep the 'center' fixed:
        // NewX = CenterX - (CenterX - OldX) * ScaleRatio
        viewTransform.current.x = center.x - (center.x - viewTransform.current.x) * scaleRatio;
        viewTransform.current.y = center.y - (center.y - viewTransform.current.y) * scaleRatio;
        viewTransform.current.scale = safeScale;
    };

    // --- Mouse Handlers ---
    const handleWheel = (e: React.WheelEvent) => {
        // Prevent default browser zooming if needed, usually handled by CSS/Layout
        const center = getRelativePos(e, e.currentTarget as HTMLElement);
        const scaleSensitivity = 0.001;
        const delta = -e.deltaY * scaleSensitivity;

        zoomToPoint(viewTransform.current.scale + delta, center);
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

    // --- Touch Handlers (Mobile) ---
    const handleTouchStart = (e: React.TouchEvent) => {
        if (e.touches.length === 1) {
            isDragging.current = true;
            lastMousePos.current = { x: e.touches[0].clientX, y: e.touches[0].clientY };
        } else if (e.touches.length === 2) {
            isDragging.current = false;
            lastPinchDistance.current = getDistance(e.touches[0], e.touches[1]);
        }
    };

    const handleTouchMove = (e: React.TouchEvent) => {
        if (e.touches.length === 1 && isDragging.current) {
            // Pan
            const dx = e.touches[0].clientX - lastMousePos.current.x;
            const dy = e.touches[0].clientY - lastMousePos.current.y;
            viewTransform.current.x += dx;
            viewTransform.current.y += dy;
            lastMousePos.current = { x: e.touches[0].clientX, y: e.touches[0].clientY };
        } else if (e.touches.length === 2) {
            // Pinch Zoom
            const dist = getDistance(e.touches[0], e.touches[1]);

            if (lastPinchDistance.current !== null) {
                const deltaDistance = dist - lastPinchDistance.current;
                const zoomSensitivity = 0.005;
                const targetScale = viewTransform.current.scale + deltaDistance * zoomSensitivity;

                // Calculate the midpoint of the fingers to zoom towards THAT point
                const center = getMidpoint(e.touches[0], e.touches[1], e.currentTarget as HTMLElement);

                zoomToPoint(targetScale, center);
            }

            lastPinchDistance.current = dist;
        }
    };

    const handleTouchEnd = () => {
        isDragging.current = false;
        lastPinchDistance.current = null;
    };

    return {
        viewTransform,
        handlers: {
            onWheel: handleWheel,
            onMouseDown: handleMouseDown,
            onMouseMove: handleMouseMove,
            onMouseUp: handleMouseUp,
            onMouseLeave: handleMouseUp,
            onTouchStart: handleTouchStart,
            onTouchMove: handleTouchMove,
            onTouchEnd: handleTouchEnd,
            onTouchCancel: handleTouchEnd,
        }
    };
}