use bevy::prelude::*;
use csv::Writer;
use std::fs::OpenOptions;

pub fn save(path: String, vec: Vec<String>) {
    let r = OpenOptions::new().create(true).append(true).open(path);

    // TODO enable wasm
    let file = if let Ok(f) = r {
        f
    } else {
        error!("no CSV");
        return;
    };

    let mut wtr = Writer::from_writer(file);
    wtr.write_record(vec).unwrap_or_default();
}
