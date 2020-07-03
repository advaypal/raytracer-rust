mod vector;
fn main() {
    const IMG_WIDTH : i32 = 256;
    const IMG_HEIGHT : i32 = 256;
    let width_denom : f64 = f64::from(IMG_WIDTH - 1);
    let height_denom : f64 = f64::from(IMG_HEIGHT - 1);
    println!("P3\n{} {}\n255\n", IMG_WIDTH, IMG_HEIGHT);
    const B : i32 = (255.99 * 0.25) as i32;
    for height in (0..IMG_HEIGHT - 1).rev() {
        eprintln!("\rScanlines remaining: {} ", height);
        for width in 0..IMG_WIDTH {
            let r : f64 = f64::from(width) / width_denom;
            let g : f64 = f64::from(height) / height_denom;

            let ir : i32 = (r * 255.99) as i32;
            let ig : i32 = (g * 255.99) as i32;

            println!("{} {} {}\n", ir, ig, B);
        }
    }
    eprintln!("\nDone\n");
}
