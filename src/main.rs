// To run this: `cargo run > image.ppm` from project root

#[macro_use]
mod vector;
use vector::Color;
use vector::Point;
mod ray;
use ray::Ray;
mod hittable;
mod sphere;
mod hittablelist;
use hittable::Hittable;
use hittablelist::HittableList;
use sphere::Sphere;
mod camera;
use camera::Camera;
use rand::prelude::*;

// Write image to stdout
// Uses stderr for progress logs
fn main() {
    /* ===================
     * = IMAGE CONSTANTS =
     * ===================
     */
    
    // Aspect ratio = width / height
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    // Dimensions in pixels
    const IMG_HEIGHT: i32 = 216;
    const IMG_WIDTH: i32 = ( ASPECT_RATIO * IMG_HEIGHT as f64 ) as i32;
    
    // Color values are integers over the range: [0, 255]
    const MAX_COLOR_VALUE : i32 = 255;

    const SAMPLES_PER_PIXEL: i32 = 100;

    let mut world = HittableList::new();
    let sphere1 = Sphere { center: Point { x: 0.0, y: 0.0, z: -1.0 }, radius: 0.5 };
    let sphere2 = Sphere { center: Point { x: 0.0, y: -100.5, z: -1.0 }, radius: 100.0 };
    let mut rng = rand::thread_rng();
    world.add(&sphere1);
    world.add(&sphere2);
    
    // Write ppm headers
    write_ppm_headers(IMG_WIDTH, IMG_HEIGHT, MAX_COLOR_VALUE);

    // Trace each pixel on the screen
    // Start from the lower left corner
    // Trace left to right, bottom to top row
    for height in (0..IMG_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {} ", height);
        for width in 0..IMG_WIDTH {
            let mut pixel_color = Color { x: 0.0, y: 0.0, z: 0.0 };
            for _ in 0..SAMPLES_PER_PIXEL {
                // Fractional components of horizontal, vertical vectors respectively
                let (h_rand, w_rand) : (f64, f64) = (rng.gen(), rng.gen());
                let u = ((width as f64) + w_rand) / (IMG_WIDTH - 1) as f64;
                let v = ((height as f64) + h_rand) / (IMG_HEIGHT - 1) as f64;
                let r = Camera::get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }
            write_color(&pixel_color, SAMPLES_PER_PIXEL);
        }
    }
    eprintln!("Done");
}
fn write_color(color: &Color, samples_per_pixel: i32) -> () {
    let scale: f64 = 1.0 / f64::from(samples_per_pixel);
    let (r, g, b) = (color.x * scale, color.y * scale, color.z * scale);
    let r_int: i32 = (256.0 * clamp(r, 0.0, 0.999)) as i32;
    let g_int: i32 = (256.0 * clamp(g, 0.0, 0.999)) as i32;
    let b_int: i32 = (256.0 * clamp(b, 0.0, 0.999)) as i32;
    println!("{} {} {}", r_int, g_int, b_int);
    
}
fn clamp(x: f64, min: f64, max: f64) -> f64{
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}


fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let hit_rec_opt = world.hit(r, 0.0, f64::INFINITY);
    let unit_color = Color { x: 1.0, y: 1.0, z: 1.0 };
    match hit_rec_opt {
        Some(hit_rec) => (hit_rec.normal + unit_color) * 0.5,
        None => {
            let unit_direction = r.direction.unit_vector();
            let t = 0.5 * (unit_direction.y + 1.0);
            let other_color = Color { x: 0.5, y: 0.7, z: 1.0 };
            unit_color * (1.0 - t) + other_color * t            
        }
    }
}

// PPM specifications are here: http://davis.lbl.gov/Manuals/NETPBM/doc/ppm.html
// PPM example: https://en.wikipedia.org/wiki/Netpbm#PPM_example
fn write_ppm_headers(IMG_WIDTH: i32, IMG_HEIGHT: i32, MAX_COLOR_VALUE: i32) {

    // Write "P3" header for PPM format
    println!("P3");

    // Write image width, height
    println!("{} {}", IMG_WIDTH, IMG_HEIGHT);

    // Write maximum color value
    println!("{}", MAX_COLOR_VALUE);
}
