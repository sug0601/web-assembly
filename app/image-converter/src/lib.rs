use image::{RgbaImage, Rgba, DynamicImage};
use wasm_bindgen::prelude::*;
use image::imageops::FilterType;
use image::imageops::rotate180;
use base64::{Engine as _, engine::general_purpose};
use std::io::Cursor;
use js_sys::Promise;
use image::GenericImageView;


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


#[wasm_bindgen]
pub fn pixelate_image(base64_data: &str, pixel_size: u32) -> Promise {
    let result = std::panic::catch_unwind(|| {
        let decoded = general_purpose::STANDARD.decode(base64_data)
            .map_err(|_| JsValue::from_str("Base64 decoding error"))?;
        let img = image::load_from_memory(&decoded)
            .map_err(|_| JsValue::from_str("Error loading image"))?;
        
        let (width, height) = img.dimensions();
        let mut pixelated = RgbaImage::new(width, height);
        let img = img.to_rgba8();
        
        for y in (0..height).step_by(pixel_size as usize) {
            for x in (0..width).step_by(pixel_size as usize) {
                let mut sum_r = 0;
                let mut sum_g = 0;
                let mut sum_b = 0;
                let mut sum_a = 0;
                let mut count = 0;
                
                for j in 0..pixel_size {
                    for i in 0..pixel_size {
                        if x + i < width && y + j < height {
                            let pixel = img.get_pixel(x + i, y + j);
                            sum_r += pixel[0] as u32;
                            sum_g += pixel[1] as u32;
                            sum_b += pixel[2] as u32;
                            sum_a += pixel[3] as u32;
                            count += 1;
                        }
                    }
                }
                
                if count > 0 {
                    let avg_pixel = Rgba([
                        (sum_r / count) as u8,
                        (sum_g / count) as u8,
                        (sum_b / count) as u8,
                        (sum_a / count) as u8,
                    ]);
                    
                    for j in 0..pixel_size {
                        for i in 0..pixel_size {
                            if x + i < width && y + j < height {
                                pixelated.put_pixel(x + i, y + j, avg_pixel);
                            }
                        }
                    }
                }
            }
        }
        
        let mut buf = Vec::new();
        let mut cursor = Cursor::new(&mut buf);
        pixelated.write_to(&mut cursor, image::ImageFormat::Png)
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
pub fn blend_images(base64_img1: &str, base64_img2: &str, blend_ratio: f32) -> Promise {
    let result = std::panic::catch_unwind(|| {
        let decoded_img1 = general_purpose::STANDARD.decode(base64_img1)
            .map_err(|_| JsValue::from_str("Base64 decoding error (img1)"))?;
        let decoded_img2 = general_purpose::STANDARD.decode(base64_img2)
            .map_err(|_| JsValue::from_str("Base64 decoding error (img2)"))?;

        let img1 = image::load_from_memory(&decoded_img1)
            .map_err(|_| JsValue::from_str("Error loading image1"))?;
        let img2 = image::load_from_memory(&decoded_img2)
            .map_err(|_| JsValue::from_str("Error loading image2"))?;

        // Resize img2 to match img1 dimensions if they are different
        let (width1, height1) = img1.dimensions();
        let mut img2_resized = img2.clone();
        let (width2, height2) = img2.dimensions();
        if (width1 != width2) || (height1 != height2) {
            img2_resized = img2.resize_exact(width1, height1, FilterType::Lanczos3);
        }

        let (width, height) = img1.dimensions();
        let mut blended = RgbaImage::new(width, height);

        for y in 0..height {
            for x in 0..width {
                let p1 = img1.get_pixel(x, y);
                let p2 = img2_resized.get_pixel(x, y);

                let r = (p1[0] as f32 * blend_ratio + p2[0] as f32 * (1.0 - blend_ratio)) as u8;
                let g = (p1[1] as f32 * blend_ratio + p2[1] as f32 * (1.0 - blend_ratio)) as u8;
                let b = (p1[2] as f32 * blend_ratio + p2[2] as f32 * (1.0 - blend_ratio)) as u8;
                let a = (p1[3] as f32 * blend_ratio + p2[3] as f32 * (1.0 - blend_ratio)) as u8;

                blended.put_pixel(x, y, Rgba([r, g, b, a]));
            }
        }

        let mut buf = Vec::new();
        let mut cursor = Cursor::new(&mut buf);
        DynamicImage::ImageRgba8(blended)
            .write_to(&mut cursor, image::ImageFormat::Png)
            .map_err(|_| JsValue::from_str("Image encoding failed"))?;

        let encoded = general_purpose::STANDARD.encode(&buf);
        Ok(encoded)
    });

    match result {
        Ok(Ok(encoded)) => Promise::resolve(&JsValue::from_str(&encoded)),
        Ok(Err(e)) => Promise::reject(&e),
        Err(_) => Promise::reject(&JsValue::from_str("An error occurred during processing")),
    }
}
