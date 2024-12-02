use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;
use base64::Engine;
use image::load_from_memory;
use image::ImageFormat::Png;
use std::io::Cursor;
#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String{
    log(&"image been uploaded".into());
    let base64_to_vector = base64::prelude::BASE64_STANDARD.decode(encoded_file).unwrap();
    log(&"image been Decoded".into());
    let mut img = load_from_memory(&base64_to_vector).unwrap();
    log(&"image been Loaded".into());
    img = img.grayscale();
    log(&"image been grayscaled".into());
    let mut buffer: Vec<u8> = Vec::new();
    let mut writer = Cursor::new(&mut buffer);
    img.write_to(&mut writer, Png).unwrap();
    log(&"New image been Written".into());
    let encoded_img = base64::prelude::BASE64_STANDARD.encode(&buffer);
    let data_url = format!("data:image/png;base64,{}", encoded_img);
    return data_url
}