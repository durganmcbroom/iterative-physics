use crate::collide::{Collide, Collision};
use crate::err::{EngineResult, ErrorKind};
use crate::math::integration::{leapfrog_displacement, leapfrog_velocity};
use crate::math::solve::Environment;
use crate::math::{Column, Vector};
use std::collections::HashMap;

pub mod err;
pub mod math;

pub struct Basis {
    pub name: &'static str,
    pub axis: &'static str,
}

pub trait Space {
    type Linear: Vector;
    type Angular: Vector;

    const LINEAR_BASES: &'static [Basis];
    const ANGULAR_BASES: &'static [Basis];

    fn cross_both(w: &Self::Angular, r: &Self::Linear) -> Self::Linear;
    fn cross_linear(a: &Self::Linear, b: &Self::Linear) -> Self::Angular;
}

#[derive(Clone)]
pub struct BodyState<V: Vector> {
    pub displacement: V,
    pub velocity: V,
    pub acceleration: V,
}

#[derive(Clone)]
pub struct BodyProperties {
    pub mass: f64,
    // moment of inertia
    pub moi: f64,
}

impl BodyProperties {
    pub fn weightless() -> Self {
        BodyProperties {
            mass: 0.0,
            moi: 0.0,
        }
    }

    pub fn rectangle(mass: f64, height: f64, width: f64) -> BodyProperties {
        let moi = mass / 12.0 * (height.powi(2) + width.powi(2));

        BodyProperties { mass, moi }
    }
}

#[derive(Clone)]
pub enum Shape {
    Rec(f64, f64),     // Width, height
    Ellipse(f64, f64), // major/minor axis
    Manifold(Vec<Column<3>>),
}

#[derive(Clone)]
pub struct Body<S: Space> {
    pub name: String,
    pub shape: Shape,
    pub linear: BodyState<S::Linear>,
    pub angular: BodyState<S::Angular>,
    pub properties: BodyProperties,
}

impl<S: Space> Body<S> {
    pub fn at_rest(
        name: String,
        shape: Shape,
        position: S::Linear,
        rotation: S::Angular,
        properties: BodyProperties,
    ) -> Body<S> {
        Body {
            name,
            shape,
            linear: BodyState {
                displacement: position,
                velocity: S::Linear::empty(),
                acceleration: S::Linear::empty(),
            },
            angular: BodyState {
                displacement: rotation,
                velocity: S::Angular::empty(),
                acceleration: S::Angular::empty(),
            },
            properties,
        }
    }

}

pub struct Engine<S: Space> {
    bodies: Vec<Body<S>>,
    env: Environment,
    collider: Box<dyn Collide<S>>,
    delta_t: f64,
    restitution: f64,
    // unit_vectors: Vec<String>,
}

impl<S: Space + Clone> Engine<S> {
    pub fn new(
        bodies: Vec<Body<S>>,
        env: Environment,
        collider: Box<dyn Collide<S>>,
        delta_t: f64,
        restitution: f64,
    ) -> Self {
        Engine {
            bodies,
            env,
            collider,
            delta_t,
            restitution,
        }
    }

    fn eval_impl<O: Vector>(
        var: &'static str,
        owner: String,
        bases: &'static [Basis],
        env: &Environment,
        bodies: &Vec<Body<S>>,
        selector: fn(&Body<S>) -> &O,
    ) -> EngineResult<Option<Vec<f64>>> {
        let mut result = Vec::new();
        let mut overrides = HashMap::new();

        for i in 0..O::dof() {
            overrides.insert(bases[i].name.to_string(), 0.0);
        }

        for x in bodies {
            for i in 0..O::dof() {
                overrides.insert(format!("{}_{}", bases[i].axis, x.name), *selector(x).get(i));
            }
        }

        for i in 0..O::dof() {
            let form = format!("{}_{}", var, owner);
            overrides.insert(bases[i].name.to_string(), 1.0);
            let part = match env.evaluate(form.clone(), overrides.clone()) {
                Ok(x) => Ok(Some(x)),
                Err(e) => match e.kind.clone() {
                    ErrorKind::UnsatisfiedVariable(x) => {
                        if x == form {
                            Ok(None)
                        } else {
                            Err(e)
                        }
                    }
                    _ => Err(e),
                },
            };
            result.push(part);
            overrides.insert(bases[i].name.to_string(), 0.0);
        }

        result
            .into_iter()
            .collect::<EngineResult<Option<Vec<f64>>>>()
    }

