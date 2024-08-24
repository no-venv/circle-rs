// circle
use std::f32::consts::PI;
use std::str::from_utf8;
const TIME: f32 = 17.0; // height
const WIDTH: f32 = 39.0; // width
const PATTERN: u8 = 0x23;
pub fn main() {
    let mut circle_string_result: String = String::new();
    let mut circle_line_templete = vec![0x20u8; WIDTH as usize];
    let the_circle_pi = PI / TIME;
    let middle_index = (WIDTH / 2.0) as usize;

    for x in 0..(TIME as usize){
        let y = ((WIDTH * (the_circle_pi * x as f32).sin()) / 2.0).floor() as usize;

        circle_line_templete[middle_index..middle_index + y].fill(PATTERN);
        circle_line_templete[middle_index - y..middle_index].fill(PATTERN);

        circle_string_result.push_str(&format!(
            "{}\n",
            from_utf8(&circle_line_templete).unwrap()
        ));
        circle_line_templete.fill(0x20u8);
    }

    println!("{}\n tada!", circle_string_result);
}