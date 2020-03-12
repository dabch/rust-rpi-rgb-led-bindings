mod led_matrix_c;

use led_matrix_c::{RGBLedMatrix, RGBLedMatrixOptions, LedCanvas};
//use led_matrix_c::{RGBLedMatrix, RGBLedMatrixOptions};
use led_matrix_c::{led_matrix_create_from_options, led_matrix_get_canvas};
use ::std::os::raw::{c_char, c_int};

pub struct LedMatrix {
    handle: *mut RGBLedMatrixOptions,
    options: LedMatrixOptions,
}

pub struct Led

impl LedMatrixOptions {
    pub fn new() -> RGBLedMatrixOptions {
        RGBLedMatrixOptions {
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
            _bitfield_1: RGBLedMatrixOptions::new_bitfield_1(0, 0, 0),
            __bindgen_padding_0: [0; 3],
        }
    }
}

impl RGBLedMatrix {
    pub fn new(options: &mut RGBLedMatrixOptions) -> *mut RGBLedMatrix {
        unsafe{
            let ptr = led_matrix_create_from_options(options as *mut RGBLedMatrixOptions, 0 as *mut c_int, 0 as *mut*mut*mut c_char);
            println!("matrix ptr: {:p}", ptr);
            let mut m = *ptr;
            println!("matrix ptr II: {:p}", &mut m as *mut RGBLedMatrix);
            ptr
        }
    }

    pub fn get_canvas(matrix: *mut RGBLedMatrix) -> LedCanvas {
        unsafe {
            println!("starting out, matrix: {:p}...", matrix as *mut RGBLedMatrix);
            let ptr = led_matrix_get_canvas(matrix as *mut RGBLedMatrix);
            println!("got ptr: {:p}", ptr);
            *ptr
        }
    }
}
