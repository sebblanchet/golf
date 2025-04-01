## Golf

> seb

Golf ball flight simulation using `Rust` and `bevy` to run in the browser using `WASM`.


### Features

- Runs in browser
- Change club head loft, speed, spin
- Visualize in 3D world

## Developping

Install required cargo deps and run standalone as native app:

```bash
cd sim
./bin/install.sh

cargo run
```

### Build

Full docker build with `wasm-bindgen` for web artifacts:

```bash
cd sim
./bin/docker.sh
```

### References

1. [Barber III, J. (n.d.). Golf ball flight dynamics.](https://fragelada.fysik.org/resurser/GolfBallDynamics.pdf)
1. [The physics of golf - A Raymond Penner](https://raypenner.com/golf-physics.pdf)
1. [Golf Ball Flight Dynamics](https://www.math.union.edu/~wangj/courses/previous/math238w13/Golf%20Ball%20Flight%20Dynamics2.pdf)
1. [The optimum loft of a driver - A. Raymond Penner](https://www.researchgate.net/profile/Albert-Penner/publication/243492348_The_physics_of_golf_The_optimum_loft_of_a_driver/links/5852e23a08ae0c0f322275a8/The-physics-of-golf-The-optimum-loft-of-a-driver.pdf)
1. [Golf ball sim](https://github.com/sb362/golfball-simulation)
