#![allow(dead_code)]

use std::ops::Index;
use std::ops::IndexMut;
use std::fmt;

const BOX_SIZE: usize = 8;
const BOX_LENGHT: usize = 64;

pub struct Unit<T>
where T: Copy + Default
{
    data: [T; BOX_LENGHT],
}

impl<T> Unit<T>
where T: Copy + Default
{
    pub fn new() -> Unit<T> {
        Unit {
            data: [Default::default(); BOX_LENGHT],
        }
    }

    fn size(&self) -> usize {
        BOX_SIZE
    }

    fn len(&self) -> usize {
        BOX_LENGHT
    }

    fn at(&self, x: usize, y: usize) -> T {
        self.data[y * BOX_SIZE + x]
    }

}

impl<T> Index<(usize, usize)> for Unit<T>
where T: Copy + Default
{
    type Output = T;
    fn index(& self, (x, y): (usize, usize)) -> & T {
        &self.data[y * self.size() + x]
    }
}

impl<T> IndexMut<(usize, usize)> for Unit<T>
where T: Copy + Default
{
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut T {
        &mut self.data[y * self.size() + x]
    }
}

impl<'a, T> IntoIterator for &'a Unit<T>
where T: Copy + Default
{
    type Item = T;
    type IntoIter = UnitIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        UnitIterator {
            unit_box: self,
            index: 0,
        }
    }
}

impl<T> fmt::Debug for Unit<T> 
where T: Copy + Default + fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.size() {
            let a = i * self.size();
            let b = (i + 1) * self.size();
            writeln!(f, "{:?}", &self.data[a..b]).unwrap();
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct UnitIterator<'a, T>
where T: Copy + Default
{
    unit_box: &'a Unit<T>,
    index: usize,
}

impl<'a, T> Iterator for UnitIterator<'a, T>
where T: Copy + Default
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            i if i < self.unit_box.len() => {
                self.index += 1;
                Some(self.unit_box.data[i])
            },
            _ => None
        }
    }
}