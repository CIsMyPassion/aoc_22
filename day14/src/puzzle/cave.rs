use super::*;

pub trait Cave {
    fn drop_sand(&mut self, drop_location: Position) -> bool;
    fn sand_count(&self) -> usize;

    #[cfg(test)]
    fn rock_count(&self) -> usize;
}