    fn calculate_impulse(
        a: &Body<S>,
        b: &Body<S>,
        collision: &Collision<S>,
        restitution: f64,
    ) -> f64 {
        let n = collision.normal.unit();

        // --- 1. Calculate Relative Velocity (v_rel) ---
        // v_point = v_linear + (w x r)
        // We use S::cross_angular for the (w x r) term
        let v_a_point = a
            .linear
            .velocity
            .plus(&S::cross_both(&a.angular.velocity, &collision.a));

        let v_b_point = b
            .linear
            .velocity
            .plus(&S::cross_both(&b.angular.velocity, &collision.b));

        // v_rel = v_b - v_a
        // Assuming you have a minus, otherwise: b.plus(a.scale(-1.0))
        let v_rel = v_b_point.plus(&v_a_point.scale(-1.0));
        let v_rel_n = v_rel.dot(&n);

        // If separating, no impulse
        if v_rel_n > 0.0 {
            return 0.0;
        }

        // --- 2. Inverse Mass Terms ---
        // 1/m_a + 1/m_b
        let inv_mass_sum = (if a.properties.mass > 0.0 {
            1.0 / a.properties.mass
        } else {
            0.0
        }) + (if b.properties.mass > 0.0 {
            1.0 / b.properties.mass
        } else {
            0.0
        });

        // --- 3. Angular Terms ---
        // ((r x n)^2) / I
        // We use S::cross_linear for (r x n)
        let rxn_a = S::cross_linear(&collision.a, &n);
        let rxn_b = S::cross_linear(&collision.b, &n);

        // In 2D: rxn is Scalar. magnitude^2 is just value^2.
        // In 3D: rxn is Vector. magnitude^2 is dot(self, self).
        let ang_a = if a.properties.moi > 0.0 {
            rxn_a.magnitude().powi(2) / a.properties.moi
        } else {
            0.0
        };

        let ang_b = if b.properties.moi > 0.0 {
            rxn_b.magnitude().powi(2) / b.properties.moi
        } else {
            0.0
        };

        // --- 4. Final Calculation ---
        // j = -(1 + e) * v_rel_norm / (1/m + 1/m + ang_a + ang_b)
        let numerator = -(1.0 + restitution) * v_rel_n;
        let denominator = inv_mass_sum + ang_a + ang_b;

        if denominator == 0.0 {
            0.0
        } else {
            numerator / denominator
        }
    }

    // Just applying impulse to A.
    fn apply_impulse(a: &mut Body<S>, b: &Body<S>, collision: Collision<S>, restitution: f64) {
        let impulse = Self::calculate_impulse(a, b, &collision, restitution);

        // println!("Body: {}", a.name);
        // println!("{}", collision.normal);
        let delta_v = collision.normal.scale(impulse / a.properties.mass);
        let delta_omega =
            S::cross_linear(&collision.normal.scale(impulse / a.properties.moi), &collision.a,);

        a.linear.velocity = a.linear.velocity.plus(&delta_v.scale(-1.0));
        a.angular.velocity = a.angular.velocity.plus(&delta_omega);

        // b.linear.velocity = b.linear.velocity.plus(&delta_v.scale(-1.0));
        // b.angular.velocity = b.angular.velocity.plus(&delta_omega.scale(-1.0));
    }

