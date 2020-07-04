mod vector;
use vector::Vector;
use vector::VectorType;
mod ray;
use ray::Ray;


fn main() {
    const IMG_WIDTH : i32 = 256;
    const IMG_HEIGHT : i32 = 256;
    let width_denom : f64 = f64::from(IMG_WIDTH - 1);
    let height_denom : f64 = f64::from(IMG_HEIGHT - 1);
    println!("P3\n{} {}\n255\n", IMG_WIDTH, IMG_HEIGHT);
    for height in (0..IMG_HEIGHT - 1).rev() {
        eprintln!("\rScanlines remaining: {} ", height);
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
