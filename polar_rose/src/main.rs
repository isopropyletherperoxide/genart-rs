use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    // get canvas to draw on
    let draw = app.draw();
    let mut colored_points: Vec<(Point2, Rgb<u8>)> = vec![]; 
    let gunk = map_range(
        app.mouse.x,
        -(app.window_rect().w() / 2.0),
        app.window_rect().w() / 2.0,
        2.0,
        50.0,
    );
    let gunk2 = map_range(
        app.mouse.y,
        -(app.window_rect().h() / 2.0),
        app.window_rect().h() / 3.0,
        10.0,
        500.0,
    );
    for theta in 0..(3600) {
        let (x, y) = polar_to_cartesian(rose_gen(theta as f32 * 0.1, gunk2, gunk), theta as f32);
        colored_points.push((pt2(x, y), rgb(255, (rose_gen(theta as f32, gunk2 / 2.0, gunk) / 1.0) as u8, 255)));
    }
    draw.background().color(BLACK);
    draw.polyline()
        .weight(0.5)
        .join_round()
        .points_colored(colored_points);
    draw.to_frame(app, &frame).unwrap();
}

fn rose_gen(theta: f32, a: f32, n: f32) -> f32 {
    a * (n * theta).to_radians().sin()
}

fn polar_to_cartesian(r: f32, theta: f32) -> (f32, f32) {
    (r * theta.to_radians().cos(), r * theta.to_radians().sin())
}
