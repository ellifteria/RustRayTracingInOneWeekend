fn main() {
    let img_width = 256;
    let img_height = 256;

    println!("P3\n{img_width} {img_height}\n255");

    for inv_row in 1..=img_height {
        for col in 1..=img_width {
            let row = img_height - inv_row;

            let ir = 254.999 * (col as f32) / ((img_width) as f32);
            let ig = 254.999 * (row as f32) / ((img_height) as f32);
            let ib = 254.999 * 0.25;

            println!("{ir:.0} {ig:.0} {ib:.0}");
        }
    }
}
