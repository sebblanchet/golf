use bevy::prelude::*;
use csv::{Terminator, WriterBuilder};
use std::fs::OpenOptions;

#[cfg(target_arch = "wasm32")]
pub fn download_csv(filename: &str, rows: Vec<Vec<String>>) {
    use wasm_bindgen::JsCast;
    use web_sys::{Blob, BlobPropertyBag, HtmlAnchorElement, Url};

    // Build CSV bytes using csv::Writer to ensure proper quoting
    let mut wtr = WriterBuilder::new()
        .terminator(Terminator::CRLF)
        .from_writer(Vec::<u8>::new());
    for row in rows {
        let _ = wtr.write_record(row);
    }
    let _ = wtr.flush();
    let data = wtr.into_inner().unwrap_or_default();

    // Create a Blob from bytes
    let array = js_sys::Array::new();
    array.push(&js_sys::Uint8Array::from(&data[..]));
    let mut props = BlobPropertyBag::new();
    props.type_("text/csv;charset=utf-8");
    let blob = Blob::new_with_u8_array_sequence_and_options(&array, &props).expect("blob creation");

    // Create object URL and click a temporary anchor
    let url = Url::create_object_url_with_blob(&blob).expect("object url");
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            let a = document
                .create_element("a")
                .expect("create a")
                .dyn_into::<HtmlAnchorElement>()
                .expect("into anchor");
            a.set_href(&url);
            a.set_download(filename);
            a.click();
        }
    }
    let _ = Url::revoke_object_url(&url);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn download_csv(path: &str, rows: Vec<Vec<String>>) {
    let r = OpenOptions::new().create(true).append(true).open(path);
    let file = if let Ok(f) = r {
        f
    } else {
        error!("no CSV");
        return;
    };

    let mut wtr = WriterBuilder::new()
        .terminator(Terminator::CRLF)
        .from_writer(file);
    for row in rows {
        let _ = wtr.write_record(row);
    }
    let _ = wtr.flush();
}
