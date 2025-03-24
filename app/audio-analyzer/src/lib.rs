use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn process_frequency_data(data: &[u8]) -> Vec<u8> {
    // Create a new vector to store the processed data
    let mut processed_data = Vec::with_capacity(data.len());
    
    // Apply a simple smoothing algorithm
    for i in 0..data.len() {
        let mut value = data[i] as f32;
        
        // Apply some processing to the frequency data
        // This is a simple example - you could implement more complex
        // signal processing algorithms here
        
        // Simple smoothing with adjacent values
        if i > 0 && i < data.len() - 1 {
            value = (data[i-1] as f32 * 0.25) + (value * 0.5) + (data[i+1] as f32 * 0.25);
        }
        
        // Apply a slight boost to mid frequencies (just as an example)
        let normalized_position = i as f32 / data.len() as f32;
        if normalized_position > 0.2 && normalized_position < 0.8 {
            value *= 1.2;
        }
        
        // Ensure we don't exceed the maximum value
        value = value.min(255.0);
        
        processed_data.push(value as u8);
    }
    
    processed_data
}

// This is called when the WebAssembly module is initialized
#[wasm_bindgen(start)]
pub fn main() {
    // Initialize the WebAssembly module
    // This could set up any required state
    
    // Enable better error messages in debug mode
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
}
