use amethyst::ecs::prelude::*;
use crate::components::{Position, Tile};

pub struct GameTile;

impl GameTile {
    /// Generates a new GameTile with random value (between 0 and 1) and random position.
    /// 
    /// Because a GameTile cannot be generated on a position that's already in use, we have to
    /// avoid generating an impossible case. 
    /// 
    /// The easiest way to do this in this case is to generate a list of all possible positions, and pick one at random. 
    pub fn new_random<'a, IP: Iterator<Item=&'a Position>>(world: &mut World, used_positions: IP) -> Option<Entity> {

        Some(
            world.create_entity()
                .with(Position::new_random(used_positions))
                .with(Tile::new_random())
                .build()
        )
    }
}