    pub fn tick(&mut self) -> EngineResult<()> {
        let prev_state = self.bodies.clone();

        macro_rules! eval {
            ($var:literal, $name:expr, Linear) => {
                Engine::<S>::eval_impl::<S::Linear>(
                    $var,
                    $name.clone(),
                    S::LINEAR_BASES,
                    &self.env,
                    &(prev_state),
                    |x| &x.linear.displacement,
                )
            };
            ($var:literal, $name:expr, Angular) => {
                Engine::<S>::eval_impl::<S::Angular>(
                    $var,
                    $name.clone(),
                    S::ANGULAR_BASES,
                    &self.env,
                    &(prev_state),
                    |x| &x.angular.displacement,
                )
            };
        }

        macro_rules! update_state {
            (
                $body:expr,
                $displacement:literal,
                $velocity:literal,
                $acceleration:literal,
                $vec_kind:ty,
                $state:expr,
                $kind:ident
            ) => {{
                let s = eval!($displacement, $body.name, $kind)?;

                if let Some(s) = s {
                    $state.displacement = <$vec_kind>::new(s)?;
                } else {
                    let v = eval!($velocity, $body.name, $kind)?;

                    if let Some(v) = v {
                        let v = <$vec_kind>::new(v)?;

                        $state.velocity = v.clone();
                        $state.displacement = $state.displacement.plus(&v.scale(self.delta_t));
                    } else {
                        let a = eval!($acceleration, $body.name, $kind)?;

                        if let Some(a) = a {
                            $state.velocity = <$vec_kind>::new(
                                a.iter()
                                    .enumerate()
                                    .map(|(i, component)| {
                                        leapfrog_velocity(
                                            self.delta_t,
                                            *$state.velocity.get(i),
                                            *$state.acceleration.get(i),
                                            *component,
                                        )
                                    })
                                    .collect::<Vec<f64>>(),
                            )?;

                            $state.displacement = <$vec_kind>::new(
                                a.iter()
                                    .enumerate()
                                    .map(|(i, component)| {
                                        leapfrog_displacement(
                                            self.delta_t,
                                            *$state.displacement.get(i),
                                            *$state.velocity.get(i),
                                            *component,
                                        )
                                    })
                                    .collect::<Vec<f64>>(),
                            )?;

                            $state.acceleration = <$vec_kind>::new(a)?;
                        } else {
                            // If no definitions are present, just integrate velocity
                            $state.displacement = $state
                                .displacement
                                .plus(&($state.velocity).scale(self.delta_t));
                        }
                    }
                }
            }};
        }

        for mut a in self.bodies.iter_mut() {
            for b in prev_state.iter() {
                if a.name == b.name {
                    continue;
                }
                if let Some(collision) = self.collider.collide(a, b) {
                    Self::apply_impulse(&mut a, &b, collision, self.restitution);
                }
            }
        }

        for body in self.bodies.iter_mut() {
            update_state!(body, "s", "v", "a", S::Linear, body.linear, Linear);

            update_state!(
                body,
                "q",
                "omega",
                "alpha",
                S::Angular,
                body.angular,
                Angular
            );
        }

        Ok(())
    }

    pub fn bodies(&self) -> &Vec<Body<S>> {
        &self.bodies
    }

    pub fn print_diagnostics(&self) {
        println!("Engine state:");
        println!("Bodies: -----");

        fn to_string<T: Vector>(v: &T) -> String {
            let mut output = "".to_string();
            for i in 0..T::dof() {
                output += v.get(i).to_string().as_str();
                if i < T::dof() - 1 {
                    output += ", ";
                }
            }

            output
        }

        for x in self.bodies.iter() {
            println!(
                "-> {}: ({}) - {{{}}}",
                x.name,
                to_string(&x.linear.displacement),
                to_string(&x.linear.velocity),
            )
        }
    }
}

pub mod spaces {
    use crate::math::{Column, Vector};
    use crate::{Basis, Space};

    #[derive(Debug, Clone)]
    pub struct Space2D {}

    impl Space for Space2D {
        type Linear = Column<2>;
        type Angular = Column<1>;

        const LINEAR_BASES: &'static [Basis] = &[
            Basis {
                name: "hati",
                axis: "x",
            },
            Basis {
                name: "hatj",
                axis: "y",
            },
        ];
        const ANGULAR_BASES: &'static [Basis] = &[Basis {
            name: "hatk",
            axis: "theta",
        }];

        fn cross_both(w: &Self::Angular, r: &Self::Linear) -> Self::Linear {
            Column::vector([-w.get(0) * r.get(1), w.get(0) * r.get(0)])
        }

        fn cross_linear(a: &Self::Linear, b: &Self::Linear) -> Self::Angular {
            Column::vector([(a.get(0) * b.get(1)) - (a.get(1) * b.get(0))])
        }
    }
}

