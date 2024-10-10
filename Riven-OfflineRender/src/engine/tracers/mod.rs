use crate::engine::base::ray::Ray;
use crate::engine::objects::object::HitList;

pub enum TracerTypes{

}

pub trait Tracer{
    fn new_tracer(world : &HitList);
    fn trace_ray(ray : &Ray);
}