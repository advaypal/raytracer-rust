// To run this: `cargo run > image.ppm` from project root

mod vector;
use vector::Vector;
use vector::VectorType;
mod ray;
use ray::Ray;


// Write image to stdout
// Uses stderr for progress logs
fn main() {
    const IMG_WIDTH : i32 = 256;
    let width_denom : f64 = f64::from(IMG_WIDTH - 1);

    const IMG_HEIGHT : i32 = 256;
    let height_denom : f64 = f64::from(IMG_HEIGHT - 1);

    const MAX_COLOUR_VALUE : i32 = 255;

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
        eprintln!("\rScanlines remaining: {} ", height);

        // Write the pixels for each row from left to right
        for width in 0..IMG_WIDTH {
            let pixel_color = Vector {
                vec_type: VectorType::Color,
                x: f64::from(width) / width_denom,
                y: f64::from(height) / height_denom,
                z: 0.25
            };
            pixel_color.write_color();
        }
    }
    eprintln!("\nDone\n");
}

fn ray_color(r: Ray) -> Vector {
    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    let unit_color = Vector {
        vec_type: VectorType::Color,
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    let other_color = Vector {
        vec_type: VectorType::Color,
        x: 0.5,
        y: 0.7,
        z: 1.0,
    };
    unit_color * (1.0 - t) + other_color * t
}
