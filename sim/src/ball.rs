use bevy::color::palettes::basic::BLUE;
use bevy::prelude::*;

use std::f32::consts::PI;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::csv::save;
use crate::state::{self, Ouputs};

#[derive(Clone, Component)]
pub struct Ball {
    pub time: Vec<f32>,
    pub position: Vec<Vec3>,
    pub velocity: Vec<Vec3>,
    pub acceleration: Vec<Vec3>,
    pub spin: Vec<Vec3>,
    pub start: String,
    pub m: f32,
    pub r: f32,
    pub c_d: f32,
    pub c_m: f32,
    pub rho: f32,
    pub a: f32,
}

impl Ball {
    pub fn new(inputs: &state::Inputs, t: f32) -> Self {
        let start = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
            .to_string();

        let m = inputs.m;
        let r = inputs.r;
        let c_d = inputs.c_d;
        let c_m = inputs.c_m;
        let rho = inputs.rho;
        let a = PI * r * r;

        Self {
            time: vec![t],
            position: vec![inputs.position],
            velocity: vec![inputs.velocity],
            spin: vec![inputs.spin],
            acceleration: vec![Vec3::ZERO],
            start,
            m,
            r,
            c_d,
            c_m,
            rho,
            a,
        }
    }

    pub fn save_params(&self) {
        let path = format!("out/{}_params.csv", self.start);

        let head = vec![
            "m".to_string(),
            "r".to_string(),
            "c_d".to_string(),
            "c_m".to_string(),
            "rho".to_string(),
            "a".to_string(),
        ];
        let v = vec![
            self.m.to_string(),
            self.r.to_string(),
            self.c_d.to_string(),
            self.c_m.to_string(),
            self.rho.to_string(),
            self.a.to_string(),
        ];
        save(path.clone(), head);
        save(path.clone(), v);
    }

    pub fn save_data(&self) {
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
        save(path.clone(), head);

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
            save(path.clone(), v);
        }
    }
}

pub fn simulation(
    time: Res<Time>,
    mut ball_query: Query<(&mut Transform, &mut Ball)>,
    mut outputs: ResMut<Ouputs>,
    mut gizmos: Gizmos,
) {
    for (mut transform, mut ball) in ball_query.iter_mut() {
        // get time
        let dt = time.delta_seconds();
        let t = time.elapsed().as_secs_f32();

        // extract
        let position = ball.position.last().copied().unwrap_or(Vec3::ZERO);
        let velocity = ball.velocity.last().copied().unwrap_or(Vec3::ZERO);
        let mut spin = ball.spin.last().copied().unwrap_or(Vec3::ZERO);

        // follow
        gizmos.linestrip(ball.position.clone(), BLUE);

        if velocity == Vec3::ZERO {
            // stop sim
            ball.save_data();
            return;
        }

        // norm
        let speed = velocity.length();
        let unit_velocity = velocity / speed;

        // drag force
        let f_d = 0.5 * ball.c_d * ball.rho * ball.a * speed * speed * unit_velocity;

        // magnus force
        let s = ball.r * spin.length() / speed;
        let c_m = (-3.25 * s * s) + 1.99 * s;
        let f_m = 0.5 * c_m * ball.rho * ball.a * (spin.cross(velocity));

        // gravitational force
        let g = Vec3::new(0., 9.81, 0.);
        let f_g = ball.m * g;

        // total force
        let total_force = -f_g - f_d + f_m;

        // calculate acceleration
        let acceleration = total_force / ball.m;

        // update velocity using the trapezoidal rule
        let mut new_velocity = velocity + acceleration * dt;

        // update position using the trapezoidal rule
        let mut new_position = position + (((velocity + new_velocity) / 2.0) * dt);

        // verify when to stop
        if new_position.y < 0.0 {
            new_position.y = 0.0;
            new_velocity.x = 0.0;
            new_velocity.y = 0.0;
            new_velocity.z = 0.0;
        }

        // slow the spin
        spin -= 0.01 * spin;

        // save
        ball.time.push(t);
        ball.position.push(new_position);
        ball.velocity.push(new_velocity);
        ball.acceleration.push(acceleration);
        ball.spin.push(spin);

        // update state
        outputs.ball = Some(ball.clone());
        transform.translation = new_position;
    }
}
