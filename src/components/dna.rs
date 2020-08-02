use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Dna{
    pub movements : Vec<u32>,
    pub choices : Vec<u32>
}

impl Dna{
    pub fn new(base_dna: &Vec<u32>) -> Self {
        Dna{
            movements: base_dna[..8].to_vec(),
            choices: base_dna[8..].to_vec()
        } 
    }
}
