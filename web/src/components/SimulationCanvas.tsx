import React, {useEffect, useRef} from 'react';
import {Move, ZoomIn} from 'lucide-react';
import type {Body} from '../types';
import {useCanvasTransform} from '../hooks/useCanvasTransform';
import {Body2D, Engine2D} from "interop";

const FPS = 1 / 60.

interface SimulationCanvasProps {
    bodies: Body[];
    running: boolean;
    equations: string[]
}

export const SimulationCanvas: React.FC<SimulationCanvasProps> = ({bodies, running, equations}) => {
    const canvasRef = useRef<HTMLCanvasElement | null>(null);
    const containerRef = useRef<HTMLDivElement | null>(null);
    const {viewTransform, handlers} = useCanvasTransform(1);
    const runningRef = useRef<boolean>(false);
    const engine = useRef<Engine2D | null>(null);

    useEffect(() => {
        runningRef.current = running;

        console.log(equations)
        console.log(bodies)

        if (running) {
            try {
                engine.current = Engine2D.new(
                    bodies.map((body) => {
                        return Body2D.new(
                            body.name,
                            body.properties.mass,
                            body.shape.width,
                            body.shape.height,
                            body.linear.displacement.x,
                            body.linear.displacement.y,
                            body.linear.velocity.x,
                            body.linear.velocity.y,
                        )
                    }),
                    equations?.filter((e) => e.length != 0) ?? [],
                    FPS
                )
            } catch (e) {
                console.error(e);
            }
        } else {
            engine.current = null
        }
    }, [running]);

    useEffect(() => {
        const canvas = canvasRef.current;
        if (!canvas) return;
        const ctx = canvas.getContext('2d');
        if (!ctx) return;

        let animationFrameId: number;

        const resizeCanvas = () => {
            if (containerRef.current && canvas) {
                canvas.width = containerRef.current.clientWidth;
                canvas.height = containerRef.current.clientHeight;
            }
        };

        window.addEventListener('resize', resizeCanvas);
        resizeCanvas();

        const render = () => {
            const {x: transX, y: transY, scale} = viewTransform.current;

            // Clear
            ctx.fillStyle = '#f9fafb';
            ctx.fillRect(0, 0, canvas.width, canvas.height);

            ctx.save();
            ctx.translate(transX, transY);
            ctx.scale(scale, scale);

            // Draw Grid
            drawGrid(ctx, scale);

            // Draw Origin
            drawOrigin(ctx, scale);

            engine.current?.tick()

            // Draw Bodies
            if (runningRef.current) {
                // console.log(engine.current?.get_state()[0].y())
                // engine.current?.get_state().forEach((body) => {
                //     console.log("Hey")
                // })
                engine.current?.get_state().forEach((body) => {
                    drawBody(ctx, body.x(), body.y(), body.theta(), body.name(), scale, bodies)
                })
            } else {
                bodies.forEach((body) => {
                    drawBody(ctx, body.linear.displacement.x, body.linear.displacement.y, body.angular.displacement, body.name, scale, bodies)
                })
            }

            ctx.restore();
            animationFrameId = requestAnimationFrame(render);
        };

        render();
        return () => {
            window.removeEventListener('resize', resizeCanvas);
            cancelAnimationFrame(animationFrameId);
        };
    }, [bodies]); // Re-bind effect if bodies change

    return (
        <div
            ref={containerRef}
            className="flex-1 relative bg-gray-50 overflow-hidden cursor-move"
            {...handlers}
        >
            <canvas ref={canvasRef} className="block touch-none"/>
            <div className="absolute top-4 left-4 pointer-events-none">
                <div
                    className="bg-white/90 backdrop-blur px-2 py-1 rounded border border-gray-200 text-[10px] text-gray-400 font-mono flex gap-3">
                    <span className="flex items-center gap-1"><Move size={10}/> Pan</span>
                    <span className="flex items-center gap-1"><ZoomIn size={10}/> Scroll to Zoom</span>
                </div>
            </div>
            <div className="absolute top-4 right-4 pointer-events-none">
            </div>
        </div>
    );
};

// --- Drawing Helpers (Local to this file) ---

function drawGrid(ctx: CanvasRenderingContext2D, scale: number) {
    ctx.strokeStyle = '#e5e7eb';
    ctx.lineWidth = 1 / scale;
    const gridSize = 50;
    const bigRange = 4000;
    ctx.beginPath();
    for (let x = -bigRange; x <= bigRange; x += gridSize) {
        ctx.moveTo(x, -bigRange);
        ctx.lineTo(x, bigRange);
    }
    for (let y = -bigRange; y <= bigRange; y += gridSize) {
        ctx.moveTo(-bigRange, y);
        ctx.lineTo(bigRange, y);
    }
    ctx.stroke();
}

function drawOrigin(ctx: CanvasRenderingContext2D, scale: number) {
    ctx.strokeStyle = '#9ca3af';
    ctx.lineWidth = 2 / scale;
    ctx.beginPath();
    ctx.moveTo(-20, 0);
    ctx.lineTo(20, 0);
    ctx.moveTo(0, -20);
    ctx.lineTo(0, 20);
    ctx.stroke();
}

// Note that the canvas's up and down direction are different from what we want. (have to invert y)
function drawBody(
    ctx: CanvasRenderingContext2D,
    x: number, y: number, rotation: number,
    name: string,
    scale: number,
    all: Body[]
) {
    let body = all.find((b) => b.name == name)!;
    const color = body.color || '#3b82f6';
    const [width, height] = [body.shape.width, body.shape.height];

    ctx.save();
    ctx.translate(x, -y);
    ctx.rotate(rotation);

    ctx.fillStyle = color;
    ctx.shadowColor = 'rgba(0, 0, 0, 0.1)';
    ctx.shadowBlur = 10;
    ctx.shadowOffsetY = 4;
    ctx.fillRect(-width / 2, -height / 2, width, height);

    ctx.strokeStyle = 'rgba(0,0,0,0.2)';
    ctx.lineWidth = 2 / scale;
    ctx.strokeRect(-width / 2, -height / 2, width, height);

    ctx.fillStyle = 'rgba(255,255,255,0.8)';
    ctx.beginPath();
    ctx.arc(0, 0, 2 / scale, 0, Math.PI * 2);
    ctx.fill();

    ctx.restore();

    // Label
    ctx.save();
    ctx.translate(x, -y - height / 2 - 10);
    ctx.scale(1 / scale, 1 / scale);
    ctx.fillStyle = '#1f2937';
    ctx.font = '12px sans-serif';
    ctx.textAlign = 'center';
    ctx.fillText(name, 0, 0);
    ctx.restore();
}