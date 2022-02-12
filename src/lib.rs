//! Helpers for working with 2D maps of tiles common in games.
//! ## Features
//! * [x] Convenient API to get and set tiles
//! * [x] Use any type that implements [`Default`] as a tile
//! * [x] stack-allocated, fixed-size maps with [`StaticMap`][row::StaticMap].
//! * [x] heap-allocated, dynamic maps with [`DynamicMap`][row::DynamicMap].
//! * [x] Borrow, mutably or immutably, [regions][region] of maps
//! ## Goals
//! * [ ] Resizable maps
//! * [ ] Maps stored as a graph, for easier path-finding
//! * [ ] Maps that store tiles in column-major order
//! * [ ] Maps composed of chunks
//! * [ ] Implement [`Index`][core::ops::Index] and [`IndexMut`][core::ops::IndexMut] for map types.
//! ## Questions
//! * When borrowing a region of a region, should we borrow from the root, parent map, or from the
//! first region?
//! * Can we implement [`MapRows::rows`][row::MapRows::rows()] and
//! [`MapRowsMut::rows_mut`][row::MapRowsMut::rows_mut()] wihtout adding another layer of dynamic
//! dispatch to the iterator?
//! * Is there any benefit, even from an API standpoint, in creating maps with interior mutability,
//! or locking?
//! ## No STD
//! This crate doesn't rely on the Rust standard library. However, by default, it does rely on
//! [`alloc`] for types that allocate, like [`DynamicMap`][row::DynamicMap]. Disabling the "alloc"
//! Cargo feature will relax this requirement, and remove any types that allocate.

#![no_std]
#![warn(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod region;
pub mod row;
pub use region::{MapRegion, MapRegionMut};

/// An abstraction over a 2D array of tiles.
///
/// For mutable operations on maps, see [`MapMut`].
pub trait Map {
    /// The type of each tile, or cell of the grid
    type Tile;

    /// Get a tile at the specified position. Returns [`None`] if the coordinates are out of
    /// bounds.
    fn get(&self, x: usize, y: usize) -> Option<Self::Tile>
    where
        Self::Tile: Copy;
    /// Get a reference to a tile at the specified position. Returns [`None`] if the coordinates
    /// are out of bounds.
    fn get_ref(&self, x: usize, y: usize) -> Option<&Self::Tile>;

    /// Get the width of the map, in tiles.
    fn width(&self) -> usize;
    /// Get the height of the map, in tiles.
    fn height(&self) -> usize;

    /// Get the size of the map (width, height) in tiles.
    fn size(&self) -> (usize, usize) {
        (self.width(), self.height())
    }

    /// Test if the coordinates are in bounds of the map.
    fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width() && y < self.height()
    }

    /// Get a reference to a region of this map.
    ///
    /// The returned [`MapRegion`] also implements `Map`, with its own coordinate system, which it
    /// translates to the parent map's coordinate system
    /// on the fly.
    fn region(
        &self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> Option<MapRegion<Self::Tile, Self>>
    where
        Self: Sized,
    {
        MapRegion::new(self, x, y, width, height)
    }
}

/// An abstraction over a mutable 2D array of tiles.
///
/// For immutable operations on maps, see [`Map`].
pub trait MapMut: Map {
    /// Get a mutable reference to a tile at the specified position. Returns [`None`] if the
    /// coordinates are out of bounds.
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Self::Tile>;

    /// If the coordinates are in bounds of the map, replaces that tile with `new`.
    ///
    /// Returns [`Ok`] with the old value on success, or [`Err`] with the passed value if the
    /// coordinates were out of bounds.
    fn replace(&mut self, x: usize, y: usize, new: Self::Tile) -> Result<Self::Tile, Self::Tile> {
        if let Some(tile) = self.get_mut(x, y) {
            Ok(core::mem::replace(tile, new))
        } else {
            Err(new)
        }
    }

    /// If the coordinates are in bounds of the map, replaces that tile with a default
    /// tile.
    ///
    /// Returns [`Ok`] with the old value on success, or [`Err`] with the default tile if the
    /// coordinates were out of bounds.
    fn replace_default(&mut self, x: usize, y: usize) -> Result<Self::Tile, Self::Tile>
    where
        Self::Tile: Default,
    {
        self.replace(x, y, Self::Tile::default())
    }

    /// If the coordinates are in bounds of the map, sets that tile to `new` and returns [`true`].
    /// Returns [`false`] if the coordinates were out of bounds.
    fn set(&mut self, x: usize, y: usize, new: Self::Tile) -> bool {
        if let Some(tile) = self.get_mut(x, y) {
            *tile = new;
            true
        } else {
            false
        }
    }

    /// If the coordinates are in bounds of the map, sets that tile to the default tile and returns
    /// [`true`]. Returns [`false`] if the coordinates were out of bounds.
    fn set_default(&mut self, x: usize, y: usize) -> bool
    where
        Self::Tile: Default,
    {
        self.set(x, y, Self::Tile::default())
    }

    /// Clears the entire map to the default tile.
    fn clear(&mut self)
    where
        Self::Tile: Default,
    {
        for y in 0..self.height() {
            for x in 0..self.width() {
                self.set_default(x, y);
            }
        }
    }

    /// Clears the entire map to `new`.
    fn clear_to(&mut self, new: Self::Tile)
    where
        Self::Tile: Clone,
    {
        // Todo: Any performant way to prevent the extraneous clone for the last element in a
        // generic way?
        for y in 0..self.height() {
            for x in 0..self.width() {
                self.set(x, y, new.clone());
            }
        }
    }

    /// Get a mutable reference to a region of this map.
    ///
    /// The returned [`MapRegionMut`] also implements [`Map`] and `MapMut`, with its own coordinate
    /// system, which it translates to the parent map's coordinate system on the fly.
    fn region_mut(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> Option<MapRegionMut<Self::Tile, Self>>
    where
        Self: Sized,
    {
        MapRegionMut::new(self, x, y, width, height)
    }
}

/// Commonly used types and traits
pub mod prelude {
    pub use super::{
        region::Region,
        row::{MapRows, MapRowsMut},
        Map, MapMut,
    };
}
