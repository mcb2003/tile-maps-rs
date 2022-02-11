use super::Map;

pub struct MapRegion<'a, T, M: Map<Tile = T>> {
    map: &'a M,
    top: usize,
    left: usize,
    width: usize,
    height: usize,
}

impl<'a, T, M: Map<Tile = T>> MapRegion<'a, T, M> {
    pub fn new(map: &'a M, x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            map,
            top: y,
            left: x,
            width,
            height,
        }
    }

    pub fn map(&self) -> &M {
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

impl<'a, T, M: Map<Tile = T>> Map for MapRegion<'a, T, M> {
    type Tile = T;

    fn get(&self, x: usize, y: usize) -> Option<Self::Tile>
    where
    Self::Tile: Copy {
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
