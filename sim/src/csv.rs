use csv::Writer;
use std::fs::OpenOptions;

pub fn save(path: String, vec: Vec<String>) {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap(); // TODO

    let mut wtr = Writer::from_writer(file);
    wtr.write_record(vec).unwrap_or_default();
}
