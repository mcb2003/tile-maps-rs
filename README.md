# tile-maps

Helpers for working with 2D maps of tiles common in games.

## Features

-   [x] Convenient API to get and set tiles
-   [x] Use any type that implements `Default` as a tile
-   [x] stack-allocated, fixed-size maps with `StaticMap`.
-   [x] heap-allocated, dynamic maps with `DynamicMap`.
-   [x] Borrow, mutably or immutably, regions of maps

## Goals

-   [ ] Resizable maps
-   [ ] Maps stored as a graph, for easier path-finding
-   [ ] Maps that store tiles in column-major order
-   [ ] Maps composed of chunks
-   [ ] Implement `Index` and `IndexMut` for map types.

## Questions

-   When borrowing a region of a region, should we borrow from the root, parent map, or from the first region?
-   Can we implement `MapRows::rows` and `MapRowsMut::rows_mut` wihtout adding another layer of dynamic dispatch to the
    iterator?
-   Is there any benefit, even from an API standpoint, in creating maps with interior mutability, or locking?

## No STD

This crate doesn't rely on the Rust standard library. However, by default, it does rely on [`alloc`][alloc] for types
that allocate, like `DynamicMap`. Disabling the "alloc" Cargo feature will relax this requirement, and remove any types
that allocate.

[alloc]: <https://docs.rs/alloc>
