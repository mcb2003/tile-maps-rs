use super::Map;

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

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Self::Tile> {
        self.tiles.get_mut(y).and_then(|row| row.get_mut(x))
    }

    fn width(&self) -> usize {
        WIDTH
    }

    fn height(&self) -> usize {
        HEIGHT
    }
}
