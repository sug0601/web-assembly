use image::{Rgba};
use wasm_bindgen::prelude::*;
use image::imageops::FilterType;
use image::imageops::rotate180;
use base64::{Engine as _, engine::general_purpose};
use std::io::Cursor;
use js_sys::Promise;

#[wasm_bindgen]
pub fn convert_to_grayscale(base64_data: &str) -> Promise {
    let result = std::panic::catch_unwind(|| {
        let decoded = general_purpose::STANDARD.decode(base64_data)
            .map_err(|_| JsValue::from_str("Base64 decoding error"))?;
        let img = image::load_from_memory(&decoded)
            .map_err(|_| JsValue::from_str("Error loading image"))?;
        
        let gray_img = img.to_luma8();
        let mut buf = Vec::new();
        let mut cursor = Cursor::new(&mut buf);
        gray_img.write_to(&mut cursor, image::ImageFormat::Png)
            .map_err(|_| JsValue::from_str("Image encoding failed"))?;

        let encoded = general_purpose::STANDARD.encode(&buf);
        Ok(encoded)
    });

    let js_promise = match result {
        Ok(Ok(encoded)) => js_sys::Promise::resolve(&JsValue::from_str(&encoded)),
        Ok(Err(e)) => js_sys::Promise::reject(&e),
        Err(_) => js_sys::Promise::reject(&JsValue::from_str("An error occurred during processing")),
    };

    js_promise
}

#[wasm_bindgen]
pub fn resize_image(base64_data: &str, width: u32, height: u32) -> Promise {
    let result = std::panic::catch_unwind(|| {
        let decoded = general_purpose::STANDARD.decode(base64_data)
            .map_err(|_| JsValue::from_str("Base64 decoding error"))?;
        let img = image::load_from_memory(&decoded)
            .map_err(|_| JsValue::from_str("Error loading image"))?;
        
        let resized = img.resize(width, height, FilterType::Lanczos3);
        let mut buf = Vec::new();
        let mut cursor = Cursor::new(&mut buf);
        resized.write_to(&mut cursor, image::ImageFormat::Png)
            .map_err(|_| JsValue::from_str("Image encoding failed"))?;

        let encoded = general_purpose::STANDARD.encode(&buf);
        Ok(encoded)
    });

    let js_promise = match result {
        Ok(Ok(encoded)) => js_sys::Promise::resolve(&JsValue::from_str(&encoded)),
        Ok(Err(e)) => js_sys::Promise::reject(&e),
        Err(_) => js_sys::Promise::reject(&JsValue::from_str("An error occurred during processing")),
    };

    js_promise
}

#[wasm_bindgen]
pub fn rotate_image(base64_data: &str) -> Promise {
    let result = std::panic::catch_unwind(|| {
        let decoded = general_purpose::STANDARD.decode(base64_data)
            .map_err(|_| JsValue::from_str("Base64 decoding error"))?;
        let img = image::load_from_memory(&decoded)
            .map_err(|_| JsValue::from_str("Error loading image"))?;
        
        let rotated = rotate180(&img);
        let mut buf = Vec::new();
        let mut cursor = Cursor::new(&mut buf);
        rotated.write_to(&mut cursor, image::ImageFormat::Png)
            .map_err(|_| JsValue::from_str("Image encoding failed"))?;

        let encoded = general_purpose::STANDARD.encode(&buf);
        Ok(encoded)
    });

    let js_promise = match result {
        Ok(Ok(encoded)) => js_sys::Promise::resolve(&JsValue::from_str(&encoded)),
        Ok(Err(e)) => js_sys::Promise::reject(&e),
        Err(_) => js_sys::Promise::reject(&JsValue::from_str("An error occurred during processing")),
    };

    js_promise
}

#[wasm_bindgen]
pub fn blur_image(base64_data: &str, radius: f32) -> Promise {
    let result = std::panic::catch_unwind(|| {
        let decoded = general_purpose::STANDARD.decode(base64_data)
            .map_err(|_| JsValue::from_str("Base64 decoding error"))?;
        let img = image::load_from_memory(&decoded)
            .map_err(|_| JsValue::from_str("Error loading image"))?;
        
        let blurred = img.blur(radius);
        
        let mut buf = Vec::new();
        let mut cursor = Cursor::new(&mut buf);
        blurred.write_to(&mut cursor, image::ImageFormat::Png)
            .map_err(|_| JsValue::from_str("Image encoding failed"))?;

        let encoded = general_purpose::STANDARD.encode(&buf);
        Ok(encoded)
    });

    let js_promise = match result {
        Ok(Ok(encoded)) => js_sys::Promise::resolve(&JsValue::from_str(&encoded)),
        Ok(Err(e)) => js_sys::Promise::reject(&e),
        Err(_) => js_sys::Promise::reject(&JsValue::from_str("An error occurred during processing")),
    };

    js_promise
}

#[wasm_bindgen]
pub fn clip_image(base64_data: &str, x: u32, y: u32, width: u32, height: u32) -> Promise {
    let result = std::panic::catch_unwind(|| {
        let decoded = general_purpose::STANDARD.decode(base64_data)
            .map_err(|_| JsValue::from_str("Base64 decoding error"))?;
        let img = image::load_from_memory(&decoded)
            .map_err(|_| JsValue::from_str("Error loading image"))?;
        
        let cropped = img.crop_imm(x, y, width, height);
        
        let mut buf = Vec::new();
        let mut cursor = Cursor::new(&mut buf);
        cropped.write_to(&mut cursor, image::ImageFormat::Png)
            .map_err(|_| JsValue::from_str("Image encoding failed"))?;

        let encoded = general_purpose::STANDARD.encode(&buf);
        Ok(encoded)
    });

    let js_promise = match result {
        Ok(Ok(encoded)) => js_sys::Promise::resolve(&JsValue::from_str(&encoded)),
        Ok(Err(e)) => js_sys::Promise::reject(&e),
        Err(_) => js_sys::Promise::reject(&JsValue::from_str("An error occurred during processing")),
    };

    js_promise
}

#[wasm_bindgen]
pub fn invert_colors(base64_data: &str) -> Promise {
    let result = std::panic::catch_unwind(|| {
        let decoded = general_purpose::STANDARD.decode(base64_data)
            .map_err(|_| JsValue::from_str("Base64 decoding error"))?;
        let img = image::load_from_memory(&decoded)
            .map_err(|_| JsValue::from_str("Error loading image"))?;
        
        // Invert the colors for each pixel in the RGBA image
        let mut inverted = img.to_rgba8();
        for pixel in inverted.pixels_mut() {
            let (r, g, b, a) = (pixel[0], pixel[1], pixel[2], pixel[3]);
            *pixel = Rgba([255 - r, 255 - g, 255 - b, a]);
        }
        
        let mut buf = Vec::new();
        let mut cursor = Cursor::new(&mut buf);
        inverted.write_to(&mut cursor, image::ImageFormat::Png)
            .map_err(|_| JsValue::from_str("Image encoding failed"))?;

        let encoded = general_purpose::STANDARD.encode(&buf);
        Ok(encoded)
    });

    let js_promise = match result {
        Ok(Ok(encoded)) => js_sys::Promise::resolve(&JsValue::from_str(&encoded)),
        Ok(Err(e)) => js_sys::Promise::reject(&e),
        Err(_) => js_sys::Promise::reject(&JsValue::from_str("An error occurred during processing")),
    };

    js_promise
}
