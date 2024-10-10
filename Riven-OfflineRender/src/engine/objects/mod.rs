use crate::engine::base::interval::Interval;
use crate::engine::base::ray::Ray;
use crate::engine::bounding_model::aabb::AABB;
use crate::engine::bounding_model::bvh::BvhNode;
use crate::engine::objects::hit_record::HitRecord;
use crate::engine::objects::object::{GeometricObject, HitList};
use crate::engine::objects::Objects::{BVH, List, Planes, Spheres};
use crate::engine::objects::plane::Plane;
use crate::engine::objects::sphere::Sphere;

pub mod sphere;
pub mod object;
pub mod hit_record;
pub mod plane;

#[derive(Clone)]
pub enum Objects{
    Spheres(Sphere),
    Planes(Plane),
    List(HitList),
    BVH(Box<BvhNode>),
}

impl Objects{
    pub fn hit(&self, ray: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        match self{
            Planes(plane) => plane.hit(ray, ray_t, rec),
            Spheres(s) => s.hit(ray, ray_t, rec),
            List(list) => list.hit(ray, ray_t, rec),
            BVH(BvhNode) => BvhNode.hit(ray, ray_t, rec),
        }
    }

    pub fn bounding_box(&self) -> AABB {
        match self {
            Planes(plane) => plane.bounding_box(),
            Spheres(s) => s.bounding_box(),
            List(list) => list.bounding_box(),
            BVH(BvhNode) => BvhNode.bounding_box()
        }
    }
}
