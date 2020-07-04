// To run this: `cargo run > image.ppm` from project root

mod vector;
use vector::Vector;
use vector::Color;
use vector::Point;
mod ray;
use ray::Ray;


// Write image to stdout
// Uses stderr for progress logs
fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    
    const IMG_WIDTH : i32 = 384;
    let WIDTH_DENOM : f64 = f64::from(IMG_WIDTH - 1);
    
    // const doesn't allow function calls
    let IMG_HEIGHT : i32 = (f64::from(IMG_WIDTH) / ASPECT_RATIO) as i32;
    let HEIGHT_DENOM : f64 = f64::from(IMG_HEIGHT - 1);

    const MAX_COLOUR_VALUE : i32 = 255;

    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f64 = 1.0;

    const ORIGIN: Point = Point { x: 0.0, y: 0.0, z: 0.0 };

    const HORIZONTAL: Point = Point { x: VIEWPORT_WIDTH, y: 0.0, z: 0.0 };

    const VERTICAL: Point = Point { x: 0.0, y: VIEWPORT_HEIGHT, z: 0.0 };

    const FOCAL_LENGTH_VEC: Point = Point { x: 0.0, y: 0.0, z: FOCAL_LENGTH };
    
    // const doesn't allow function calls
    let LOWER_LEFT_CORNER = ORIGIN - HORIZONTAL / 2.0
        - VERTICAL / 2.0 - FOCAL_LENGTH_VEC;

    // PPM specifications are here: http://davis.lbl.gov/Manuals/NETPBM/doc/ppm.html
    // PPM example: https://en.wikipedia.org/wiki/Netpbm#PPM_example

    // Write "P3" header for PPM format
    println!("P3");

    // Write image width, height
    println!("{} {}", IMG_WIDTH, IMG_HEIGHT);

    // Write maximum color value
    println!("{}", MAX_COLOUR_VALUE);

    // Write pixels from top to bottom row
    for height in (0..IMG_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {} ", height);

        // Write the pixels for each row from left to right
        for width in 0..IMG_WIDTH {
            let u = f64::from(width) / WIDTH_DENOM;
            let v = f64::from(height) / HEIGHT_DENOM;
            let direction = LOWER_LEFT_CORNER + HORIZONTAL * u +
                VERTICAL * v - ORIGIN;
            let r = Ray { origin: ORIGIN, direction };
            let pixel_color = ray_color(r);
            pixel_color.write_color();
        }
    }
    eprintln!("Done");
}

fn hit_sphere(center: Point, radius: f64, r: Ray) -> f64 {
    let difference_vector = r.origin - center;
    let a = r.direction.length_squared();
    let b = 2.0 * difference_vector.dot(r.direction);
    let c = difference_vector.length_squared() - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }

}
    

fn ray_color(r: Ray) -> Color {
    let sphere_center = Point { x: 0.0, y: 0.0, z: -1.0 };
    let t = hit_sphere(sphere_center, 0.5, r);
    let unit_color = Color { x: 1.0, y: 1.0, z: 1.0 };
    if t > 0.0 {
        let N = (r.at(t) - sphere_center).unit_vector();
        return (N + unit_color) * 0.5;
    }
            
    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    let other_color = Color { x: 0.5, y: 0.7, z: 1.0 };
    unit_color * (1.0 - t) + other_color * t
}
