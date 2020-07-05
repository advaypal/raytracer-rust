// To run this: `cargo run > image.ppm` from project root

#[macro_use]
extern crate lazy_static;
mod vector;
use vector::Color;
use vector::Point;
mod ray;
use ray::Ray;
mod hittable;
mod sphere;


/*  =====================
 *  = SCREEN DIMENSIONS =
 *  =====================
 */

// Aspect ratio = width / height
const ASPECT_RATIO: f64 = 16.0 / 9.0;

// Dimensions in pixels
const IMG_HEIGHT: i32 = 216;
const IMG_WIDTH: i32 = ( ASPECT_RATIO * IMG_HEIGHT as f64 ) as i32;

// Dimensions in cartesian values
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;

// Orthogonal distance from origin to the screen
const FOCAL_LENGTH: f64 = 1.0;

/* =================================
 * = VECTOR & COORDINATE CONSTANTS =
 * =================================
 */

// VECTORS
const HORIZONTAL: Point = Point { x: VIEWPORT_WIDTH, y: 0.0, z: 0.0 };
const VERTICAL: Point = Point { x: 0.0, y: VIEWPORT_HEIGHT, z: 0.0 };
const FOCAL_LENGTH_VEC: Point = Point { x: 0.0, y: 0.0, z: FOCAL_LENGTH };

// COORDINATES
const ORIGIN: Point = Point { x: 0.0, y: 0.0, z: 0.0 };

lazy_static! {
  pub static ref LOWER_LEFT_CORNER : Point = ORIGIN
    - FOCAL_LENGTH_VEC
    - HORIZONTAL / 2.0
    - VERTICAL / 2.0;
}

/* ===================
 * = IMAGE CONSTANTS =
 * ===================
 */

// Color values are integers over the range: [0, 255]
const MAX_COLOUR_VALUE : i32 = 255;

/* ========
 * = MAIN =
 * ========
 */

// Write image to stdout
// Uses stderr for progress logs
fn main() {

    // Write ppm headers
    write_ppm_headers();

    // Trace each pixel on the screen
    // Start from the lower left corner
    // Trace left to right, bottom to top row
    for height in (0..IMG_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {} ", height);
        for width in 0..IMG_WIDTH {
            // Fractional components of horizontal, vertical vectors respectively
            let u = width as f64 / (IMG_WIDTH - 1) as f64;
            let v = height as f64 / (IMG_HEIGHT - 1) as f64;

            // Direction vector on the plane of the screen
            let direction = *LOWER_LEFT_CORNER - ORIGIN
                + HORIZONTAL * u
                + VERTICAL * v;

            // Shoot a ray out to the pixel on the screen
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
        let n = (r.at(t) - sphere_center).unit_vector();
        return (n + unit_color) * 0.5;
    }
            
    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    let other_color = Color { x: 0.5, y: 0.7, z: 1.0 };
    unit_color * (1.0 - t) + other_color * t
}

// PPM specifications are here: http://davis.lbl.gov/Manuals/NETPBM/doc/ppm.html
// PPM example: https://en.wikipedia.org/wiki/Netpbm#PPM_example
fn write_ppm_headers() {

    // Write "P3" header for PPM format
    println!("P3");

    // Write image width, height
    println!("{} {}", IMG_WIDTH, IMG_HEIGHT);

    // Write maximum color value
    println!("{}", MAX_COLOUR_VALUE);
}
