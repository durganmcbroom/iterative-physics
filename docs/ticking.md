# Physics Engine: The Ticking Process

The core of the simulation drives the state of the world forward in discrete time steps (`delta_t`). This process, encapsulated in the `tick()` method, orchestrates the interplay between the equation solver, the numerical integrator, and the collision resolution system.

## 1. The Game Loop (`tick`)

Every frame, the engine performs the following operations in strict order:

1. **Evaluation & Integration:** The engine queries the mathematical environment to determine the new forces, velocities, and positions for every body.

2. **State Update:** The `BodyState` (linear and angular) is updated using a **Leapfrog Integrator**.

3. **Collision Detection:** The engine checks for overlaps between all pairs of bodies.

4. **Resolution:** If a collision is detected, impulses are applied immediately to resolve velocity, followed by positional corrections.

## 2. Integration Strategy: Leapfrog

Standard Euler integration ($x += v * dt$) is often unstable and drifts significantly over time. This engine uses a **Symplectic Euler (Leapfrog)** variant, which offers better energy conservation for orbital mechanics and rigid body dynamics.

### The Process

For every degree of freedom (Linear `x`, `y` and Angular `theta`):

1. **Query Solver:** The engine asks the `Environment` (see `SOLVER_README.md`) for the current value of acceleration (e.g., `a_x_BodyA`).

2. **Update Velocity:** $$
   v_{i+1} = v_i + a_i \cdot \Delta t
   $$

3. **Update Position:** $$
   x_{i+1} = x_i + v_{i+1} \cdot \Delta t
   $$

By updating velocity *before* position and using the *new* velocity to calculate the position, the system remains semi-implicit and stable.

### Dynamic Overrides

The engine is flexible. It attempts to resolve variables in a specific order:

* **Position (`s`)**: If a formula like `s_x_BodyA = sin(time)` exists, it overrides physics entirely (teleportation/kinematic control).

* **Velocity (`v`)**: If defined, it overrides acceleration (constant velocity motion).

* **Acceleration (`a`)**: The default physics path.

## 3. Collision Resolution (Impulse Method)

Once the integrator has moved the bodies, they may be overlapping. The engine resolves this using **Impulse-Based Dynamics**. This instantaneously changes the velocities of the bodies without altering their positions (positions are corrected separately).

### Impulse Calculation

The magnitude of the impulse scalar $j$ is calculated to satisfy the collision constraint (bodies must not move towards each other).

$$
j = \frac{-(1 + e) \cdot (\vec{v}_{rel} \cdot \vec{n})}{ \frac{1}{m_A} + \frac{1}{m_B} + \frac{(\vec{r}_A \times \vec{n})^2}{I_A} + \frac{(\vec{r}_B \times \vec{n})^2}{I_B} }
$$

Where:

* $e$: Coefficient of restitution (bounciness).

* $\vec{v}_{rel}$: Relative velocity at the point of contact.

* $\vec{n}$: Collision normal.

* $m$: Mass.

* $I$: Moment of Inertia.

* $\vec{r}$: Vector from center of mass to contact point.

### Application

1. **Velocity Update:** The impulse $j$ is applied along the normal:

    * $\Delta \vec{v} = \frac{j \cdot \vec{n}}{m}$

    * $\Delta \omega = I^{-1} (\vec{r} \times (j \cdot \vec{n}))$

   This results in realistic reactions where hitting an object off-center causes it to spin.

2. **Positional Correction:** To prevent objects from sinking into each other due to floating-point errors or high speeds (tunneling), the engine applies a "sinking correction".

    * It iteratively pushes the bodies apart along the collision normal based on the penetration depth.

    * This is weighted by mass (lighter objects are pushed further than heavier ones).