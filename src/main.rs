mod vec3;
mod color_tools;

mod include {
    pub use std::io::Write;
    pub use crate::vec3::*;
    pub use crate::color_tools::*;
}

use include::*;

fn main() {
    let img_width = 256;
    let img_height = 256;

    println!("P3\n{img_width} {img_height}\n255");

    for inv_row in 1..=img_height {
        for col in 1..=img_width {
            eprint!("\rLine: {}    ", col);
            let row = img_height - inv_row;

            let ir = (col as f64) / ((img_width) as f64);
            let ig = (row as f64) / ((img_height) as f64);
            let ib = 0.25;

            let curr_color = Vec3::new(ir, ig, ib);
            println!("{}", write_color(curr_color));
            let _ = std::io::stderr().flush();
        }
    }
    eprint!("\n");
}