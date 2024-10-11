use crate::engine::sampler::Sampler;

struct JitterSampler{

}

impl Sampler for JitterSampler{
    fn get_samples() -> Vec<f32> {
        Vec::new()
    }
}