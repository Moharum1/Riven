use crate::engine::base::interval::Interval;
use crate::engine::base::point::Point3;
use crate::engine::base::ray::Ray;
use crate::engine::base::vector::Vector3;
use crate::engine::bounding_model::aabb::AABB;
use crate::engine::lighting::diffuse_lighting_model::MaterialType;
use crate::engine::objects::hit_record::HitRecord;
use crate::engine::objects::object::GeometricObject;

struct Quad{
    q : Point3,
    u : Vector3,
    v : Vector3,
    d : f32,
    normal : Vector3,
    mat : MaterialType,
    bbox : AABB
}

impl Quad{
    pub fn new(q : Point3, u : Vector3, v : Vector3, mat : MaterialType) -> Quad {
        let normal = u.cross(&v).unit_vector();
        Self{
            q,
            u,
            v,
            d : normal.dot(&(q - Point3::default())),
            normal,
            mat,
            bbox : Self::set_bounding_box(q , u , v)
        }
    }

    fn set_bounding_box(q : Point3, u : Vector3, v : Vector3) -> AABB {
        let bbox_diagonal11 = AABB::from_points(q, q + u + v);
        let bbox_diagonal12 = AABB::from_points(q + u, q + v);

        AABB::from_aabb(bbox_diagonal11, bbox_diagonal12)
    }
}

impl GeometricObject for Quad{
    fn hit(&self, ray: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        let denom = self.normal.dot(&ray.direction);

        if denom.abs() < 1e-8{
            return false;
        }

        let t = (self.d - self.normal.dot(&(ray.origin - Point3::default()))) / denom;
        if !ray_t.contains(t) {
            return false
        }

        let intersection = ray.at(t);

        rec.t = t;
        rec.point = intersection;
        rec.mat = self.mat.to_owned();
        rec.set_face_normal(ray, self.normal);

        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.to_owned()
    }
}