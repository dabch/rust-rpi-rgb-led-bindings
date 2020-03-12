mod led_matrix_c;

pub use led_matrix_c::LedMatrixOptions;
use led_matrix_c as c;
use ::std::os::raw::{c_char, c_int};

pub struct LedMatrix {
    handle: *mut c::LedMatrix,
    options: LedMatrixOptions,
}

pub struct LedCanvas {
    handle: *mut c::LedCanvas,
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

}
