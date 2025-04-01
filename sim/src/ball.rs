use bevy::color::palettes::basic::BLUE;
use bevy::math::NormedVectorSpace;
use bevy::prelude::*;

use std::f32::consts::PI;
use std::fs;
//use std::time::{SystemTime, UNIX_EPOCH};

use crate::csv;
use crate::state;

#[derive(Clone, Component)]
pub struct Ball {
    pub time: Vec<f32>,
    pub position: Vec<Vec3>,
    pub velocity: Vec<Vec3>,
    pub acceleration: Vec<Vec3>,
    pub spin: Vec<Vec3>,
    pub f_g: Vec<Vec3>,
    pub f_d: Vec<Vec3>,
    pub f_m: Vec<Vec3>,
    pub start: String,
    pub inputs: state::Inputs,
    pub a: f32,
}

impl Ball {
    pub fn new(inputs: &state::Inputs, t: f32) -> Self {
        //let start = SystemTime::now()
        //    .duration_since(UNIX_EPOCH)
        //    .unwrap_or_default()
        //    .as_secs()
        //    .to_string();
        //    TODO wasm
        let start = "0001".to_string();

        let a = PI * inputs.r.powi(2);

        Self {
            time: vec![t],
            position: vec![inputs.position],
            velocity: vec![inputs.velocity],
            spin: vec![inputs.spin],
            acceleration: vec![Vec3::ZERO],
            f_g: vec![Vec3::ZERO],
            f_d: vec![Vec3::ZERO],
            f_m: vec![Vec3::ZERO],
            start,
            a,
            inputs: inputs.clone(),
        }
    }

    pub fn _init() {
        // TODO
    }

    pub fn lift(&self, velocity: Vec3, rvelocity: Vec3) -> Vec3 {
        let s = self.inputs.r * rvelocity.norm() / velocity.norm();
        let c_m = (-3.25 * s.powi(2)) + 1.99 * s;

        if c_m == 0. {
            return Vec3::ZERO;
        }

        let f_m = 0.5 * self.inputs.rho * self.a * c_m;

        // cross product of angular velocity and linear velocity, for direction of spin
        let rxv = rvelocity.cross(velocity);
        let rl = rxv.norm();
        let rxv = rxv / rl;

        // magnitude of spin is considered in coefficient of lift
        f_m * velocity.norm().powi(2) * rxv
    }

    pub fn drag(&self, velocity: Vec3) -> Vec3 {
        let speed = velocity.norm();
        let unit = velocity / speed;
        let c_d = sphere_cd(speed, self.inputs.r, self.inputs.mu);
        -0.5 * c_d * self.inputs.rho * self.a * speed.powi(2) * unit
    }

    pub fn gravity(&self) -> Vec3 {
        let g = Vec3::new(0., -9.81, 0.);
        self.inputs.m * g
    }

    pub fn save_params(&self) {
        // TODO wasm
        let path = format!("out/{}_params.csv", self.start);

        let head = vec![
            "m".to_string(),
            "r".to_string(),
            "rho".to_string(),
            "mu".to_string(),
            "a".to_string(),
        ];
        let v = vec![
            self.inputs.m.to_string(),
            self.inputs.r.to_string(),
            self.inputs.rho.to_string(),
            self.inputs.mu.to_string(),
            self.a.to_string(),
        ];
        csv::save(path.clone(), head);
        csv::save(path.clone(), v);
    }

    pub fn _save_data(&self) {
        let path = format!("out/{}_data.csv", self.start);

        if fs::exists(path.clone()).unwrap_or_default() {
            // dont save
            return;
        } else {
            dbg!(&path);
        }

        let head = vec![
            "t".to_string(),
            "xx".to_string(),
            "xy".to_string(),
            "xz".to_string(),
            "vx".to_string(),
            "vy".to_string(),
            "vz".to_string(),
            "wx".to_string(),
            "wy".to_string(),
            "wz".to_string(),
        ];
        csv::save(path.clone(), head);

        let n = self.time.len();
        for i in 0..n {
            let v = vec![
                self.time[i].to_string(),
                self.position[i].x.to_string(),
                self.position[i].y.to_string(),
                self.position[i].z.to_string(),
                self.velocity[i].x.to_string(),
                self.velocity[i].y.to_string(),
                self.velocity[i].z.to_string(),
                self.spin[i].x.to_string(),
                self.spin[i].y.to_string(),
                self.spin[i].z.to_string(),
            ];
            csv::save(path.clone(), v);
        }
    }
}

