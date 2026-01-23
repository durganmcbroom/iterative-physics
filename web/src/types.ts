export interface Vector {
    x: number;
    y: number;
}

export interface BodyState<T> {
    displacement: T;
    velocity: T;
}

export interface BodyProperties {
    mass: number;
    moi: number;
}

export interface Shape {
    type: 'Rectangle';
    width: number;
    height: number;
}

export interface Body {
    name: string;
    color: string;
    shape: Shape;
    properties: BodyProperties;
    linear: BodyState<Vector>;
    angular: BodyState<number>;
}

export interface Equation {
    id: number;
    text: string;
}

export interface BodyFormState {
    name: string;
    color: string;
    mass: number;
    width: number;
    height: number;
    posX: number;
    posY: number;
    velX: number;
    velY: number;
    rot: number;
}

export interface ViewTransform {
    x: number;
    y: number;
    scale: number;
}