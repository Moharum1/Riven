use std::cmp::Ordering;
use crate::engine::base::constants::constants::ranged_random_int;
use crate::engine::base::interval::Interval;
use crate::engine::base::ray::Ray;
use crate::engine::bounding_model::aabb::AABB;
use crate::engine::objects::hit_record::HitRecord;
use crate::engine::objects::object::{GeometricObject, HitList};
use crate::engine::objects::Objects;
use crate::engine::objects::Objects::BVH;

#[derive(Clone)]
pub struct BvhNode{
    left : Objects,
    right : Objects,
    bbox : AABB
}

impl BvhNode{

    pub fn from_world(mut hit_list: HitList) -> Objects {
        BVH(Box::new(
            Self::bvh_new_node(hit_list.objects.clone(), 0, hit_list.objects.len())
        ))
    }

    pub fn bvh_new_node(mut objects: Vec<Objects>, start: usize, end: usize) -> BvhNode {
        let axis = ranged_random_int(0, 3);

        let comparator = match axis {
            0 => |a: &Objects, b: &Objects| {
                Self::box_compare(a, b, 0)
            },

            1 => |a: &Objects, b: &Objects| {
                Self::box_compare(a, b, 1)
            },

            _ => |a: &Objects, b: &Objects| {
                Self::box_compare(a, b, 2)
            },
        };
        let object_span = end - start;


        let (left, right) = if object_span == 1 {
            // If there's only one object, both children point to the same object
            (objects[start].clone(),objects[start].clone())
        } else if object_span == 2 {
            // If there are two objects, the left and right children each take one
            (objects[start].clone() , objects[start + 1].clone())
        } else {
            // Sort objects by bounding box along the specified axis
            objects[start..end].sort_by(comparator);

            // Split the sorted objects in half and create child nodes
            let mid = start + object_span / 2;
            let left = BVH(Box::new(BvhNode::bvh_new_node(objects.clone(), start, mid)));
            let right = BVH(Box::new(BvhNode::bvh_new_node(objects, mid, end)));

            (left, right)
        };


        Self {
            bbox: AABB::from_aabb(left.bounding_box(), right.bounding_box()),
            left,
            right,
        }
    }
    //
    // pub fn box_compare(&self, bbox1 : AABB, bbox2 : AABB, axis : i32) -> bool {
    //     let box1_axis = bbox1.get_axis_interval(axis);
    //     let box2_axis = bbox2.get_axis_interval(axis);
    //
    //     return box1_axis.min < box2_axis.min
    // }
    //
    // fn box_x_compare(&self, bbox1 : AABB, bbox2 : AABB) -> bool {
    //     return self.box_compare(bbox1, bbox2, 0)
    // }
    //
    fn box_compare(bbox1 : &Objects, bbox2 : &Objects, axis : i32) -> Ordering {
        bbox1.bounding_box().get_axis_interval(axis).min
                .partial_cmp(&bbox2.bounding_box().get_axis_interval(axis).min)
                .unwrap()
    }
    //
    // fn box_z_compare(&self, bbox1 : AABB, bbox2 : AABB) -> bool {
    //     return self.box_compare(bbox1, bbox2, 2)
    // }


}

impl GeometricObject for BvhNode{
    fn hit(&self, ray: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {

        if !self.bbox.hit(ray, ray_t) {
            return false
        }

        let hit_left = self.left.hit(ray, ray_t, rec);
        let hit_right = self.right.hit(
            ray,
            &mut Interval::new( ray_t.min, if hit_left { rec.t } else { ray_t.max }),
            rec
        );

        return hit_right || hit_left
    }

    fn bounding_box(&self) -> AABB {
         self.bbox.clone()
    }
}

