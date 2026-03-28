// default state variables
pub const DEFAULT_M: f32 = 0.04593; // mass of the ball in kg
pub const DEFAULT_R: f32 = 0.04267 / 2.; // radius of the ball in meters
pub const DEFAULT_RHO: f32 = 1.225; // air density in kg/m^3
pub const DEFAULT_MU: f32 = 1.46e-5; // air viscosity at 25 in m^2/s

// conversions
pub const M_TO_YD: f32 = 1.093_613_3;
pub const MS_TO_MPH: f32 = 2.236_936_3;
pub const RADS_TO_RPM: f32 = 60.0 / (2.0 * std::f32::consts::PI);
pub const G_MS_2: f32 = 9.81;

// colours
pub const COLOUR_BALL_LINE: &str = "a51616";
pub const COLOUR_SKY: &str = "66bdff";
pub const COLOUR_GRASS: &str = "00be00";
pub const COLOUR_BALL: &str = "ffffff";

// spacings
pub const UI_LABEL_SPACE: f32 = 6.0;
