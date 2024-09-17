use std::rc::Rc;
use std::sync::Arc;
use crate::engine::objects::object::Object;

pub mod sphere;
pub mod object;

pub type AnyObject = Box<dyn Object>;
