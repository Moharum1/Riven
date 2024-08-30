
pub trait CoOrdinate{
    // CoOrdinate will be used to handle Shared Operation between Point and Vector

}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum CoOrdinateType{
    #[default]
    Vector,
    Point,
}