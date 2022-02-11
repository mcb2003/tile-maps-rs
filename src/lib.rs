mod static_map;
pub use static_map::StaticMap;
mod dynamic_map;
pub use dynamic_map::DynamicMap;

pub trait Map {
    type Tile;
    
    fn get(&self, x: usize, y: usize) -> Option<Self::Tile>
        where
            Self::Tile: Copy;
    fn get_ref(&self, x: usize, y: usize) -> Option<&Self::Tile>;
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Self::Tile>;

    fn width(&self) -> usize;
    fn height(&self) -> usize;

    fn size(&self) -> (usize, usize) {
        (self.width(), self.height())
    }

    fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width() && y < self.height()
    }
}
