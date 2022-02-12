#[cfg(feature = "alloc")]
use alloc::boxed::Box;

use crate::{Map, MapMut};
use super::{MapRows, MapRowsMut};

#[derive(Clone)]
pub struct StaticMap<T, const WIDTH: usize, const HEIGHT: usize> {
    tiles: [[T; WIDTH]; HEIGHT],
}

impl<T, const WIDTH: usize, const HEIGHT: usize> Default for StaticMap<T, WIDTH, HEIGHT>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            tiles: [(); HEIGHT].map(|_| [(); WIDTH].map(|_| T::default())),
        }
    }
}

impl<T, const WIDTH: usize, const HEIGHT: usize> StaticMap<T, WIDTH, HEIGHT>
where
    T: Copy + Default,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn default_copy() -> Self
    where
        T: Copy,
    {
        Self {
            tiles: [[T::default(); WIDTH]; HEIGHT],
        }
    }
}

impl<T, const WIDTH: usize, const HEIGHT: usize> Map for StaticMap<T, WIDTH, HEIGHT> {
    type Tile = T;

    fn get(&self, x: usize, y: usize) -> Option<Self::Tile>
    where
        Self::Tile: Copy,
    {
        self.tiles.get(y).and_then(|row| row.get(x)).copied()
    }

    fn get_ref(&self, x: usize, y: usize) -> Option<&Self::Tile> {
        self.tiles.get(y).and_then(|row| row.get(x))
    }

    fn width(&self) -> usize {
        WIDTH
    }

    fn height(&self) -> usize {
        HEIGHT
    }
}

impl<T, const WIDTH: usize, const HEIGHT: usize> MapMut for StaticMap<T, WIDTH, HEIGHT> {
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Self::Tile> {
        self.tiles.get_mut(y).and_then(|row| row.get_mut(x))
    }

    fn clear(&mut self)
    where
        Self::Tile: Default,
    {
        for tile in self.tiles.iter_mut().flat_map(|r| r.iter_mut()) {
            *tile = Self::Tile::default();
        }
    }

    fn clear_to(&mut self, new: Self::Tile)
    where
        Self::Tile: Clone,
    {
        // Todo: Any performant way to prevent the extraneous clone for the last element in a
        // generic way?
        for tile in self.tiles.iter_mut().flat_map(|r| r.iter_mut()) {
            *tile = new.clone();
        }
    }
}

impl<T, const WIDTH: usize, const HEIGHT: usize> MapRows for StaticMap<T, WIDTH, HEIGHT> {
    fn row(&self, row: usize) -> Option<&[Self::Tile]> {
        self.tiles.get(row).map(|r| r.as_slice())
    }

    #[cfg(feature = "alloc")]
    fn rows(&self) -> Box<dyn DoubleEndedIterator<Item = &[Self::Tile]> + '_> {
        Box::new(self.tiles.iter().map(|r| r.as_slice()))
    }
}

impl<T, const WIDTH: usize, const HEIGHT: usize> MapRowsMut for StaticMap<T, WIDTH, HEIGHT> {
    fn row_mut(&mut self, row: usize) -> Option<&mut [Self::Tile]> {
        self.tiles.get_mut(row).map(|r| r.as_mut_slice())
    }

    #[cfg(feature = "alloc")]
    fn rows_mut(&mut self) -> Box<dyn DoubleEndedIterator<Item = &mut [Self::Tile]> + '_> {
        Box::new(self.tiles.iter_mut().map(|r| r.as_mut_slice()))
    }
}