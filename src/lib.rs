#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

mod static_map;
pub use static_map::StaticMap;
#[cfg(feature = "alloc")]
mod dynamic_map;
#[cfg(feature = "alloc")]
pub use dynamic_map::DynamicMap;
mod region;
pub use region::MapRegion;
mod region_mut;
pub use region_mut::MapRegionMut;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

pub trait Map {
    type Tile;

    fn get(&self, x: usize, y: usize) -> Option<Self::Tile>
    where
        Self::Tile: Copy;
    fn get_ref(&self, x: usize, y: usize) -> Option<&Self::Tile>;

    fn width(&self) -> usize;
    fn height(&self) -> usize;

    fn size(&self) -> (usize, usize) {
        (self.width(), self.height())
    }

    fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width() && y < self.height()
    }

    fn region(&self, x: usize, y: usize, width: usize, height: usize) -> Option<MapRegion<Self::Tile, Self>>
    where
        Self: Sized,
    {
        MapRegion::new(self, x, y, width, height)
    }
}

pub trait MapMut: Map {
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Self::Tile>;

    fn region_mut(&mut self, x: usize, y: usize, width: usize, height: usize) -> Option<MapRegionMut<Self::Tile, Self>>
    where
        Self: Sized,
    {
        MapRegionMut::new(self, x, y, width, height)
    }
}

pub trait MapRows: Map {
    fn row(&self, row: usize) -> Option<&[Self::Tile]>;
    #[cfg(feature = "alloc")]
    fn rows(&self) -> Box<dyn DoubleEndedIterator<Item = &[Self::Tile]> + '_>;
}

pub trait MapRowsMut: MapRows {
    fn row_mut(&mut self, row: usize) -> Option<&mut [Self::Tile]>;
    #[cfg(feature = "alloc")]
    fn rows_mut(&mut self) -> Box<dyn DoubleEndedIterator<Item = &mut [Self::Tile]> + '_>;
}
