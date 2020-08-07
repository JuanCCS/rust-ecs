#[derive(Clone, Copy, Default)]
pub struct GeneticsContext{
    pub iteration: u32,
    pub iters_per_generation: u32
}

impl GeneticsContext {
    pub fn new() -> Self {
        GeneticsContext{
        iteration: 0,
        iters_per_generation: 8   
        }
    }
}
