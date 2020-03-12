#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
//include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
mod led_matrix;

use led_matrix::{RGBLedMatrixOptions, RGBLedMatrix};
use ::std::os::raw::{c_char, c_int};
use std::{thread, time};

fn main() {
    println!("Hello, world!");
    /*let mut options = RGBLedMatrixOptions {
        hardware_mapping: 0 as *const ::std::os::raw::c_char,
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
        led_rgb_sequence: 0 as *const ::std::os::raw::c_char,
        pixel_mapper_config: 0 as  *const ::std::os::raw::c_char,
        panel_type: 0 as  *const ::std::os::raw::c_char,
        _bitfield_1: RGBLedMatrixOptions::new_bitfield_1(0, 0, 0),
        __bindgen_padding_0: [0; 3],
    };
    */
    let mut options = RGBLedMatrixOptions::new();

    let mut matrix = RGBLedMatrix::from_options(&mut options);

    let mut canvas = RGBLedMatrix::get_canvas(matrix);
    //let mut canvas = matrix.get_canvas();

    println!("done!");

    

    /*unsafe {
        let matrix = led_matrix_create_from_options(&mut options, 0 as *mut c_int, 0 as *mut*mut*mut c_char);
    

        let canvas = led_matrix_get_canvas(matrix);

        led_matrix_set_brightness(matrix, 10);
        draw_line(canvas, 0, 0, 63, 31, 255, 255, 255);
    

        thread::sleep(time::Duration::from_millis(10000));

        led_matrix_set_brightness(matrix, 100);
        draw_line(canvas, 0, 32, 63, 0, 255, 255, 255);
        thread::sleep(time::Duration::from_millis(10000));
    }
    */
    
}
