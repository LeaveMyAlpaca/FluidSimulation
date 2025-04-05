# Fluid Simulation

## [Video Demonstration](https://www.youtube.com/watch?v=QOP5tywBQ6o)

This project is a fluid simulation implemented in Rust using the Bevy game engine.

-   The simulation is based on the concepts presented in [this video](https://www.youtube.com/watch?v=rSKMYc1CQHE).
-   It achieves approximately 15 frames per second with 100,000 particles, performing 3 physics updates per frame (approximately 45 updates per second) in debug mode.
-   The simulation utilizes a maximum of 40-45% of CPU resources (Ryzen 7950X3D).

## Key Bottlenecks:

-   Bevy's sprite rendering performance.
-   Suboptimal multi-threading within the core algorithm.
-   Inability to run the simulation in release mode.

## Potential Improvements:

-   Profiling with the `tracey` profiling tool to identify performance hotspots.
-   Implementing custom mesh rendering for particles to bypass sprite rendering limitations.
