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

// Write image to stdout
// Uses stderr for progress logs
fn main() {
 
    let mut world = HittableList::new();
    let sphere1 = Sphere { center: Point { x: 0.0, y: 0.0, z: -1.0 }, radius: 0.5 };
    let sphere2 = Sphere { center: Point { x: 0.0, y: -100.5, z: -1.0 }, radius: 100.0 };
    world.add(&sphere1);
    world.add(&sphere2);

    // Write ppm headers
    write_ppm_headers();

    // Trace each pixel on the screen
    // Start from the lower left corner
    // Trace left to right, bottom to top row
    
    for height in (0..Camera::IMG_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {} ", height);
        for width in 0..Camera::IMG_WIDTH {
            // Fractional components of horizontal, vertical vectors respectively
            let u = width as f64 / (Camera::IMG_WIDTH - 1) as f64;
            let v = height as f64 / (Camera::IMG_HEIGHT - 1) as f64;
            let r = Camera::get_ray(u, v);
            let pixel_color = ray_color(&r, &world);
            pixel_color.write_color();
        }
    }
    eprintln!("Done");
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
fn write_ppm_headers() {

    // Write "P3" header for PPM format
    println!("P3");

    // Write image width, height
    println!("{} {}", Camera::IMG_WIDTH, Camera::IMG_HEIGHT);

    // Write maximum color value
    println!("{}", Camera::MAX_COLOR_VALUE);
}
