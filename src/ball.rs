use bevy::prelude::*;

use std::f32::consts::PI;

use crate::constants::{COLOUR_BALL_LINE, G_MS_2};
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
    pub re: Vec<f32>,
    pub c_d: Vec<f32>,
    pub c_m: Vec<f32>,
    pub start: String,
    pub inputs: state::Inputs,
    pub a: f32,
}

impl Ball {
    pub fn new(inputs: &state::Inputs, t: f32) -> Self {
        let start = Self::make_start_id();

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
            re: vec![0.],
            c_d: vec![0.],
            c_m: vec![0.],
        }
    }

    pub fn save_combined_csv(&self) {
        let filename = format!("{}.csv", self.start);
        let mut rows: Vec<Vec<String>> = Vec::new();
        rows.push(vec![
            "m".into(),
            "r".into(),
            "rho".into(),
            "mu".into(),
            "a".into(),
            "".into(),
            "".into(),
            "".into(),
            "".into(),
            "".into(),
            "".into(),
            "".into(),
            "".into(),
            "".into(),
            "".into(),
            "".into(),
            "".into(),
            "".into(),
            "".into(),
        ]);
        rows.push(vec![
            self.inputs.m.to_string(),
            self.inputs.r.to_string(),
            self.inputs.rho.to_string(),
            self.inputs.mu.to_string(),
            self.a.to_string(),
            "".into(),
            "".into(),
            "".into(),
            "".into(),
            "".into(),
            "".into(),
            "".into(),
            "".into(),
            "".into(),
            "".into(),
            "".into(),
            "".into(),
            "".into(),
            "".into(),
        ]);
        rows.push(vec![
            "t".into(),
            "xx".into(),
            "xy".into(),
            "xz".into(),
            "vx".into(),
            "vy".into(),
            "vz".into(),
            "wx".into(),
            "wy".into(),
            "wz".into(),
            "fmx".into(),
            "fmy".into(),
            "fmz".into(),
            "fdx".into(),
            "fdy".into(),
            "fdz".into(),
            "re".to_string(),
            "c_d".to_string(),
            "c_m".to_string(),
        ]);
        for i in 0..self.time.len() {
            rows.push(vec![
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
                self.f_m[i].x.to_string(),
                self.f_m[i].y.to_string(),
                self.f_m[i].z.to_string(),
                self.f_d[i].x.to_string(),
                self.f_d[i].y.to_string(),
                self.f_d[i].z.to_string(),
                self.re[i].to_string(),
                self.c_d[i].to_string(),
                self.c_m[i].to_string(),
            ]);
        }
        csv::download_csv(&filename, rows);
    }

    fn make_start_id() -> String {
        #[cfg(target_arch = "wasm32")]
        {
            // Milliseconds since epoch via JS Date
            let ms = js_sys::Date::now() as u64;
            return ms.to_string();
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            use std::time::{SystemTime, UNIX_EPOCH};
            let ms = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis();
            return ms.to_string();
        }
    }

    pub fn is_start(&self) -> bool {
        self.position.last().unwrap().y == 0. && self.velocity.last().unwrap().length() != 0.
    }

    pub fn log_once(&self, msg: String) -> () {
        if self.is_start() {
            info!("{}", msg);
        }
    }

    pub fn reynolds(&self, v: f32) -> f32 {
        (2.0 * self.inputs.r * v * self.inputs.rho) / self.inputs.mu
    }

    pub fn get_cd(&self, re: f32) -> f32 {
        // Clamp output value as it is only an approximation
        if re > 120000.0 {
            return 0.370;
        } else if re < 55000.0 {
            return 0.8;
        }

        // Array of coefficients
        let mut coeffs = [
            9.4641e-20,
            -3.80737e-14,
            5.72049e-09,
            -3.81337e-04,
            9.926202,
        ];

        // Return value of polynomial approximation
        coeffs.reverse();
        coeffs
            .iter()
            .enumerate()
            .map(|(i, &c)| c * re.powi(i as i32))
            .sum()
    }

    pub fn get_cm(&self, w: f32, v: f32) -> f32 {
        let s = (self.inputs.r * w) / v;
        (-3.25 * s * s) + 1.99 * s
    }

    pub fn magnus(&self, velocity: Vec3, omega: Vec3) -> (f32, Vec3) {
        // optional constant c_m
        let mut c_m = self.inputs.c_m;
        if c_m == 0. {
            self.log_once("using variable c_m".into());
            c_m = self.get_cm(omega.length(), velocity.length());
            if c_m == 0. {
                return (0., Vec3::ZERO);
            }
        } else {
            self.log_once("using constant c_m".into());
        }

        // cross product of angular velocity and linear velocity, for direction of spin
        let oh = omega / omega.length();
        let vh = velocity / velocity.length();
        let rxv = oh.cross(vh);

        // magnitude of spin is considered in coefficient of lift
        let f_m = 0.5 * self.inputs.rho * self.a * c_m * velocity.length().powi(2) * rxv;
        (c_m, f_m)
    }

    pub fn drag(&self, velocity: Vec3, re: f32) -> (f32, Vec3) {
        // optional constant c_d
        let mut c_d = self.inputs.c_d;
        if c_d == 0. {
            self.log_once("using variable c_d".into());
            c_d = self.get_cd(re);
        } else {
            self.log_once("using constant c_d".into());
        }

        let f_d = -0.5 * c_d * self.inputs.rho * self.a * velocity.length() * velocity;
        (c_d, f_d)
    }

    pub fn gravity(&self) -> Vec3 {
        let g = Vec3::new(0., -G_MS_2, 0.);
        self.inputs.m * g
    }
}

pub fn vx(vclub: f32, theta: f32, smash: f32) -> f32 {
    let theta_rad = theta.to_radians();
    let o = vclub * theta_rad.cos() * smash;
    info!("vx {} - smash {} ({} @ {})", o, smash, vclub, theta);
    o
}

pub fn vy(vclub: f32, theta: f32, smash: f32) -> f32 {
    let theta_rad = theta.to_radians();
    let o = vclub * theta_rad.sin() * smash;
    info!("vy {} - smash {} ({} @ {})", o, smash, vclub, theta);
    o
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

        // extract
        let position = ball.position.last().copied().unwrap_or(Vec3::ZERO);
        let velocity = ball.velocity.last().copied().unwrap_or(Vec3::ZERO);
        let mut spin = ball.spin.last().copied().unwrap_or(Vec3::ZERO);

        // follow
        let color = Srgba::hex(COLOUR_BALL_LINE).unwrap();
        gizmos.linestrip(ball.position.clone(), color);

        if velocity == Vec3::ZERO {
            return;
        }

        let re = ball.reynolds(velocity.length());
        let (c_d, f_d) = ball.drag(velocity, re);
        let (c_m, f_m) = ball.magnus(velocity, spin);

        // gravitational force
        let f_g = ball.gravity();

        // total force
        let total_force = f_g + f_d + f_m;

        // calculate acceleration
        let acceleration = total_force / ball.inputs.m;

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
        ball.re.push(re);
        ball.c_d.push(c_d);
        ball.c_m.push(c_m);

        // update state
        spin -= (ball.inputs.decel / 100.) * spin;
        outputs.ball = Some(ball.clone()); // TODO make efficient
        transform.translation = new_position;
    }
}
