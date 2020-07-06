use std::fs::File;
use std::io::prelude::*;
use indicatif::ProgressBar;
mod vec3;
use vec3::Vec3;
use vec3::ToRgb;
use vec3::Ray;
use vec3::{unit_vector, dot};

// TODO: Refactor a lot of the outer code (everything outside of the inner double loops) out into a main function

/**
 * 2. Creating a simple ppm image
 */
pub fn simple_ppm() -> std::io::Result<()> {
    let mut file = File::create("data/simple.ppm")?;
    let image_width = 256;
    let image_height = 256;
    let header = format!("P3\n{} {}\n255\n", image_width, image_height);
    let bar = ProgressBar::new(image_height);

    let mut body: String = String::from("");

    for j in (0..(image_height)).rev() {
        bar.inc(1);
        for i in 0..(image_width) {
            let r = i as f32 / (image_width - 1) as f32 * 255.0;
            let g = j as f32 / (image_height - 1) as f32 * 255.0;
            let b = 0.25 as f32 * 255.0;

            let c = Vec3 { value: [r, g, b] };
            let string = c.to_rgb_with_newline();
            
            body.push_str(&string);
        }
    }
    
    let data = format!("{}{}", header, body);

    file.write_all(data.as_bytes())?;
    bar.finish();
    return Ok(());
}

/**
 * 4. Creating a Ray in a Viewport
 */

pub fn simple_viewport() -> std::io::Result<()> {
    let mut file = File::create("data/viewport.ppm")?;
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 768; // Aspect Ratio: 16/9
    let image_height = 432;
    let header = format!("P3\n{} {}\n255\n", image_width, image_height);

    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = viewport_height * aspect_ratio;
    let focal_length = 1.0;

    let origin = Vec3 {value: [0.0, 0.0, 0.0]};
    let horizontal = Vec3 { value: [viewport_width, 0.0, 0.0] };
    let vertical = Vec3 { value: [0.0, viewport_height, 0.0] };
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3 { value: [0.0, 0.0, focal_length] };

    let bar = ProgressBar::new(image_height);

    let mut body: String = String::from("");

    for j in (0..(image_height)).rev() {
        bar.inc(1);
        for i in 0..(image_width) {
            let x = i as f32 / (image_width - 1) as f32;
            let y = j as f32 / (image_height - 1) as f32;

            let direction = lower_left_corner + x*horizontal + y*vertical - origin;
            let ray = Ray {origin: origin, direction};

            let color = ray_color(&ray) * 255.0;
            
            let string = color.to_rgb_with_newline();
            
            body.push_str(&string);
        }
    }
    
    let data = format!("{}{}", header, body);

    file.write_all(data.as_bytes())?;
    bar.finish();
    return Ok(());
}

/**
 * 5. Creating a Simple Sphere
 */

pub fn simple_sphere() -> std::io::Result<()> {
    let mut file = File::create("data/viewport.ppm")?;
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 768; // Aspect Ratio: 16/9
    let image_height = 432;
    let header = format!("P3\n{} {}\n255\n", image_width, image_height);

    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = viewport_height * aspect_ratio;
    let focal_length = 1.0;

    let origin = Vec3 {value: [0.0, 0.0, 0.0]};
    let horizontal = Vec3 { value: [viewport_width, 0.0, 0.0] };
    let vertical = Vec3 { value: [0.0, viewport_height, 0.0] };
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3 { value: [0.0, 0.0, focal_length] };
    let bar = ProgressBar::new(image_height);

    let mut body: String = String::from("");

    for j in (0..(image_height)).rev() {
        bar.inc(1);
        for i in 0..(image_width) {
            let x = i as f32 / (image_width - 1) as f32;
            let y = j as f32 / (image_height - 1) as f32;

            let direction = lower_left_corner + x*horizontal + y*vertical - origin;
            let ray = Ray {origin: origin, direction};
            // TODO: Refactor to use Rust idiomatic match or if and reuse ray_color
            let color = ray_color_with_sphere(&ray) * 255.0;
            
            let string = color.to_rgb_with_newline();
            
            body.push_str(&string);
        }
    }
    
    let data = format!("{}{}", header, body);

    file.write_all(data.as_bytes())?;
    bar.finish();
    return Ok(());
}

fn hit_sphere(center: &Vec3, radius: f32, ray: &Ray) -> bool {
    let oc = ray.origin - *center;
    let a = dot(&ray.direction, &ray.direction);
    let b = 2.0 * dot(&oc, &ray.direction);
    let c = dot(&oc, &oc) - radius*radius;
    let discriminant = b * b - 4.0 * a * c;
    return discriminant > 0.0;
}

fn ray_color(ray: &Ray) -> Vec3 {
    let unit_direction = unit_vector(&ray.direction);
    let t = 0.5*(unit_direction.value[1] + 1.0);
    return (1.0 - t) * Vec3 {value: [1.0, 1.0, 1.0]} + t * Vec3 {value: [0.5, 0.7, 1.0]};
}

fn ray_color_with_sphere(ray: &Ray) -> Vec3 {
    let sphere_center = Vec3 {value : [0.0, 0.0, -1.0]};
    if hit_sphere(&sphere_center, 0.5, ray) {
        return Vec3 { value: [1.0, 0.0, 0.0] };
    }

    let unit_direction = unit_vector(&ray.direction);
    let t = 0.5 * (unit_direction.value[1] + 1.0);
    return (1.0-t)*Vec3{value: [1.0, 1.0, 1.0]} + t*Vec3 {value: [0.5, 0.7, 1.0]};
}