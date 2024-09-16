use crate::engine::lighting::diffuse_lighting_model::material::DiffuseMaterial;

pub mod material;
pub mod lambertian;
pub mod metal;
pub mod dielectric;


pub type AnyMaterial = Box<dyn DiffuseMaterial>;