pub mod collide {
    use crate::math::{Column, Matrix, Vector};
    use crate::spaces::Space2D;
    use crate::{Body, Shape, Space};

    fn rot_2d(deg: f64) -> Matrix<2, 2> {
        Matrix::new([[deg.cos(), -deg.sin()], [deg.sin(), deg.cos()]])
    }

    #[derive(Debug, Clone)]
    pub struct Collision<S: Space> {
        pub a: S::Linear,
        pub b: S::Linear,
        pub normal: S::Linear, // Relative to A
    }

    pub trait Collide<S: Space> {
        fn collide(&self, a: &Body<S>, b: &Body<S>) -> Option<Collision<S>>;
    }

    pub struct Collide2D {}

    enum CollisionPoint {
        Parameterization { t_a: f64, t_b: f64 },
        Parallel,
        NonColliding,
    }

    impl Collide2D {
        pub fn new() -> Self {
            Collide2D {}
        }

        fn bases(body: &Body<Space2D>) -> Vec<Column<2>> {
            let shape = &body.shape;

            let untransformed = match shape {
                Shape::Rec(width, height) => {
                    vec![
                        Matrix::vector([width / 2.0, height / 2.0]),  // Top Right
                        Matrix::vector([-width / 2.0, height / 2.0]), // Top left
                        Matrix::vector([-width / 2.0, -height / 2.0]), // Bottom left
                        Matrix::vector([width / 2.0, -height / 2.0]), // Bottom Right
                    ]
                }
                Shape::Ellipse(_, _) => {
                    todo!()
                }
                Shape::Manifold(_) => {
                    todo!()
                }
            };

            let transformation = rot_2d(body.angular.displacement.content[0][0]);

            untransformed
                .iter()
                .map(|x| transformation.multiply(&x))
                .collect::<Vec<_>>()
        }
        /// Determine the collision boundary between two basis
        fn intersection_point(
            a: Column<2>,
            a_basis: Column<2>,
            b: Column<2>,
            b_basis: Column<2>,
            //       a  , b
        ) -> CollisionPoint {
            fn slope(col: &Column<2>) -> f64 {
                col.get(1) / col.get(0)
            }

            // If they are both in-line and parallel, t will be inf as any value of t satisfies the series of
            // equations. We have to manually check for this case.
            let (t_a, t_b) = if slope(&a_basis) == slope(&b_basis)  {
                // There is no more calculation we can do with the data here
                return CollisionPoint::Parallel;
            } else {
                // If slopes aren't the same, find the parameterized point of intersection
                // Ax=B => x=A^-1*B
                // Of the system of equations: (where a_basis, a, b_basis, and b are all column vectors)
                // t_a * a_basis + a = t_b * b_basis + b
                let product = Matrix::vector([b.get(0) - a.get(0), b.get(1) - a.get(1)]);
                let a_inv = Matrix::new([
                    [-*b_basis.get(1), *b_basis.get(0)],
                    [-*a_basis.get(1), *a_basis.get(0)],
                ])
                .scale(1.0 / (b_basis.get(0) * a_basis.get(1) - a_basis.get(0) * b_basis.get(1)));

                let x = a_inv.multiply(&product);

                (*x.get(0), *x.get(1))
            };

            if t_a >= 0.0 && t_a <= 1.0 && t_b >= 0.0 && t_b <= 1.0 {
                CollisionPoint::Parameterization { t_a, t_b }
            } else {
                CollisionPoint::NonColliding
            }
        }
    }

