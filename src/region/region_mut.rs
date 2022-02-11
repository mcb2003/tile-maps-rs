#[cfg(feature = "alloc")]
use alloc::boxed::Box;

use crate::{Map, MapMut, row::{MapRows, MapRowsMut}};

pub struct MapRegionMut<'a, T, M: Map<Tile = T>> {
    map: &'a mut M,
    top: usize,
    left: usize,
    width: usize,
    height: usize,
}

impl<'a, T, M: Map<Tile = T>> MapRegionMut<'a, T, M> {
    pub fn new(map: &'a mut M, x: usize, y: usize, width: usize, height: usize) -> Option<Self> {
        // Bounds are exclusive
        if map.in_bounds(x + width - 1, y + height - 1) {
            Some(Self {
                map,
                top: y,
                left: x,
                width,
                height,
            })
        } else {
            None
        }
    }

    pub fn map(&self) -> &M {
        self.map
    }

    pub fn map_mut(&mut self) -> &mut M {
        self.map
    }

    pub fn top(&self) -> usize {
        self.top
    }

    pub fn left(&self) -> usize {
        self.left
    }

    pub fn bottom(&self) -> usize {
        self.top + self.height
    }

    pub fn right(&self) -> usize {
        self.left + self.width
    }
}

impl<'a, T, M: Map<Tile = T>> Map for MapRegionMut<'a, T, M> {
    type Tile = T;

    fn get(&self, x: usize, y: usize) -> Option<Self::Tile>
    where
        Self::Tile: Copy,
    {
        if self.in_bounds(x, y) {
            self.map.get(self.left + x, self.top + y)
        } else {
            None
        }
    }

    fn get_ref(&self, x: usize, y: usize) -> Option<&Self::Tile> {
        if self.in_bounds(x, y) {
            self.map.get_ref(self.left + x, self.top + y)
        } else {
            None
        }
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

impl<'a, T, M: MapMut<Tile = T>> MapMut for MapRegionMut<'a, T, M> {
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Self::Tile> {
        if self.in_bounds(x, y) {
            self.map.get_mut(self.left + x, self.top + y)
        } else {
            None
        }
    }
}

impl<'a, T, M: MapRows<Tile = T>> MapRows for MapRegionMut<'a, T, M> {
    fn row(&self, row: usize) -> Option<&[Self::Tile]> {
        self.map
            .row(row)
            .and_then(|r| r.get(self.left()..self.right()))
    }

    #[cfg(feature = "alloc")]
    fn rows(&self) -> Box<dyn DoubleEndedIterator<Item = &[Self::Tile]> + '_> {
        // Todo: Find a way to not allocate another Box?
        Box::new(self.map.rows().map(|r| &r[self.left()..self.right()]))
    }
}

impl<'a, T, M: MapRowsMut<Tile = T>> MapRowsMut for MapRegionMut<'a, T, M> {
    fn row_mut(&mut self, row: usize) -> Option<&mut [Self::Tile]> {
        let left = self.left();
        let right = self.right();
        self.map.row_mut(row).and_then(|r| r.get_mut(left..right))
    }

    #[cfg(feature = "alloc")]
    fn rows_mut(&mut self) -> Box<dyn DoubleEndedIterator<Item = &mut [Self::Tile]> + '_> {
        // Todo: Find a way to not allocate another Box?
        Box::new(
            self.map
                .rows_mut()
                .map(|r| &mut r[self.left..][..self.width]),
        )
    }
}
