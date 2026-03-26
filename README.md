## Golf

> seb

Golf ball flight simulation using `Rust` and `bevy` to run in the browser using `WASM`.

Read [this blog post on my personal website](https://sebblanchet.com/blog/2026/03/26/golf-sim/)

### Features

- Runs in either browser or on desktop
- Change club head loft, speed, spin
- Visualize in 3D world
- Plotting and data export via CSV for data validation
- User inputs for club head speed, loft, spins, etc..

### Build

Build the WASM blob and deploy:

```bash
./bin/docker.sh
```

Or developp locally:

```bash
cargo run
```

### References

1. [Barber III, J. (n.d.). Golf ball flight dynamics.](https://fragelada.fysik.org/resurser/GolfBallDynamics.pdf)
1. [The physics of golf - A Raymond Penner](https://raypenner.com/golf-physics.pdf)
1. [Golf Ball Flight Dynamics](https://www.math.union.edu/~wangj/courses/previous/math238w13/Golf%20Ball%20Flight%20Dynamics2.pdf)
1. [The optimum loft of a driver - A. Raymond Penner](https://www.researchgate.net/profile/Albert-Penner/publication/243492348_The_physics_of_golf_The_optimum_loft_of_a_driver/links/5852e23a08ae0c0f322275a8/The-physics-of-golf-The-optimum-loft-of-a-driver.pdf)
1. [Golf ball sim](https://github.com/sb362/golfball-simulation)
