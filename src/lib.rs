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

    fn replace(&mut self, x: usize, y: usize, new: Self::Tile) -> Result<Self::Tile, Self::Tile> {
        if let Some(tile) = self.get_mut(x, y) {
            Ok(core::mem::replace(tile, new))
        } else {
            Err(new)
        }
    }

    fn replace_default(&mut self, x: usize, y: usize) -> Result<Self::Tile, Self::Tile>
    where
    Self::Tile: Default {
        self.replace(x, y, Self::Tile::default())
    }
    
    fn set(&mut self, x: usize, y: usize, new: Self::Tile) -> bool {
        if let Some(tile) = self.get_mut(x, y) {
            *tile = new;
            true
        } else {
            false
        }
    }
    
    fn set_default(&mut self, x: usize, y: usize) -> bool
    where
    Self::Tile: Default {
        self.set(x, y, Self::Tile::default())
    }

fn clear(&mut self)
    where
        Self::Tile: Default {
    for y in 0..self.height() {
    for x in 0..self.width() {
        self.set_default(x, y);
    }
    }
}

fn clear_to(&mut self, new: Self::Tile)
    where
        Self::Tile: Clone {
            // Todo: Any performant way to prevent the extraneous clone for the last element in a
            // generic way?
    for y in 0..self.height() {
    for x in 0..self.width() {
        self.set(x, y, new.clone());
    }
    }
}

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

pub mod prelude {
    pub use super::{Map, MapMut, MapRows, MapRowsMut};
}
