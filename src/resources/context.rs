#[derive(Clone, Copy, Default)]
pub struct GeneticsContext{
    pub iteration: usize,
    pub iters_per_generation: usize
}

impl GeneticsContext {
    pub fn new() -> Self {
        GeneticsContext{
        iteration: 0,
        iters_per_generation: 8   
        }
    }
}