    impl Collide<Space2D> for Collide2D {
        fn collide(&self, a: &Body<Space2D>, b: &Body<Space2D>) -> Option<Collision<Space2D>> {
            // Each basis is a vector from the centroid of the object to a point of its face
            let a_bases = Self::bases(a);
            let b_bases = Self::bases(b);

            let mut collisions = Vec::<Collision<Space2D>>::new();

            macro_rules! basis {
                ($bases:expr, $i:expr) => {
                    // current point - next_point = current basis
                    &$bases
                        .get(($i + 1) % $bases.len())
                        .unwrap()
                        .plus(&$bases.get($i).unwrap().scale(-1.0))
                };
            }

            for i in 0..a_bases.len() {
                let point_a = a_bases.get(i).unwrap().plus(&a.linear.displacement);
                let basis_a = basis!(a_bases, i);

                for j in 0..b_bases.len() {
                    let point_b = b_bases.get(j).unwrap().plus(&b.linear.displacement);
                    let basis_b = basis!(b_bases, j);

                    let boundary = Self::intersection_point(
                        point_a.clone(),
                        basis_a.clone(),
                        point_b.clone(),
                        basis_b.clone(),
                    );

                    match boundary  {
                        CollisionPoint::Parameterization { t_a, t_b } => {
                            let collision_point_a = a_bases.get(i).unwrap().plus(&basis_a.scale(t_a));
                            let collision_point_b = b_bases.get(j).unwrap().plus(&basis_b.scale(t_b));

                            let normal = Matrix::vector([*basis_a.get(1), -basis_a.get(0)]);

                            collisions.push(Collision {
                                a: collision_point_a,
                                b: collision_point_b,
                                normal,
                            })
                        }
                        CollisionPoint::Parallel => {

                            // TODO (works without this but less complete)
                        }
                        CollisionPoint::NonColliding => {
                            // Nothing
                        }
                    }
                }
            }

            if collisions.is_empty() {
                return None;
            }

            let result = collisions.clone().into_iter().reduce(|acc, e| Collision {
                a: acc.a.plus(&e.a).scale(0.5),
                b: acc.b.plus(&e.b).scale(0.5),
                normal: acc.normal.plus(&e.normal).unit(),
            });

            result
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::{BodyProperties, BodyState};
        use std::f64::consts::PI;

        // #[test]
        // fn test_collinear_collision_basis() {
        //     // Same basis
        //     assert_eq!(
        //         Collide2D::intersection_point(
        //             Matrix::vector([1.0, 0.0]),
        //             Matrix::vector([1.0, 0.0]),
        //             Matrix::vector([0.0, 0.0]),
        //             Matrix::vector([1.0, 0.0]),
        //         ),
        //         CollisionPoint::Parameterization{t_a: 0.0, t_b: 0.0}
        //     );
        //
        //     // Collinear non-overlapping basis
        //     assert_eq!(
        //         Collide2D::intersection_point(
        //             Matrix::vector([2.0, 0.0]),
        //             Matrix::vector([1.0, 0.0]),
        //             Matrix::vector([0.0, 0.0]),
        //             Matrix::vector([1.0, 0.0]),
        //         ),
        //         CollisionPoint::NonColliding
        //     );
        //
        //     // Collinear overlapping x
        //     assert_eq!(
        //         Collide2D::intersection_point(
        //             Matrix::vector([0.5, 0.0]),
        //             Matrix::vector([1.0, 0.0]),
        //             Matrix::vector([0.0, 0.0]),
        //             Matrix::vector([1.0, 0.0]),
        //         ),
        //         CollisionPoint::Parameterization{t_a: 0.0, t_b: 0.0}
        //     );
        //
        //     // Collinear overlapping y
        //     assert_eq!(
        //         Collide2D::intersection_point(
        //             Matrix::vector([0.0, 0.0]),
        //             Matrix::vector([0.0, 1.0]),
        //             Matrix::vector([0.0, 1.0]),
        //             Matrix::vector([0.0, 1.0]),
        //         ),
        //         Some((0.0, 0.0))
        //     );
        //
        //     // Collinear overlapping xy
        //     assert_eq!(
        //         Collide2D::intersection_point(
        //             Matrix::vector([0.0, 0.0]),
        //             Matrix::vector([1.0, 1.0]),
        //             Matrix::vector([0.5, 0.5]),
        //             Matrix::vector([1.0, 1.0]),
        //         ),
        //         Some((0.0, 0.0))
        //     );
        //
        //     // Collinear non-overlapping xy
        //     assert_eq!(
        //         Collide2D::intersection_point(
        //             Matrix::vector([0.0, 0.0]),
        //             Matrix::vector([1.0, 1.0]),
        //             Matrix::vector([1.5, 1.5]),
        //             Matrix::vector([1.0, 1.0]),
        //         ),
        //         None
        //     );
        // }

        // #[test]
        // fn test_intersecting_collision_boundaries() {
        //     // Perpendicular
        //     assert_eq!(
        //         Collide2D::intersection_point(
        //             Matrix::vector([0.0, 0.5]),
        //             Matrix::vector([1.0, 0.0]),
        //             Matrix::vector([0.5, 0.0]),
        //             Matrix::vector([0.0, 1.0]),
        //         ),
        //         Some((0.5, 0.5))
        //     );
        //
        //     // Slanted
        //     assert_eq!(
        //         Collide2D::intersection_point(
        //             Matrix::vector([1.0, 0.0]),
        //             Matrix::vector([0.0, 2.0]),
        //             Matrix::vector([2.0, 0.0]),
        //             Matrix::vector([-2.0, 4.0]),
        //         ),
        //         Some((1.0, 0.5))
        //     );
        // }

        #[test]
        fn test_form_basis() {
            let body = Body {
                name: "A".to_string(),
                shape: Shape::Rec(2.0, 2.0),
                linear: BodyState {
                    displacement: Matrix::vector([0.0, 0.0]),
                    velocity: Matrix::vector([0.0, 0.0]),
                    acceleration: Matrix::empty(),
                },
                angular: BodyState {
                    displacement: Matrix::vector([0.0]),
                    velocity: Matrix::vector([0.0]),
                    acceleration: Matrix::empty(),
                },
                properties: BodyProperties::weightless(),
            };

            let basis = Collide2D::bases(&body);
            assert_eq!(
                basis,
                vec![
                    Matrix::vector([1.0, 1.0]),
                    Matrix::vector([-1.0, 1.0]),
                    Matrix::vector([-1.0, -1.0]),
                    Matrix::vector([1.0, -1.0]),
                ]
            );
            println!("Basis {:?}", basis);

            let body = Body {
                name: "A".to_string(),
                shape: Shape::Rec(4.0, 2.0),
                linear: BodyState {
                    displacement: Matrix::vector([0.0, 0.0]),
                    velocity: Matrix::vector([0.0, 0.0]),
                    acceleration: Matrix::empty(),
                },
                angular: BodyState {
                    displacement: Matrix::vector([PI / 4.0]),
                    velocity: Matrix::vector([0.0]),
                    acceleration: Matrix::empty(),
                },
                properties: BodyProperties::weightless(),
            };

            let basis = Collide2D::bases(&body);
            println!("Second Basis");
            for x in basis {
                // Values won't be exact because of trig, just print it for verification.
                println!("({}, {})", x.get(0), x.get(1));
            }
        }

        #[test]
        fn test_face_collision() {
            let c2d = Collide2D {};

            let a = Body {
                name: "A".to_string(),
                shape: Shape::Rec(2.0, 2.0),
                linear: BodyState {
                    displacement: Matrix::vector([0.0, 0.0]),
                    velocity: Matrix::vector([0.0, 0.0]),
                    acceleration: Matrix::empty(),
                },
                angular: BodyState {
                    displacement: Matrix::vector([0.0]),
                    velocity: Matrix::vector([0.0]),
                    acceleration: Matrix::empty(),
                },
                properties: BodyProperties::weightless(),
            };

            let b = Body {
                name: "B".to_string(),
                shape: Shape::Rec(2.0, 2.0),
                linear: BodyState {
                    displacement: Matrix::vector([2.0, 0.0]),
                    velocity: Matrix::vector([0.0, 0.0]),
                    acceleration: Matrix::empty(),
                },
                angular: BodyState {
                    displacement: Matrix::vector([PI / 4.0]),
                    velocity: Matrix::vector([0.0]),
                    acceleration: Matrix::empty(),
                },
                properties: BodyProperties::weightless(),
            };

            let collision = c2d.collide(&a, &b);

            println!("{:?}", collision)
        }
    }
}
