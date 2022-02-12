//! A borrowed, rectangular region of a [`Map`].
//!
//! [`MapRegion`] and [`MapRegionMut`] themselves implement [`Map`], and [`MapRegionMut`] also
//! implements [`MapMut`][crate::MapMut]. They act as sub-maps of the parent map.
//!
//! Regions have their own coordinate system: (0, 0) is the top-left corner of the *region*, not of
//! the parent map. The coordinates are translated and passed to the parent map.

mod region;
pub use region::MapRegion;
mod region_mut;
pub use region_mut::MapRegionMut;

use crate::Map;

/// Methods common to both [`MapRegion`] and [`MapRegionMut`].
pub trait Region: Map {
    /// The type of the parent map referenced by this region.
    type Parent: Map;

    /// Get a reference to the parent map.
    ///
    /// It may be more ergonomic to use [`Map::region()`] instead.
    fn map(&self) -> &Self::Parent;

    /// Returns the y coordinate of the top of this region on the parent map.
    fn top(&self) -> usize;
    /// Returns the x coordinate of the left of this region on the parent map.
    fn left(&self) -> usize;

    /// Returns the y coordinate of the bottom of this region on the parent map.
    fn bottom(&self) -> usize {
        self.top() + self.height()
    }

    /// Returns the x coordinate of the right of this region on the parent map.
    fn right(&self) -> usize {
        self.left() + self.width()
    }
}
