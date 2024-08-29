
pub trait CoOrdinate{
    fn new(x: f32, y: f32, z: f32) -> Self;

    // CoOrdinate will be used to handle Shared Operation between Point and Vector

}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CoOrdinateType{
    Vector,
    Point,
}