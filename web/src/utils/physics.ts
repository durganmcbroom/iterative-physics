import type {BodyState, BodyProperties} from '../types';

export const createBodyState = <T>(displacement: T, velocity: T): BodyState<T> => ({
    displacement,
    velocity,
});

export const calculateRectProperties = (mass: number, height: number, width: number): BodyProperties => {
    // Moment of Inertia for a rectangle
    const moi = (mass / 12.0) * (Math.pow(height, 2) + Math.pow(width, 2));
    return { mass, moi };
};