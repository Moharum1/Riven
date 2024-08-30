use std::process::Output;
use crate::engine::base::point::Point3;
use crate::engine::base::ray::Ray;
use crate::engine::objects::object::{HitRecord, Object};

pub struct Sphere{
    center : Point3,
    radius : f32,
}

impl Sphere{
    pub fn new(center: Point3, radius: f32) -> Self{
        Self{
            center,
            radius,
        }
    }

    // pub fn hit(&self, ray: &Ray) -> f32 {
    //     let oc = self.center - ray.origin;
    //     let a = ray.direction.len_squared();
    //     let h = ray.direction.dot(&oc);
    //     let c = oc.len_squared() - self.radius * self.radius;
    //     let discriminant = h * h - a * c;
    //
    //     return if discriminant < 0f32 {
    //         -1f32
    //     } else {
    //         (h - discriminant.sqrt()) / a
    //     }
    // }
}

impl Object for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.len_squared();
        let h = oc.dot(&ray.direction);
        let c = oc.len_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant > 0.0 {
            let sqrt_d = discriminant.sqrt();

            // First root
            let mut root = (-h - sqrt_d) / a;
            if root < t_min || root > t_max {
                // Second root
                root = (-h + sqrt_d) / a;
                if root < t_min || root > t_max {
                    return false;
                }
            }

            rec.t = root;
            rec.point = ray.at(rec.t);
            let outward_normal = (rec.point - self.center) / self.radius;
            rec.set_face_normal(ray, outward_normal);
            return true;
        }

        false
    }
}