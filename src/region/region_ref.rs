#[cfg(feature = "alloc")]
use alloc::boxed::Box;

use super::Region;
use crate::{row::MapRows, Map};

/// An immutable reference to a rectangular region of a [`Map`].
pub struct MapRegion<'a, T, M: Map<Tile = T>> {
    map: &'a M,
    top: usize,
    left: usize,
    width: usize,
    height: usize,
}

impl<'a, T, M: Map<Tile = T>> MapRegion<'a, T, M> {
    /// Create a new `MapRegion` from a parent map.
    ///
    /// Returns [`None`] if any of the coordinates are out of bounds.
    /// # Example
    /// ```
    /// use tile_maps::{row::DynamicMap, region::MapRegion, prelude::*};
    ///
    /// let map = DynamicMap::<i32>::new(10, 10);
    /// let region = MapRegion::new(&map, 1, 2, 4, 3).expect("Coordinates out of bounds");
    ///
    /// assert_eq!(region.left(), 1);
    /// assert_eq!(region.top(), 2);
    /// assert_eq!(region.right(), 1 + 4);
    /// assert_eq!(region.bottom(), 2 + 3);
    ///
    /// // Coordinates are out of bounds
    /// assert!(MapRegion::new(&map, 10, 10, 3, 3).is_none());
    /// ```
    pub fn new(map: &'a M, x: usize, y: usize, width: usize, height: usize) -> Option<Self> {
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
}

impl<'a, T, M: Map<Tile = T>> Region for MapRegion<'a, T, M> {
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

impl<'a, T, M: Map<Tile = T>> Map for MapRegion<'a, T, M> {
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

impl<'a, T, M: MapRows<Tile = T>> MapRows for MapRegion<'a, T, M> {
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
