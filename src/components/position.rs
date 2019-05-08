use amethyst::ecs::prelude::*;
use rand::prelude::*;
use std::collections::HashSet;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
/// Component that holds the position as logic coordinates on the grid
/// 
/// The logic coordinates system follows Amethyst's window-rendering coord system, 
/// i.e. bottom-left is (0;0).
/// 
/// The grid of a typical 2048 game being 4x4 : 
///     * The minimum position is (0,0)
///     * The maximum position is (3,3)
/// 
pub struct Position {
    x: u32, 
    y: u32,
}

impl Component for Position {
    type Storage = DenseVecStorage<Self>;
}

impl Position {
    /// Creates a new instance of the Position component 
    /// with given attributes 
    #[allow(dead_code)]
    pub fn new(x: u32, y: u32) -> Self {
        Position {
            x,
            y,
        }
    }

    /// Creates a new instance of the Position component 
    /// given all already-used Positions to avoid duplicate
    pub fn new_random<'a, IP: Iterator<Item=&'a Position>>(used_positions: IP) -> Self {
        // Generates all possible positions (from (0,0) to (3,3))
        let mut potential: HashSet<Position> = 
        (0..4)
        .flat_map(|x| {
            (0..4).map(move |y| Position::new(x,y))
        })
        .collect();

        // All used positions are removed base on their Hash
        used_positions.for_each(|p| { potential.remove(p); });

        // Pick some random item from the HashSet
        let idx = random::<usize>() % potential.len();
        potential.into_iter().skip(idx).next().unwrap()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    /// Generating a random position should never panic 
    #[test]
    fn comp_position_random() {
        let positions: Vec<Position> = vec![
            Position::new(0,0),
            Position::new(1,0),
            Position::new(2,1),
            Position::new(0,1),
            Position::new(1,2),
            Position::new(0,2),
            Position::new(1,3),
        ];

        // Generates a bunch of Positions, stores them 
        // in a HashSet to provide Unicity
        let generated = (0..100)
        .map(|_| {
            Position::new_random(positions.iter())
        })
        .collect::<HashSet<Position>>();

        // on 100 generations, it is reasonable to assume that we should have generated 
        // at least 4 different positions
        assert!(generated.len() > 4);

        // And of course, given that we use 7 positions and there are 
        // a maximum of 16 possible positions, this generated number 
        // should be maximum (16-7) = 9 or less
        assert!(generated.len() <= 9); 
    }
}