pub mod backend;
pub mod chart;
pub mod charts;
pub mod window;

use std::ops::Range;

pub fn mult_range(range: Range<f32>, mult: f32) -> Range<f32> {
    let delta = range.end - range.start;

    let half_delta = delta / 2.0;

    let midpoint = range.end - half_delta;

    let adjusted_delta = half_delta * mult;

    let start = midpoint - adjusted_delta;
    let end = midpoint + adjusted_delta;

    Range { start, end }
}
