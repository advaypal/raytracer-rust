use crate::vector;
use vector::Point;
use crate::ray;
use ray::Ray;

pub struct Camera {}

impl Camera {
    /*  =====================
     *  = SCREEN DIMENSIONS =
     *  =====================
     */

    // Aspect ratio = width / height
    const ASPECT_RATIO: f64 = 16.0 / 9.0;

    // Dimensions in cartesian values
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = Camera::ASPECT_RATIO * Camera::VIEWPORT_HEIGHT;

    // Orthogonal distance from origin to the screen
    const FOCAL_LENGTH: f64 = 1.0;


    /* =================================
     * = VECTOR & COORDINATE CONSTANTS =
     * =================================
     */
    
    // VECTORS
    const HORIZONTAL: Point = Point { x: Camera::VIEWPORT_WIDTH, y: 0.0, z: 0.0 };
    const VERTICAL: Point = Point { x: 0.0, y: Camera::VIEWPORT_HEIGHT, z: 0.0 };
    const FOCAL_LENGTH_VEC: Point = Point { x: 0.0, y: 0.0, z: Camera::FOCAL_LENGTH };

    // COORDINATES
    const ORIGIN: Point = Point { x: 0.0, y: 0.0, z: 0.0 };

    // Lazy static doesn't work within structs
    fn lower_left_corner() -> Point {
        Camera::ORIGIN - Camera::FOCAL_LENGTH_VEC - Camera::HORIZONTAL / 2.0
            - Camera::VERTICAL / 2.0
    }
    
    pub fn get_ray(u: f64, v: f64) -> Ray {
        // Direction vector on the plane of the screen
        let direction = Camera::lower_left_corner() - Camera::ORIGIN
            + Camera::HORIZONTAL * u
            + Camera::VERTICAL * v;

        // Shoot a ray out to the pixel on the screen
        Ray { origin: Camera::ORIGIN, direction }
    }
}
