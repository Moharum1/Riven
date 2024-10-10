use crate::engine::base::interval::Interval;
use crate::engine::base::point::Point3;
use crate::engine::base::ray::Ray;
use crate::engine::base::vector::Vector3;
use crate::engine::bounding_model::aabb::AABB;
use crate::engine::lighting::diffuse_lighting_model::MaterialType;
use crate::engine::objects::hit_record::HitRecord;
use crate::engine::objects::object::GeometricObject;
use crate::engine::objects::Objects;
use crate::engine::objects::Objects::Planes;

#[derive(Clone)]
pub struct Plane{
    point  : Point3, // Point through which the plane passes
    normal : Vector3, // The plane Normal
    kepsilon: f32,
    mat : MaterialType
}

impl Plane{
    pub fn new(point : Point3, normal : Vector3, mat : MaterialType) -> Objects {
        Planes(Self{
            point,
            normal,
            kepsilon : 0.0,
            mat
        })
    }
}

impl GeometricObject for Plane{
    fn hit(&self, ray: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {

        let denominator = ray.direction.dot(&self.normal);
        if denominator.abs() < 1e-8 {
            return false; // Ray is parallel to the plane, no hit
        }
        let t: f32 = (self.point - ray.origin).dot(&self.normal) / denominator;

        if t > self.kepsilon{
            rec.t = t;
            rec.normal = self.normal;
            rec.point = ray.origin + (t * ray.direction);
            rec.mat = self.mat.clone();

            true
        }else {
            false
        }
    }

    fn bounding_box(&self) -> AABB {
        return AABB::default()
    }
}

#[cfg(test)]
mod test_plane {
    use crate::engine::base::constants::constants;
    use crate::engine::base::interval::Interval;
    use crate::engine::base::point::Point3;
    use crate::engine::base::ray::Ray;
    use crate::engine::base::vector::Vector3;
    use crate::engine::lighting::diffuse_lighting_model::lambertian::Lambertian;
    use crate::engine::objects::hit_record::HitRecord;
    use crate::engine::objects::plane::Plane;

    #[test]
    fn plane_intersect_a_ray(){
        let mat = Lambertian::new(1.0, 0.0, 0.0);
        let plane = Plane::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0,0.0), mat);

        let ray = Ray::new(Point3::new(0.0, 1.0, 0.0), Vector3::new(0.0, -1.0, 0.0));
        let mut rec = HitRecord::default();
        let mut interval = Interval::new(0.0001f32, constants::INFINITY);

        println!("{}", plane.hit(&ray, &mut interval, &mut rec))
    }
}