use wasm_bindgen::prelude::*;

// Import console.log for debugging
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Macro for easy console.log
macro_rules! console_log {
    ($($t:tt)*) => (log(&format!($($t)*)))
}

#[wasm_bindgen]
pub fn grayscale(data: &mut [u8]) {
    console_log!("Processing {} pixels in WebAssembly", data.len() / 4);
    
    for i in (0..data.len()).step_by(4) {
        let r = data[i] as f32;
        let g = data[i + 1] as f32;
        let b = data[i + 2] as f32;
        
        // Standard grayscale conversion
        let gray = 0.299 * r + 0.587 * g + 0.114 * b;
        let gray_u8 = gray as u8;
        
        data[i] = gray_u8;
        data[i + 1] = gray_u8;
        data[i + 2] = gray_u8;
        // Alpha channel (data[i + 3]) stays the same
    }
}

#[wasm_bindgen]
pub fn blur(data: &mut [u8], width: usize, height: usize) {
    // Create a copy of the original data to read from
    let src_data = data.to_vec();
    
    // Simple box blur implementation
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            for c in 0..3 {  // Process R, G, B channels
                let idx = (y * width + x) * 4 + c;
                
                // 3x3 kernel average
                let mut sum = 0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let nx = (x as isize + dx) as usize;
                        let ny = (y as isize + dy) as usize;
                        sum += src_data[(ny * width + nx) * 4 + c] as u32;
                    }
                }
                
                data[idx] = (sum / 9) as u8;
            }
        }
    }
}

#[wasm_bindgen]
pub fn edge_detection(data: &mut [u8], width: usize, height: usize) {
    // Create a copy for reading while we modify the original
    let src_data = data.to_vec();
    
    // Convert to grayscale first
    for i in (0..src_data.len()).step_by(4) {
        let r = src_data[i] as f32;
        let g = src_data[i + 1] as f32;
        let b = src_data[i + 2] as f32;
        
        let gray = 0.299 * r + 0.587 * g + 0.114 * b;
        
        for c in 0..3 {
            data[i + c] = gray as u8;
        }
    }
    
    // Copy the grayscale data
    let gray_data = data.to_vec();
    
    // Sobel operator for edge detection
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let mut gx = 0;
            let mut gy = 0;
            
            // Apply Sobel operator
            // Top row
            gx += -1 * gray_data[((y-1) * width + (x-1)) * 4] as i32;
            gx +=  0 * gray_data[((y-1) * width + x) * 4] as i32;
            gx +=  1 * gray_data[((y-1) * width + (x+1)) * 4] as i32;
            
            // Middle row
            gx += -2 * gray_data[(y * width + (x-1)) * 4] as i32;
            gx +=  0 * gray_data[(y * width + x) * 4] as i32;
            gx +=  2 * gray_data[(y * width + (x+1)) * 4] as i32;
            
            // Bottom row
            gx += -1 * gray_data[((y+1) * width + (x-1)) * 4] as i32;
            gx +=  0 * gray_data[((y+1) * width + x) * 4] as i32;
            gx +=  1 * gray_data[((y+1) * width + (x+1)) * 4] as i32;
            
            // Top row
            gy += -1 * gray_data[((y-1) * width + (x-1)) * 4] as i32;
            gy += -2 * gray_data[((y-1) * width + x) * 4] as i32;
            gy += -1 * gray_data[((y-1) * width + (x+1)) * 4] as i32;
            
            // Middle row
            gy +=  0 * gray_data[(y * width + (x-1)) * 4] as i32;
            gy +=  0 * gray_data[(y * width + x) * 4] as i32;
            gy +=  0 * gray_data[(y * width + (x+1)) * 4] as i32;
            
            // Bottom row
            gy +=  1 * gray_data[((y+1) * width + (x-1)) * 4] as i32;
            gy +=  2 * gray_data[((y+1) * width + x) * 4] as i32;
            gy +=  1 * gray_data[((y+1) * width + (x+1)) * 4] as i32;
            
            // Calculate magnitude
            let mag = ((gx * gx + gy * gy) as f32).sqrt() as u8;
            
            // Apply threshold
            let idx = (y * width + x) * 4;
            let edge_value = if mag > 100 { 255 } else { 0 };
            
            data[idx] = edge_value;
            data[idx + 1] = edge_value;
            data[idx + 2] = edge_value;
            // Alpha stays the same
        }
    }
}