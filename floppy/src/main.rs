use color_processing::*;
use nannou::prelude::*;

const WIDTH: i32 = 512;
const HEIGHT: i32 = 512;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    // get canvas to draw on
    let draw = app.draw();
    // set background to blue
    draw.background().color(BLACK);
    for i in 10..100 {
        let points = gen_coords(
            map_to_window_width(&frame, app.mouse.x, 5.0, 50.0),
            i as f32 * map_to_window_height(&frame, app.mouse.y, -1.0, 1.0),
        );
        draw.polyline().weight(3.0).points_colored(points);
    }
    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}

fn map_to_window_width(frame: &Frame, x: f32, out_min: f32, out_max: f32) -> f32 {
    map_range(
        x,
        -frame.rect().w() / 2.0,
        frame.rect().w() / 2.0,
        out_min,
        out_max,
    )
}

fn map_to_window_height(frame: &Frame, x: f32, out_min: f32, out_max: f32) -> f32 {
    map_range(
        x,
        -frame.rect().h() / 2.0,
        frame.rect().h() / 2.0,
        out_min,
        out_max,
    )
}
fn gen_coords(x_mut: f32, y_mult: f32) -> Vec<(Point2, Rgb<u8>)> {
    let mut coords = vec![];
    let col = Color::new_rgb(0, 0, 255);
    let col = col.brighten(y_mult.abs() as f64 * 0.2);
    for i in -WIDTH * 2..WIDTH * 2 {
        let x = i as f32 / x_mut;
        let y_new = x.sin() * y_mult;
        // let y_new = x.sin() + x.cos() * y_mult;
        // let y_new =  2.0 * ((x + x + 0.5) / 2.0).sin() + (2.0 * x + 5.0) * y_mult;
        // let y_new =  2.0 * ((x + x + 0.5) / 2.0).sin() + (2.0 * x).cos() * y_mult;
        // let y_new =  2.0 * ((x + x + 0.5) / 2.0).sin() + ((x + x + 0.5) / 2.0).cos() * y_mult;
        // let y_new =  2.0 * ((x + x + 0.5) / 2.0).sin() + ((x - x + 0.5) / 2.0).cos() * y_mult;
        // let y_new = x.sin() + x / 10.0 * y_mult;
        // let y_new =  2.0 * ((x + x + 0.5) / 2.0).sin() + (2.0 * x + 5.0).sin() * y_mult;
        // let y_new = 2.0 * ((x + x + 0.5) / 2.0).sin() + ((x * x + 0.5) / 2.0).sin() * y_mult;
        // let y_new = 2.0 * ((x + x * 1.0) / 2.0).sin() * (x + x * 0.025 / 1.0).cos() * y_mult;
        //        let y_new = 2.0 * ((x + x * 1.0) / 2.0).sin() * ((x + (x / 3.0)) / 2.0).cos() * y_mult;
        let y_new = y_new * 10.0;
        coords.push((
            pt2(i as f32 * 2.0, y_new),
            rgb8(col.red, col.green, col.blue),
        ));
    }
    coords
}

fn sine_sum(a: f32, b: f32) -> f32 {
    2.0 * ((a + b) / 2.0).sin() * ((a + b) / 2.0).cos()
}
