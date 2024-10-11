pub mod jetter_sampler;
pub trait Sampler{
    fn get_samples() -> Vec<f32>;
}
