use core::slice::Split;
use alloc::vec::Vec;

pub type Path<'a> = &'a [u8];
pub const SEPARATOR: u8 = b'/';

pub fn head(path: Path) -> Path {
    path.split(|&b| b == SEPARATOR).last().unwrap_or(&[])
}

pub fn tail(path: Path) -> Path {
    if let Some(index) = path.iter().rposition(|&b| b == SEPARATOR) {
    
    if index < path.len() {
        &path[..index]
    } else {
        &[]
    }
    } else {
        &[]
    }

}

pub fn iter(path: Path) -> PathIter {
    PathIter {
        split: path.split(|&b| b==SEPARATOR).collect(),
        index: 0,
    }
}

pub struct PathIter<'a> {
    split: Vec<Path<'a>>,
    index: usize,
}   

impl<'a> Iterator for PathIter<'a> {
    type Item = Path<'a>;

    fn next(&mut self) -> Option<Self::Item> {

        let mut res = None;

        for i in self.index..self.split.len() {
            let segment = self.split[i];

            if segment.len() > 0 {
                res = Some(segment);
                self.index = i + 1;
                break;
            }
        }
        
        res
    }
}
