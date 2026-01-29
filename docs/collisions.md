# Physics Engine: Collision Resolution & Dynamics

This module implements a rigid-body physics engine capable of simulating 2D dynamics, including collision detection, impulse resolution, and numerical integration.

## 1. Collision Detection Strategy

Unlike many standard 2D physics engines that utilize the **Separating Axis Theorem (SAT)**, this engine implements a **Direct Edge Intersection** approach (similar to Sutherland-Hodgman clipping).

### Why not SAT?

While SAT is highly efficient for determining overlaps, it is fundamentally limited to **convex** polygons. To simulate concave shapes with SAT, one must decompose the shape into multiple convex hulls (Convex Decomposition), which adds significant complexity to the asset pipeline and runtime structure.

**We chose Direct Edge Intersection because:**
1.  **Concave Support:** It natively handles concave bodies (via `Shape::Manifold`) without needing decomposition. By mathematically solving for the intersection of every edge pair, the engine detects collisions regardless of the geometry's convexity.
2.  **Precise Contact Points:** It explicitly calculates the exact coordinates where edges cross, providing a high-fidelity contact manifold for the resolution step.

### The Algorithm (`Collide2D`)

The detection process in `collide()` follows these steps:

1.  **Basis Transformation:**
    The engine transforms the local shape vertices into World Space using the body's current linear displacement and a rotation matrix derived from its angular displacement.

2.  **Edge-Edge Intersection:**
    It iterates through every edge of Body A and compares it against every edge of Body B. The intersection is solved as a system of linear equations:

    $$P_A + t_a \vec{d}_A = P_B + t_b \vec{d}_B$$

    This is solved via matrix inversion in `intersect()`. If $0 \le t_a, t_b \le 1$, the edges intersect.

3.  **Manifold Generation:**
    * **Centroid:** All intersection points are collected, and their average is calculated to find the "center" of the collision.
    * **Normal Selection:** The collision normal is determined by finding the face of the geometry closest to this collision centroid.
    * **Shoelace Formula:** To resolve complex overlaps, the engine constructs polygons from the intersecting vertices and calculates their signed areas using the Shoelace Formula. This helps determine the direction and magnitude of the correction required.

## 2. Collision Resolution (Impulse Method)

Once a collision is detected, the engine resolves it using **Impulse-Based Dynamics**. This instantaneously changes the velocities of the bodies without altering their positions (positions are corrected separately).

The implementation (`calculate_impulse`) handles both linear and angular components, allowing objects to spin when hit off-center.