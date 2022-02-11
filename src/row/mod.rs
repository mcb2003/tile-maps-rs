mod static_map;
pub use static_map::StaticMap;
#[cfg(feature = "alloc")]
mod dynamic_map;
#[cfg(feature = "alloc")]
pub use dynamic_map::DynamicMap;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

use crate::Map;

/// Methods for working with [`Map`]s stored as contiguous rows.
///
/// For mutable operations on rows, see [`MapRowsMut`].
pub trait MapRows: Map {
    /// Get a slice of tiles representing the row at index `row`. Returns [`None`] if `row` is out
    /// of bounds.
    fn row(&self, row: usize) -> Option<&[Self::Tile]>;
    /// Get a [`DoubleEndedIterator`] of slices representing rows on this map.
    #[cfg(feature = "alloc")]
    fn rows(&self) -> Box<dyn DoubleEndedIterator<Item = &[Self::Tile]> + '_>;
}

/// Methods for mutating [`Map`]s stored as contiguous rows.
///
/// For immutable operations on rows, see [`MapRows`].
pub trait MapRowsMut: MapRows {
    /// Get a mutable slice of tiles representing the row at index `row`. Returns [`None`] if `row`
    /// is out of bounds.
    fn row_mut(&mut self, row: usize) -> Option<&mut [Self::Tile]>;
    /// Get a [`DoubleEndedIterator`] of mutable slices representing rows on this map.
    #[cfg(feature = "alloc")]
    #[cfg(feature = "alloc")]
    fn rows_mut(&mut self) -> Box<dyn DoubleEndedIterator<Item = &mut [Self::Tile]> + '_>;
}
