use crate::shapes::Hitable;
use crate::vector::Vec3;
use crate::color::Color;
use crate::ray::Ray;
use crate::hit::Hit;

use image::{ImageBuffer, Rgb};

use rand::Rng;

pub struct Scene {
    pub objects: Vec<Box<dyn Hitable>>,
    pub lights: Vec<Vec3>,
}

impl Scene {
    fn hit(&self, ray: Ray) -> Option<Hit> {
        let mut maybe_closest_hit_record: Option<Hit> = None;

        for obj in self.objects.iter() {
            if let Some(new_hit_record) = obj.hit(ray) {
                match &maybe_closest_hit_record {
                    Some(closest_hit_record) => {
                        if closest_hit_record.dist > new_hit_record.dist {
                            maybe_closest_hit_record = Some(new_hit_record);
                        }
                    },
                    None => {
                        maybe_closest_hit_record = Some(new_hit_record);
                    }
                }
            }
        }

        maybe_closest_hit_record
    }

    fn cast_ray(&self, ray: Ray, depth_left: u32) -> Option<Color> {
        if depth_left <= 0 {
            return None;
        }

        let closest_hit_record = self.hit(ray)?;

        let bounced_ray = Ray {
            pos: closest_hit_record.hit_point,
            dir: ray.dir.bounce_with_normal(closest_hit_record.normal),
        };

        let closest_hit_obj_color = closest_hit_record.hit.color(closest_hit_record.hit_point);

        match self.cast_ray(bounced_ray, depth_left - 1) {
            Some(bounced_ray_color) => {
                Some(Color::blend(closest_hit_obj_color, 0.8, bounced_ray_color))
            },
            None => Some(closest_hit_obj_color),
        }
    }

    pub fn render(&self, rendered_dims: (u32, u32), samples: u32, depth: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        assert_ne!(samples, 0);

        let (rendered_width, rendered_height) = rendered_dims;

        let mut result: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(rendered_width, rendered_height);

        for (pixel_x, pixel_y, pixel) in result.enumerate_pixels_mut() {
            let mut pixel_color_sum: (u32, u32, u32) = (0, 0, 0);
            for _ in 0..samples {

                let x = ((pixel_x as f64) - ((rendered_width as f64) / 2.0) + rand::thread_rng().gen_range(0.0, 1.0)) / (rendered_width as f64);
                let y = (((rendered_height as f64) / 2.0) - (pixel_y as f64) + rand::thread_rng().gen_range(0.0, 1.0)) / (rendered_height as f64);

                let ray = Ray {
                    pos: Vec3::new(0.0, 0.0, 0.0),
                    dir: Vec3::new(x, y, -1.0).unit(),
                };


                match self.cast_ray(ray, depth) {
                    Some(Color {r, g, b}) => {
                        pixel_color_sum.0 += r as u32;
                        pixel_color_sum.1 += g as u32;
                        pixel_color_sum.2 += b as u32;
                    },
                    None => {
                        pixel_color_sum.0 += 127;
                        pixel_color_sum.1 += 127;
                        pixel_color_sum.2 += 127;
                    }
                };
            }

            *pixel = image::Rgb([
                (pixel_color_sum.0 / samples) as u8,
                (pixel_color_sum.1 / samples) as u8,
                (pixel_color_sum.2 / samples) as u8,
            ]);
        }

        result
    }
}