pub fn vx(vclub: f32, theta: f32, clubmass: f32, m: f32) -> f32 {
    let theta_rad = theta.to_radians();
    let e = 0.86 - 0.0029 * vclub * theta_rad.cos();
    (1.0 + e) * vclub * theta_rad.cos() / (1.0 + m / clubmass)
}

pub fn vy(vclub: f32, theta: f32, clubmass: f32, inertia: f32, m: f32, r: f32) -> f32 {
    let theta_rad = theta.to_radians();
    vclub * theta_rad.sin() / (1.0 + m / clubmass + (m * r.powi(2) / inertia))
}

pub fn spin(vclub: f32, theta: f32, clubmass: f32, inertia: f32, m: f32, r: f32) -> f32 {
    let bfp = vy(vclub, theta, clubmass, inertia, m, r);
    m * bfp * r / inertia
}

pub fn reynolds(v: f32, r: f32, mu: f32) -> f32 {
    2.0 * r * v / mu
}

pub fn re_to_cd(re: f32) -> f32 {
    // clamp output value as it is only an approximation
    if re > 120000.0 {
        return 0.370;
    } else if re < 53000.0 {
        return 0.8;
    }

    let mut coeffs = [
        9.926202,
        -3.81337e-04,
        5.72049e-09,
        -3.80737e-14,
        9.4641e-20,
    ];

    coeffs
        .iter()
        .enumerate()
        .map(|(i, &c)| c * re.powi(i as i32))
        .sum()
}

pub fn sphere_cd(v: f32, r: f32, mu: f32) -> f32 {
    let re = reynolds(v, r, mu);
    re_to_cd(re)
}

pub fn simulation(
    time: Res<Time>,
    mut ball_query: Query<(&mut Transform, &mut Ball)>,
    mut outputs: ResMut<state::Outputs>,
    mut next_state: ResMut<NextState<state::AppState>>,
    mut gizmos: Gizmos,
) {
    for (mut transform, mut ball) in ball_query.iter_mut() {
        // get time
        let dt = time.delta_seconds();
        let t = time.elapsed().as_secs_f32();

        // extract from bevy state
        let position = ball.position.last().copied().unwrap_or(Vec3::ZERO);
        let velocity = ball.velocity.last().copied().unwrap_or(Vec3::ZERO);
        let mut spin = ball.spin.last().copied().unwrap_or(Vec3::ZERO);

        // ball tracer
        gizmos.linestrip(ball.position.clone(), BLUE);

        // done sim
        if velocity == Vec3::ZERO {
            return;
        }

        // forces
        let f_d = ball.drag(velocity);
        let f_m = ball.lift(velocity, spin);
        let f_g = ball.gravity();

        // solve for acceleration
        let total_force = f_g + f_d + f_m;
        let acceleration = total_force / ball.inputs.m;

        // double integration using trapezoidal rule
        let mut new_velocity = velocity + acceleration * dt;
        let mut new_position = position + (((velocity + new_velocity) / 2.0) * dt);

        // verify when to stop
        if new_position.y < 0.0 {
            new_position.y = 0.0;
            new_velocity.x = 0.0;
            new_velocity.y = 0.0;
            new_velocity.z = 0.0;
            info!("done sim");
            next_state.set(state::AppState::Waiting);
        }

        // save
        ball.time.push(t);
        ball.position.push(new_position);
        ball.velocity.push(new_velocity);
        ball.acceleration.push(acceleration);
        ball.spin.push(spin);
        ball.f_g.push(f_g);
        ball.f_d.push(f_d);
        ball.f_m.push(f_m);

        // update state
        spin -= (ball.inputs.decel / 100.) * spin;
        outputs.ball = Some(ball.clone()); // TODO make efficient
        transform.translation = new_position;
    }
}
