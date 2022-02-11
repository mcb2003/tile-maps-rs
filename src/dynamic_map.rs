#[cfg(feature = "alloc")]
use alloc::{boxed::Box, vec, vec::Vec};

use super::{Map, MapMut, MapRows, MapRowsMut};

#[derive(Clone)]
pub struct DynamicMap<T> {
    tiles: Vec<T>,
    width: usize,
}

impl<T> DynamicMap<T> {
    pub fn new(width: usize, height: usize) -> Self
    where
        T: Default,
    {
        Self {
            tiles: core::iter::repeat_with(|| T::default())
                .take(width * height)
                .collect(),
            width,
        }
    }

    pub fn default_copy(width: usize, height: usize) -> Self
    where
        T: Copy + Default,
    {
        Self {
            tiles: vec![T::default(); width * height],
            width,
        }
    }
}

impl<T> Map for DynamicMap<T> {
    type Tile = T;

    fn get(&self, x: usize, y: usize) -> Option<Self::Tile>
    where
        Self::Tile: Copy,
    {
        self.tiles.get(x + y * self.width).copied()
    }

    fn get_ref(&self, x: usize, y: usize) -> Option<&Self::Tile> {
        self.tiles.get(x + y * self.width)
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.tiles.len() / self.width
    }
}

impl<T> MapMut for DynamicMap<T> {
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Self::Tile> {
        self.tiles.get_mut(x + y * self.width)
    }

fn clear(&mut self)
    where
        Self::Tile: Default {
            for tile in self.tiles.iter_mut() {
                *tile = Self::Tile::default();
            }
}

fn clear_to(&mut self, new: Self::Tile)
    where
        Self::Tile: Clone {
            // Todo: Any performant way to prevent the extraneous clone for the last element in a
            // generic way?
            for tile in self.tiles.iter_mut() {
        *tile = new.clone();
    }
}
}

impl<T> MapRows for DynamicMap<T> {
    fn row(&self, row: usize) -> Option<&[Self::Tile]> {
        self.tiles.get(row * self.width..(row + 1) * self.width)
    }

    #[cfg(feature = "alloc")]
    fn rows(&self) -> Box<dyn DoubleEndedIterator<Item = &[Self::Tile]> + '_> {
        Box::new(self.tiles.chunks(self.width))
    }
}

impl<T> MapRowsMut for DynamicMap<T> {
    fn row_mut(&mut self, row: usize) -> Option<&mut [Self::Tile]> {
        self.tiles.get_mut(row * self.width..(row + 1) * self.width)
    }

    #[cfg(feature = "alloc")]
    fn rows_mut(&mut self) -> Box<dyn DoubleEndedIterator<Item = &mut [Self::Tile]> + '_> {
        Box::new(self.tiles.chunks_mut(self.width))
    }
}
