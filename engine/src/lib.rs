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

    pub fn rectangle(mass: f64, width: f64, height: f64) -> BodyProperties {
        let moi = mass / 12.0 * (width.powi(2) + height.powi(2));

        BodyProperties { mass, moi }
    }
}

#[derive(Clone)]
pub enum Shape<S: Space> {
    Rec(f64, f64),     // Width, height
    Ellipse(f64, f64), // major/minor axis
    Manifold(Vec<S::Linear>),
}

#[derive(Clone)]
pub struct Body<S: Space> {
    pub name: String,
    pub shape: Shape<S>,
    pub linear: BodyState<S::Linear>,
    pub angular: BodyState<S::Angular>,
    pub properties: BodyProperties,
}

impl<S: Space> Body<S> {
    pub fn at_rest(
        name: String,
        shape: Shape<S>,
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
}

pub struct Tick<S: Space> {
    pub collisions: Vec<S::Linear>,
}

const CORRECTIVE_FRAMES: usize = 100;
const CORRECTIVE_TUNING_VALUE: f64 = 10.0;

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

    fn eval_impl<Primary: Vector, Secondary: Vector>(
        var: &'static str,
        owner: String,

        bases: &'static [Basis],
        selector: fn(&Body<S>) -> &BodyState<Primary>,

        extra_bases: &'static [Basis],
        extra_selector: fn(&Body<S>) -> &BodyState<Secondary>,

        env: &Environment,
        bodies: &Vec<Body<S>>,
    ) -> EngineResult<Option<Vec<f64>>> {
        let mut result = Vec::new();
        let mut overrides = HashMap::new();

        // Set all bases to 0 (hati, hatj, hatk etc.)
        for i in 0..Primary::dof() {
            overrides.insert(bases[i].name.to_string(), 0.0);
        }

        // Initialize body constants / properties
        for x in bodies {
            for i in 0..Primary::dof() {
                overrides.insert(format!("{}_{}", bases[i].axis, x.name), *selector(x).displacement.get(i));
            }

            for i in 0..Primary::dof() {
                overrides.insert(format!("v_{}_{}", bases[i].axis, x.name), *selector(x).velocity.get(i));
            }

            for i in 0..Secondary::dof() {
                overrides.insert(
                    format!("{}_{}", extra_bases[i].axis, x.name),
                    *extra_selector(x).displacement.get(i),
                );
            }

            for i in 0..Secondary::dof() {
                overrides.insert(
                    format!("v_{}_{}", extra_bases[i].axis, x.name),
                    *extra_selector(x).velocity.get(i),
                );
            }

            overrides.insert(format!("m_{}", x.name), x.properties.mass);
            overrides.insert(format!("I_{}", x.name), x.properties.moi);
        }

        for i in 0..Primary::dof() {
            // Work on basis i (set it to 1).
            overrides.insert(bases[i].name.to_string(), 1.0);

            let form = format!("{}_{}", var, owner);
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
        let pen_a = &collision.point.plus(&a.linear.displacement.scale(-1.0));
        let pen_b = &collision.point.plus(&b.linear.displacement.scale(-1.0));

        let v_a_point = a
            .linear
            .velocity
            .plus(&S::cross_both(&a.angular.velocity, pen_a));

        let v_b_point = b
            .linear
            .velocity
            .plus(&S::cross_both(&b.angular.velocity, pen_b));

        // v_rel = v_b - v_a
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
        let rxn_a = S::cross_linear(&pen_a, &n);
        let rxn_b = S::cross_linear(&pen_b, &n);

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
    fn apply_impulse(a: &mut Body<S>, b: &mut Body<S>, collision: Collision<S>, restitution: f64) {
        let impulse = Self::calculate_impulse(a, b, &collision, restitution);

        println!("impulse = {:?}", impulse);
        macro_rules! do_apply {
            ($body:ident, $inv:literal) => {{
                let delta_v = &collision
                    .normal
                    .scale($inv * impulse / $body.properties.mass);

                let delta_omega = S::cross_linear(
                    &collision.point.plus(&$body.linear.displacement.scale(-1.0)),
                    &collision
                        .normal
                        .scale($inv * impulse / $body.properties.moi),
                );

                $body.linear.velocity = $body.linear.velocity.plus(&delta_v);
                $body.angular.velocity = $body.angular.velocity.plus(&delta_omega);
            }};
        }

        do_apply!(a, -1.0);
        do_apply!(b, 1.0);
        // do_apply!(b, -1.0);
    }

    // Push A away
    fn apply_correction(
        collider: &Box<dyn Collide<S>>,
        a: &mut Body<S>,
        b: &Body<S>,
        mut collision: Collision<S>,
    ) {
        // Start with the inverse of the distance between them. (Small distance, large correction, etc.) and account for mass.
        for _ in 0..CORRECTIVE_FRAMES {
            let correction = collision.depth / a.properties.mass;

            a.linear.displacement = a
                .linear
                .displacement
                .plus(&collision.normal.scale(-correction));

            if let Some(c) = collider.collide(a, b) {
                collision = c;
            } else {
                break;
            }
        }
    }

    pub fn tick(&mut self) -> EngineResult<Tick<S>> {
        let prev_state = self.bodies.clone();

        macro_rules! eval {
            ($var:literal, $name:expr, Linear) => {
                Engine::<S>::eval_impl::<S::Linear, S::Angular>(
                    $var,
                    $name.clone(),
                    S::LINEAR_BASES,
                    |x| &x.linear,
                    S::ANGULAR_BASES,
                    |x| &x.angular,
                    &self.env,
                    &(prev_state),
                )
            };
            ($var:literal, $name:expr, Angular) => {
                Engine::<S>::eval_impl::<S::Angular, S::Linear>(
                    $var,
                    $name.clone(),
                    S::ANGULAR_BASES,
                    |x| &x.angular,
                    S::LINEAR_BASES,
                    |x| &x.linear,
                    &self.env,
                    &(prev_state),
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
                $kind:ident,
                $skip_accel:expr
            ) => {{
                let s = eval!($displacement, $body.name, $kind)?;

                if let Some(s) = s {
                    let old_displacement = $state.displacement.clone();
                    $state.displacement = <$vec_kind>::new(s)?;

                    $state.velocity = $state
                        .displacement
                        .plus(&old_displacement.scale(-1.0))
                        .scale(self.delta_t);
                } else {
                    let v = eval!($velocity, $body.name, $kind)?;

                    if let Some(v) = v {
                        let v = <$vec_kind>::new(v)?;

                        $state.velocity = v.clone();
                        $state.displacement = $state.displacement.plus(&v.scale(self.delta_t));
                    } else {
                        let a = eval!($acceleration, $body.name, $kind)?;

                        if let Some(a) = a
                            && !$skip_accel
                        {
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

        for body in self.bodies.iter_mut() {
            update_state!(body, "s", "v", "a", S::Linear, body.linear, Linear, false);

            update_state!(
                body,
                "q",
                "omega",
                "alpha",
                S::Angular,
                body.angular,
                Angular,
                false
            );
        }

        let mut tick = Tick {
            collisions: Vec::<S::Linear>::new(),
        };

        for i in 0..self.bodies.len() {
            let (left, right) = self.bodies.split_at_mut(i + 1);
            let a = &mut left[i];

            for j in 0..right.len() {
                let b = &mut right[j];

                if let Some(collision) = self.collider.collide(a, b) {
                    tick.collisions.push(collision.point.clone());

                    Self::apply_impulse(a, b, collision.clone(), self.restitution);

                    Self::apply_correction(&self.collider, a, b, collision.clone());
                    Self::apply_correction(&self.collider, b, a, collision);

                    // TODO: Friction
                }
            }
        }

        Ok(tick)
    }

    pub fn bodies(&self) -> &Vec<Body<S>> {
        &self.bodies
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
    use std::collections::HashSet;

    fn rot_2d(deg: f64) -> Matrix<2, 2> {
        Matrix::new([[deg.cos(), -deg.sin()], [deg.sin(), deg.cos()]])
    }

    #[derive(Debug, Clone)]
    pub struct Collision<S: Space> {
        pub point: S::Linear,
        pub normal: S::Linear, // Relative to A
        pub depth: f64,        // Relative to A
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
                Shape::Manifold(p) => p.clone(),
            };

            let transformation = rot_2d(body.angular.displacement.content[0][0]);

            untransformed
                .iter()
                .map(|x| transformation.multiply(&x))
                .collect::<Vec<_>>()
        }

        fn intersect(
            a: &Column<2>,
            a_basis: &Column<2>,
            b: &Column<2>,
            b_basis: &Column<2>,
        ) -> Option<(f64, f64)> {
            fn slope(col: &Column<2>) -> f64 {
                col.get(1) / col.get(0)
            }

            // If they are both in-line and parallel, t will be inf as any value of t satisfies the series of
            // equations. We have to manually check for this case.
            let (t_a, t_b) = if slope(&a_basis) == slope(&b_basis) {
                // There is no more calculation we can do with the data here
                return None;
                // return CollisionPoint::Parallel;
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

            Some((t_a, t_b))
        }

        /// Determine the collision boundary between two basis
        fn intersection_point(
            a: &Column<2>,
            a_basis: &Column<2>,
            b: &Column<2>,
            b_basis: &Column<2>,
            //       a  , b
        ) -> Option<Column<2>> {
            let (t_a, t_b) = Self::intersect(a, a_basis, b, b_basis)?;

            if t_a >= 0.0 && t_a <= 1.0 && t_b >= 0.0 && t_b <= 1.0 {
                Some(a.plus(&a_basis.scale(t_a)))
                // CollisionPoint::Parameterization { t_a, t_b }
            } else {
                None
            }
        }
    }

    // Point to line (defined by two points) distance
    fn ptl_distance(point: &Column<2>, a: &Column<2>, b: &Column<2>) -> f64 {
        let numerator =
            (b[1] - a[1]) * point[0] - (b[0] - a[0]) * point[1] + b[0] * a[1] - b[1] * a[0];

        let denominator = ((b[1] - a[1]).powi(2) + (b[0] - a[0]).powi(2)).sqrt();

        numerator.abs() / denominator
    }

    impl Collide<Space2D> for Collide2D {
        // Runs in NlogN
        fn collide(&self, a: &Body<Space2D>, b: &Body<Space2D>) -> Option<Collision<Space2D>> {
            // Each basis is a vector from the centroid of the object to a point of its face
            let a_bases = Self::bases(a);
            let b_bases = Self::bases(b);

            // World-Space
            let mut intersection_groups = Vec::<Vec<Column<2>>>::new();
            let mut collisions = 0;
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

                let mut intersections = Vec::<Column<2>>::new();

                for j in 0..b_bases.len() {
                    let point_b = b_bases.get(j).unwrap().plus(&b.linear.displacement);
                    let basis_b = basis!(b_bases, j);

                    let intersection =
                        Self::intersection_point(&point_a, basis_a, &point_b, basis_b);

                    if let Some(intersection) = intersection {
                        intersections.push(intersection);
                        collisions += 1;
                    }
                }

                intersection_groups.push(intersections);
            }

            if collisions == 0 {
                return None;
            }

            // World-Space
            let collision_point = intersection_groups
                .iter()
                .flatten()
                .fold(Column::empty(), |acc, it| acc.plus(&it))
                .scale(1.0 / collisions as f64);

            let intersections = intersection_groups
                .into_iter()
                .flat_map(|mut group| {
                    group.sort_unstable_by(|a, b| {
                        // Note that this will not handle points collinear with the intersection_point (could create instability)
                        let a = a.plus(&collision_point.scale(-1.0));
                        let a = (a[1] - a[0]).atan();

                        let b = b.plus(&collision_point.scale(-1.0));
                        let b = (b[1] - b[0]).atan();

                        a.partial_cmp(&b).unwrap()
                    });
                    group
                })
                .collect::<Vec<_>>();

            let normal_face = intersections
                .iter()
                .zip(intersections.iter().cycle().skip(1))
                .min_by(|(a1, b1), (a2, b2)| {
                    ptl_distance(&collision_point, a1, b1).total_cmp(&ptl_distance(
                        &collision_point,
                        a2,
                        b2,
                    ))
                })?;

            let normal_face = normal_face.0.plus(&normal_face.1.scale(-1.0));
            let normal = Column::vector([-normal_face[1], normal_face[0]]);

            // A vector of polygons alternating side
            let mut polygons: Vec<Vec<Column<2>>> = vec![vec![]];
            let mut side = 0.0;

            for (i, point) in a_bases.iter().enumerate() {
                let point = point.plus(&a.linear.displacement);

                if side == 0.0 {
                    let (t, _) =
                        Self::intersect(&point, &normal, &collision_point, &normal_face).unwrap();

                    if t != 0.0 {
                        side = t / t.abs()
                    }
                }

                let len = polygons.len();
                polygons[len - 1].push(point.clone());

                let next = a_bases
                    .get((i + 1) % a_bases.len())
                    .unwrap()
                    .plus(&a.linear.displacement);

                let face = next.plus(&point.scale(-1.0));

                if let Some((t, _)) = Self::intersect(&point, &face, &collision_point, &normal_face)
                    && t >= 0.0
                    && t < 1.0
                {
                    let intersection_point = point.plus(&face.scale(t));

                    polygons[len - 1].push(intersection_point.clone());

                    let mut new_polygon = vec![];
                    new_polygon.push(intersection_point);
                    polygons.push(new_polygon);
                }

                if i == a_bases.len() - 1
                    && let Some(pop) = polygons.pop()
                    && let Some(first) = polygons.first_mut()
                {
                    for x in pop {
                        first.push(x)
                    }
                }
            }

            // https://en.wikipedia.org/wiki/Shoelace_formula
            fn do_sum(polygons: &Vec<Vec<Column<2>>>, skip: usize) -> f64 {
                polygons
                    .iter()
                    .skip(skip)
                    .step_by(2)
                    .map(|poly| {
                        poly.iter()
                            .zip(poly.iter().cycle().skip(1))
                            .map(|(a, b)| Matrix::new([[a[0], b[0]], [a[1], b[1]]]).det())
                            .sum::<f64>()
                    })
                    .sum::<f64>()
                    / 2.0
            }

            let (area_a, area_b) = (do_sum(&polygons, 0), do_sum(&polygons, 1));

            let area_modifier = if area_a > area_b { 1.0 } else { -1.0 };

            let normal = normal.scale(side * area_modifier).unit();

            let penetration_depth = a_bases
                .iter()
                .zip(a_bases.iter().cycle().skip(1))
                .map(|(base_a, base_b)| {
                    ptl_distance(
                        &collision_point,
                        &a.linear.displacement.plus(&base_a),
                        &a.linear.displacement.plus(&base_b),
                    )
                })
                .min_by(|a, b| a.partial_cmp(b).unwrap());

            Some(Collision {
                point: collision_point,
                normal,
                depth: penetration_depth.unwrap_or(0.0),
            })
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::{BodyProperties, BodyState};
        use std::f64::consts::PI;

        #[test]
        fn test_rotated_collision() {
            let collide = Collide2D {};
            let collision = collide.collide(
                &Body::at_rest(
                    "A".to_string(),
                    Shape::Rec(2.0, 2.0),
                    Column::vector([0.0, 0.0]),
                    Column::vector([0.0]),
                    BodyProperties::rectangle(0.0, 0.0, 0.0),
                ),
                &Body::at_rest(
                    "B".to_string(),
                    Shape::Rec(2.0, 2.0),
                    Column::vector([0.0, 2.2]),
                    Column::vector([-PI / 8.0]),
                    BodyProperties::rectangle(0.0, 0.0, 0.0),
                ),
            );

            if let Some(x) = collision {
                println!("Collision: {:?}", x);
            }
        }

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
