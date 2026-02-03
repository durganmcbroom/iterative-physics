import type {Body} from "./types.ts";

type TemplateState = {
    bodies: Body[],
    equations: string[]
}

export const ObstacleCourse: TemplateState = {
    bodies: [{
        "name": "A",
        "color": "#3b82f6",
        "shape": {"type": "Rectangle", "width": 50, "height": 50},
        "properties": {"mass": 10, "moi": 4166.666666666667},
        "linear": {"displacement": {"x": 0, "y": 300}, "velocity": {"x": 0, "y": 0}},
        "angular": {"displacement": 0, "velocity": 0}
    }, {
        "name": "Bumper 1",
        "color": "#f73b3b",
        "shape": {"type": "Rectangle", "width": 200, "height": 20},
        "properties": {"mass": 10000000000, "moi": 33666666666666.668},
        "linear": {"displacement": {"x": 50, "y": 0}, "velocity": {"x": 0, "y": 0}},
        "angular": {"displacement": 0.47123889803846897, "velocity": 0}
    }, {
        "name": "Bumper 2",
        "color": "#f73b3b",
        "shape": {"type": "Rectangle", "width": 200, "height": 20},
        "properties": {"mass": 10000000000, "moi": 33666666666666.668},
        "linear": {"displacement": {"x": -300, "y": -300}, "velocity": {"x": 0, "y": 0}},
        "angular": {"displacement": 0, "velocity": 0}
    }, {
        "name": "B",
        "color": "#3fac11",
        "shape": {"type": "Rectangle", "width": 40, "height": 40},
        "properties": {"mass": 10, "moi": 2666.666666666667},
        "linear": {"displacement": {"x": -300, "y": 200}, "velocity": {"x": 0, "y": 0}},
        "angular": {"displacement": 0, "velocity": 0}
    }, {
        "name": "Bumper 3",
        "color": "#f73b3b",
        "shape": {"type": "Rectangle", "width": 200, "height": 20},
        "properties": {"mass": 1000000000, "moi": 3366666666666.6665},
        "linear": {"displacement": {"x": -600, "y": -500}, "velocity": {"x": 0, "y": 0}},
        "angular": {"displacement": -0.47123889803846897, "velocity": 0}
    }, {
        "name": "Bumper 4",
        "color": "#f73b3b",
        "shape": {"type": "Rectangle", "width": 800, "height": 20},
        "properties": {"mass": 10000000000000, "moi": 533666666666666700},
        "linear": {"displacement": {"x": -1150, "y": -500}, "velocity": {"x": 0, "y": 0}},
        "angular": {"displacement": -1.413716694115407, "velocity": 0}
    }, {
        "name": "End",
        "color": "#f73b3b",
        "shape": {"type": "Rectangle", "width": 2000, "height": 20},
        "properties": {"mass": 100000000000, "moi": 33336666666666664},
        "linear": {"displacement": {"x": 0, "y": -1000}, "velocity": {"x": 0, "y": 0}},
        "angular": {"displacement": 0, "velocity": 0}
    }, {
        "name": "Bumper 5",
        "color": "#f73b3b",
        "shape": {"type": "Rectangle", "width": 400, "height": 20},
        "properties": {"mass": 1000000000, "moi": 13366666666666.666},
        "linear": {"displacement": {"x": -750, "y": -800}, "velocity": {"x": 0, "y": 0}},
        "angular": {"displacement": 0, "velocity": 0}
    }],
    equations: [
        "g=100",
        "a_A=-g*hatj",
        "a_B=-g*hatj"
    ]
}

export const Blank: TemplateState = {
    bodies: [],
    equations: []
}

export const OneBodyProblem = {
    bodies: [{
        "name": "Planet",
        "color": "#0cc018",
        "shape": {"type": "Rectangle", "width": 80, "height": 80},
        "properties": {"mass": 105, "moi": 112000},
        "linear": {"displacement": {"x": 0, "y": 0}, "velocity": {"x": 0, "y": 0}},
        "angular": {"displacement": 0, "velocity": 0}
    }, {
        "name": "Satellite",
        "color": "#7d3bf7",
        "shape": {"type": "Rectangle", "width": 40, "height": 40},
        "properties": {"mass": 1, "moi": 266.66666666666663},
        "linear": {"displacement": {"x": 300, "y": 300}, "velocity": {"x": 120, "y": -120}},
        "angular": {"displacement": 0, "velocity": 0}
    }],
    equations: [
        "omega_Planet = 4pi*hatk",
        "G=100000",
        "r = sqrt(x_Satellite^2 + y_Satellite^2)",
        "hatr = (x_Satellite*hati + y_Satellite*hatj)/r",
        "a_Satellite = -G/r*hatr"
    ]
}

