use std::fmt::Debug;
use std::ops::{IndexMut, Index};
use std::iter::repeat;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Map<Tile: Debug + Clone> {
    pub width: usize,
    height: Option<usize>,
    pub(crate) tiles: Vec<Tile>,
    default_tile: Tile,
}

type Scalar = i32;

impl<Tile: Debug + Clone> Map<Tile> {
    pub fn with_unknown_height(width: usize, default_tile: Tile) -> Self {
        Self {
            width,
            height: None,
            tiles: vec![],
            default_tile,
        }
    }

    pub fn print<F: Fn(&Tile) -> char> (&self, tile_to_char: &F) {
        for (i, tile) in self.tiles.iter().enumerate() {
            print!("{}", tile_to_char(tile));
            if i % self.width ==(self.width-1) {
                println!();
            }
        }
    }

    fn get_index(&self, x: Scalar, y: Scalar) -> usize {
        let qx = x;
        let qy = y;
        if qx >= self.width as Scalar {
            panic!("qx: {}", qx);
        }
        if qx < 0 as Scalar {
            panic!("qx: {}", qx);
        }
        if let Some(height) = self.height {
            if qy >= height as Scalar {
                panic!("qy: {}", qy);
            }
        }
        if qy < 0 as Scalar {
            panic!("qy: {}", qy);
        }
        let index = qx + qy * (self.width as Scalar);
        index as usize
    }

    pub fn is_in_bounds(&self, x: Scalar, y: Scalar) -> bool {
        x >= 0 && y >= 0 && x < self.width as Scalar && y < self.get_height() as Scalar
    }

    pub fn get_height(&self) -> usize{
        if let Some(height) = self.height {
            height
        } else {
            (self.tiles.len()+1) / self.width
        }
    }
}

impl<Tile: Debug + Clone> IndexMut<(Scalar, Scalar)> for Map<Tile> {
    fn index_mut(&mut self, (x, y): (Scalar, Scalar)) -> &mut Self::Output {
        let index = self.get_index(x,y);
        let missing = index as isize +1 - self.tiles.len() as isize;
        if missing > 0 {
            self.tiles.extend(repeat(self.default_tile.clone()).take(missing as usize))
        }
        &mut self.tiles[index as usize]
    }
}

impl<Tile: Debug + Clone> Index<(Scalar, Scalar)> for Map<Tile> {
    type Output = Tile;

    fn index(&self, (x, y): (Scalar, Scalar)) -> &Self::Output {
        let index = self.get_index(x,y);
        &self.tiles[index as usize]
    }
}