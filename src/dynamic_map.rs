use super::Map;

#[derive(Clone)]
pub struct DynamicMap<T> {
    tiles: Vec<T>,
    width: usize,
}

impl<T> DynamicMap<T> {
    pub fn new(width: usize) -> Self {
        Self {
            tiles: Vec::new(),
            width,
        }
    }

    pub fn with_capacity(width: usize, height: usize) -> Self {
        Self {
            tiles: Vec::with_capacity(width * height),
            width,
        }
    }

    pub fn with_default(width: usize, height: usize) -> Self
    where
        T: Default,
    {
        Self {
            tiles: std::iter::repeat_with(|| T::default())
                .take(width * height)
                .collect(),
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

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Self::Tile> {
        self.tiles.get_mut(x + y * self.width)
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.tiles.len() / self.width
    }
}
