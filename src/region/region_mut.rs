#[cfg(feature = "alloc")]
use alloc::boxed::Box;

use super::Region;
use crate::{
    row::{MapRows, MapRowsMut},
    Map, MapMut,
};

/// A mutable reference to a rectangular region of a [`Map`].
pub struct MapRegionMut<'a, T, M: Map<Tile = T>> {
    map: &'a mut M,
    top: usize,
    left: usize,
    width: usize,
    height: usize,
}

impl<'a, T, M: Map<Tile = T>> MapRegionMut<'a, T, M> {
    /// Create a new `MapRegionMut` from a parent map.
    ///
    /// Returns [`None`] if any of the coordinates are out of bounds.
    /// # Example
    /// ```
    /// use tiles::{row::DynamicMap, region::MapRegionMut, prelude::*};
    ///
    /// let mut map = DynamicMap::<i32>::new(10, 10);
    /// let mut region = MapRegionMut::new(&mut map, 1, 2, 4, 3).expect("Coordinates out of bounds");
    ///
    /// assert_eq!(region.left(), 1);
    /// assert_eq!(region.top(), 2);
    /// assert_eq!(region.right(), 1 + 4);
    /// assert_eq!(region.bottom(), 2 + 3);
    ///
    /// // Coordinates are out of bounds
    /// assert!(MapRegionMut::new(&mut map, 10, 10, 3, 3).is_none());
    /// ```
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

    /// Get a mutable reference to the parent map.
    ///
    /// It may be more ergonomic to use [`MapMut::region_mut()`] instead.
    pub fn map_mut(&mut self) -> &mut M {
        self.map
    }
}

impl<'a, T, M: Map<Tile = T>> Region for MapRegionMut<'a, T, M> {
    type Parent = M;

    fn map(&self) -> &Self::Parent {
        self.map
    }

    fn top(&self) -> usize {
        self.top
    }

    fn left(&self) -> usize {
        self.left
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
