#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
//include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
mod led_matrix;

use led_matrix::{LedMatrixOptions, LedMatrix};
use ::std::os::raw::{c_char, c_int};
use std::{thread, time};

fn main() {
    println!("Hello, world!");
    let mut options = LedMatrixOptions::new();

    let mut matrix = LedMatrix::new(options);

    let mut canvas = matrix.canvas();

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
