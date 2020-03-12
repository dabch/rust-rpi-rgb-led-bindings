mod led_matrix_c;

pub use led_matrix_c::LedMatrixOptions;
use led_matrix_c as c;
use ::std::os::raw::{c_char, c_int};
use ::std::ffi::CString;

pub struct LedMatrix {
    handle: *mut c::LedMatrix,
    options: LedMatrixOptions,
}

pub struct LedCanvas {
    handle: *mut c::LedCanvas,
}

pub struct LedFont {
    handle: *mut c::LedFont,
}

impl LedMatrixOptions {
    pub fn new() -> LedMatrixOptions {
        LedMatrixOptions {
            hardware_mapping: 0 as *const c_char,
            rows: 32,
            cols: 64,
            chain_length: 0,
            parallel: 0,
            pwm_bits: 0,
            pwm_lsb_nanoseconds: 0,
            pwm_dither_bits: 0,
            brightness: 0,
            scan_mode: 0,
            row_address_type: 0,
            multiplexing: 0,
            led_rgb_sequence: 0 as *const c_char,
            pixel_mapper_config: 0 as  *const c_char,
            panel_type: 0 as  *const c_char,
            _bitfield_1: LedMatrixOptions::new_bitfield_1(0, 0, 0),
            __bindgen_padding_0: [0; 3],
        }
    }
}

impl LedMatrix {
    pub fn new(mut options: LedMatrixOptions) -> LedMatrix {
        let handle = 
            unsafe{
                let ptr = c::led_matrix_create_from_options(&mut options as *mut c::LedMatrixOptions, 0 as *mut c_int, 0 as *mut*mut*mut c_char);
                println!("matrix ptr: {:p}", ptr);
                let mut m = *ptr;
                //println!("matrix ptr II: {:p}", &mut m as *mut c::LedMatrix);
                ptr
            };
        LedMatrix { handle, options }
    }

    pub fn canvas(&self) -> LedCanvas {
        let handle = unsafe {
            println!("starting out, matrix: {:p}...", self.handle);
            let ptr = c::led_matrix_get_canvas(self.handle);
            println!("got ptr: {:p}", ptr);
            ptr
        };
        LedCanvas { handle }
    }

    pub fn offscreen_canvas(&self) -> LedCanvas {
        let handle = unsafe {
            let ptr = c::led_matrix_create_offscreen_canvas(self.handle);
            ptr
        };
        LedCanvas { handle }
    }

    pub fn swap(&self, canvas: LedCanvas) -> LedCanvas {
        let new_handle = unsafe {
            c::led_matrix_swap_on_vsync(self.handle, canvas.handle)
        };
        LedCanvas { handle: new_handle }
    }

    pub fn set_brightness(&self, brightness: u8) {
        unsafe { c::led_matrix_set_brightness(self.handle, brightness); };
    }

    pub fn get_brightness(&self) -> u8 {
        unsafe { c::led_matrix_get_brightness(self.handle) }
    }

}

impl Drop for LedMatrix {
    fn drop(&mut self) {
        unsafe {
            c::led_matrix_delete(self.handle);
        }
    }
}

impl LedCanvas {
    pub fn draw_line(
        &self,
         x0: c_int, y0: c_int, x1: c_int, y1: c_int,
         r: u8, g: u8, b: u8
    ) {
        unsafe {
            c::draw_line(self.handle, x0, y0, x1, y1, r, g, b);
        }
    }

    pub fn draw_text(&self, font: LedFont, x: c_int, y: c_int, r: u8, g: u8, b: u8, txt: String, kerning_offset: i32) -> c_int {
        let txt = CString::new(txt).expect("Failed to convert to CString");
        let txt_ptr = txt.as_ptr();
        unsafe {
            c::draw_text(self.handle, font.handle, x, y, r, g, b, txt_ptr, kerning_offset)
        }
    }
}
