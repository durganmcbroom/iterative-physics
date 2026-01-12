use engine::collide::Collide2D;
use engine::err::EngineResult;
use engine::math::{Column, Vector};
use engine::math::solve::Environment;
use engine::spaces::Space2D;
use engine::{Body, BodyProperties, BodyState, Engine, Shape};
use wasm_bindgen::prelude::wasm_bindgen;

pub type EngineError = String;

#[wasm_bindgen]
pub struct Engine2D {
    inner: Engine<Space2D>,
}

#[wasm_bindgen]
impl Engine2D {
    pub fn new(
        bodies: Vec<Body2D>,
        equations: Vec<String>,
        delta_t: f64,
    ) -> Result<Self, EngineError> {
        Ok(Engine2D {
            inner: Engine::new(
                bodies.into_iter().map(|b| b.inner).collect(),
                Environment::build(
                    equations.iter().map(|x| x.as_str()).collect(),
                    engine::math::solve::builtin::functions(),
                    engine::math::solve::builtin::constants(),
                )
                .map_err(|x| x.kind.to_string())?,
                Box::new(Collide2D {}),
                delta_t,
                1.0,
            ),
        })
    }

    pub fn tick(&mut self) -> Result<(), EngineError> {
        self.inner.tick().map_err(|x| x.kind.to_string())
    }

    pub fn get_state(&self) -> Vec<Body2D> {
        self.inner
            .bodies()
            .iter()
            .map(|x| Body2D { inner: x.clone() })
            .collect()
    }
}

#[wasm_bindgen]
pub struct Body2D {
    inner: Body<Space2D>,
}

#[wasm_bindgen]
impl Body2D {
    pub fn new(
        name: String,
        mass: f64,
        width: f64,
        height: f64,

        x: f64,
        y: f64,
        v_x: f64,
        v_y: f64,
    ) -> Self {
        Body2D {
            inner: Body {
                name: name.clone(),
                shape: Shape::Rec(width, height),
                linear: BodyState {
                    displacement: Column::vector([x, y]),
                    velocity: Column::vector([v_x, v_y]),
                    acceleration: Column::vector([0.0, 0.0]),
                },
                angular: BodyState {
                    displacement: Column::empty(),
                    velocity: Column::empty(),
                    acceleration: Column::empty(),
                },
                properties: BodyProperties::rectangle(mass, width, height),
            },
        }
    }

    pub fn name(&self) -> String {
        self.inner.name.clone()
    }

    pub fn x(&self) -> f64 {
        *self.inner.linear.displacement.get(0)
    }

    pub fn y(&self) -> f64 {
        *self.inner.linear.displacement.get(1)
    }

    pub fn theta(&self) -> f64 {
        *self.inner.angular.displacement.get(0)
    }
}