export const ThreeBodyProblem = {
    bodies: [{
        "name": "Planet",
        "color": "#0cc018",
        "shape": {"type": "Rectangle", "width": 80, "height": 80},
        "properties": {"mass": 29700000000000000.0, "moi": 31680000000000000000.0},
        "linear": {"displacement": {"x": 0, "y": 0}, "velocity": {"x": 110, "y": 110}},
        "angular": {"displacement": 0, "velocity": 0}
    }, {
        "name": "Satellite",
        "color": "#7d3bf7",
        "shape": {"type": "Rectangle", "width": 40, "height": 40},
        "properties": {"mass": 29700000000000000.0, "moi": 7920000000000000000.0},
        "linear": {"displacement": {"x": 300, "y": 300}, "velocity": {"x": 110, "y": -110}},
        "angular": {"displacement": 0, "velocity": 0}
    }, {
        "name": "Satellite_B",
        "color": "#3b82f6",
        "shape": {"type": "Rectangle", "width": 40, "height": 40},
        "properties": {"mass": 29700000000000000.0, "moi": 7920000000000000000.0},
        "linear": {"displacement": {"x": -300, "y": -300}, "velocity": {"x": 220, "y": -220}},
        "angular": {"displacement": 0, "velocity": 0}
    }],
    equations: [
        "omega_Planet = 4pi*hatk",
        "G=0.0000000000017",
        "r(x,y,p_a, p_b) = sqrt((x-p_a)^2 + (y-p_b)^2)",
        "hatr(x,y,p_a, p_b) = ((x-p_a)*hati + (y-p_b)*hatj)/r(x,y,p_a,p_b)",
        "gravity(m_a,m_b,x,y,p_a, p_b)=-(G*m_a*m_b)/r(x,y,p_a,p_b)*hatr(x,y,p_a,p_b)",
        "m_Satellite*a_Satellite = gravity(m_Planet, m_Satellite, x_Satellite, y_Satellite, x_Planet, y_Planet) + gravity(m_Satellite, m_Satellite_B, x_Satellite, y_Satellite, x_Satellite_B, y_Satellite_B)",
        "m_Satellite_B*a_Satellite_B = gravity(m_Planet, m_Satellite_B, x_Satellite_B, y_Satellite_B, x_Planet, y_Planet) + gravity(m_Satellite, m_Satellite_B, x_Satellite_B, y_Satellite_B, x_Satellite, y_Satellite)",
        "m_Planet*a_Planet = -gravity(m_Planet, m_Satellite_B, x_Satellite_B, y_Satellite_B, x_Planet, y_Planet) - gravity(m_Planet, m_Satellite, x_Satellite, y_Satellite, x_Planet, y_Planet)"
    ]
}

export const Pendulum = {
    bodies: [{
        "name": "Pendulum",
        "color": "#3b82f6",
        "shape": {"type": "Rectangle", "width": 20, "height": 400},
        "properties": {"mass": 10, "moi": 133666.6666666667},
        "linear": {"displacement": {"x": 0, "y": -200}, "velocity": {"x": 0, "y": 0}},
        "angular": {"displacement": 0, "velocity": 0}
    }, {
        "name": "Collider",
        "color": "#3b82f6",
        "shape": {"type": "Rectangle", "width": 40, "height": 40},
        "properties": {"mass": 5, "moi": 1333.3333333333335},
        "linear": {"displacement": {"x": -100, "y": -300}, "velocity": {"x": 50, "y": 0}},
        "angular": {"displacement": 0.7853981633974483, "velocity": 0}
    }],
    equations: [
        "s_Pendulum = (200sin(theta_Pendulum))hati+(-200cos(theta_Pendulum))hatj",
        "alpha_Pendulum*I_Pendulum=(200*(m_Pendulum*-100)*sin(theta_Pendulum))hatk"
    ]
}

export const Springs = {
    bodies: [{
        "name": "Source",
        "color": "#3b82f6",
        "shape": {"type": "Rectangle", "width": 40, "height": 40},
        "properties": {"mass": 1000000000000, "moi": 266666666666666.66},
        "linear": {"displacement": {"x": 0, "y": 0}, "velocity": {"x": 0, "y": 0}},
        "angular": {"displacement": 0, "velocity": 0}
    }, {
        "name": "Ideal",
        "color": "#2e7b18",
        "shape": {"type": "Rectangle", "width": 40, "height": 40},
        "properties": {"mass": 10, "moi": 2666.666666666667},
        "linear": {"displacement": {"x": 300, "y": 0}, "velocity": {"x": 0, "y": 0}},
        "angular": {"displacement": 0, "velocity": 0}
    }, {
        "name": "Source 2",
        "color": "#3b82f6",
        "shape": {"type": "Rectangle", "width": 40, "height": 40},
        "properties": {"mass": 1000000000000, "moi": 266666666666666.66},
        "linear": {"displacement": {"x": 0, "y": -100}, "velocity": {"x": 0, "y": 0}},
        "angular": {"displacement": 0, "velocity": 0}
    }, {
        "name": "Dampened",
        "color": "#f73b3b",
        "shape": {"type": "Rectangle", "width": 40, "height": 40},
        "properties": {"mass": 1, "moi": 266.66666666666663},
        "linear": {"displacement": {"x": 300, "y": -100}, "velocity": {"x": 0, "y": 0}},
        "angular": {"displacement": 0, "velocity": 0}
    }],
    equations: [
        "k=5",
        "c=0.5",
        "rest_x = 200",
        "m_Ideal*a_Ideal = (-k*(x_Ideal-rest_x))*hati",
        "m_Dampened*a_Dampened = (-k*(x_Dampened-rest_x)-c*(v_x_Dampened))*hati"
    ]
}

export const Templates = [
    {name: "Obstacle Course", state: ObstacleCourse},
    {name: "One body problem", state: OneBodyProblem},
    {name: "Three body problem", state: ThreeBodyProblem},
    {name: "Springs", state: Springs},
    {name: "Pendulum", state: Pendulum},
    {name: "Blank", state: Blank},
]