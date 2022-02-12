mod region;
pub use region::MapRegion;
mod region_mut;
pub use region_mut::MapRegionMut;

use crate::Map;

pub trait Region: Map {
    type Parent: Map;

    fn map(&self) -> &Self::Parent;

    fn top(&self) -> usize;
    fn left(&self) -> usize;

    fn bottom(&self) -> usize {
        self.top() + self.height()
    }

    fn right(&self) -> usize {
        self.left() + self.width()
    }
}
