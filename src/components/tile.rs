use amethyst::ecs::prelude::*;
use rand::prelude::*;

#[derive(Debug, Clone, Default)]
/// The Tile component is a wrapper around the value of the tile entity.
/// 
/// This component's value is defined such as the number on the Tile is `2^(Tile.value + 1)` e.g: 
///     * Tile.value = 0 <==> number = 2^(0 + 1) = 2  
///     * Tile.value = 3 <==> number = 2^(3 + 1) = 16
///     * Tile.value = 10 <==> number = 2^(11 + 1) = 2048 
/// 
/// This is so this value can be used as an index on the Spritesheet.
pub struct Tile {
    /// The value as a power of two
    value: u32,
}

impl Component for Tile {
    type Storage = DenseVecStorage<Self>;
}

impl Tile {
    /// Creates a new instance of the Tile component with a value of zero
    #[allow(dead_code)]
    pub fn new() -> Self {
        Tile {
            value: 0,
        }
    }

    /// Creates a new instance of the Tile component that is either one or zero 
    /// (as the game will sometimes generate a tile that holds 4 instead of 2)
    #[allow(dead_code)]
    pub fn new_random() -> Self {
        let value: u32 = random::<u32>() % 2;
        Tile {
            value,
        }
    }
}