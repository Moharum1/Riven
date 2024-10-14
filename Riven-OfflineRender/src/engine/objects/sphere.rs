use crate::engine::base::constants::constants::PI;
use crate::engine::base::interval::Interval;
use crate::engine::base::point::Point3;
use crate::engine::base::ray::Ray;
use crate::engine::base::vector::Vector3;
use crate::engine::bounding_model::aabb::AABB;
use crate::engine::lighting::diffuse_lighting_model::MaterialType;
use crate::engine::objects::hit_record::HitRecord;
use crate::engine::objects::object::GeometricObject;
use crate::engine::objects::Objects;
use crate::engine::objects::Objects::Spheres;

#[derive(Clone)]
pub struct Sphere{
    center : Point3,
    radius : f32,
    bbox : AABB,
    mat : MaterialType,
}

impl Sphere{
    pub fn new(center: Point3, radius: f32, mat : MaterialType) -> Objects {

        let rvec = Vector3::new(radius, radius, radius);

        Spheres(Self{
            center,
            radius,
            bbox : AABB::from_points(center - rvec, center + rvec),
            mat,
        })
    }

    pub fn normal_at(&self, point : Point3) -> Vector3 {
        return (self.center - point).unit_vector();
    }

    fn get_sphere_uv(&self, point: Vector3) -> (f32, f32) {
        let theta = -point.y.acos();
        let phi = -point.z.atan2(point.x) + PI;

        let u = phi / (2.0 * PI);
        let v = theta / PI;
        (u, v)
    }
}

impl GeometricObject for Sphere {
    fn hit(&self, ray: &Ray, ray_t : &mut Interval, rec: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.len_squared();
        let h = oc.dot(&ray.direction);
        let c = oc.len_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant > 0.0 {
            let sqrt_d = discriminant.sqrt();

            // First root
            let mut root = (-h - sqrt_d) / a;
            if !ray_t.surrounds(root) {
                // Second root
                root = (-h + sqrt_d) / a;
                if !ray_t.surrounds(root){
                    return false;
                }
            }

            rec.t = root;
            rec.point = ray.at(rec.t);
            let outward_normal = (rec.point - self.center) / self.radius;
            let (u, v) = self.get_sphere_uv(outward_normal);

            rec.set_face_normal(ray, outward_normal);
            rec.mat = self.mat.clone();
            rec.v = v;
            rec.u = u;

            return true;
        }

        false
    }

    fn bounding_box(&self) -> AABB {
        return self.bbox.to_owned()
    }
}