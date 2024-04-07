use std::ops::{Index, IndexMut};

use bevy::prelude::*;

/// Get the array index for the given position. This is the inverse operation
/// to get_2d_from_1d.
/// 
/// # Examples
/// ```
/// use bevy_flat_arrays::prelude::tools::get_1d_from_2d;
/// let i = get_1d_from_2d(2, 1, 1);
/// assert_eq!(i, 3);
/// ```
pub fn get_1d_from_2d(width: usize, x: usize, y: usize) -> usize {
    width * x + y
}

/// Get the position from an index. This is the inverse operation
/// to get_1d_from_2d.
/// 
/// # Example
/// ```
/// use bevy_flat_arrays::prelude::tools::get_1d_from_2d;
/// use bevy_flat_arrays::prelude::tools::get_2d_from_1d;
/// let x = 1;
/// let y = 1;
/// let width = 2;
/// let i = get_1d_from_2d(width, x, y);
/// let (x1, y1) = get_2d_from_1d(width, i);
/// assert_eq!(x, x1);
/// assert_eq!(y, y1);
/// ```
pub fn get_2d_from_1d(width: usize, i: usize) -> (usize, usize) {
    (i / width, i % width)
}

/// Returns the array index for the given vector.
/// This is a wrapper around get_1d_from_2d. 
/// 
/// # Example
/// ```
/// use bevy::prelude::*;
/// use bevy_flat_arrays::prelude::tools::get_1d_from_2d_ivec2;
/// use bevy_flat_arrays::prelude::tools::get_2d_from_1d_ivec2;
/// let v = IVec2 { x : 1, y : 1 };
/// let width = 2;
/// let i = get_1d_from_2d_ivec2(width, v);
/// let v1 = get_2d_from_1d_ivec2(width, i);
/// assert_eq!(v, v1);
/// ```
pub fn get_1d_from_2d_ivec2(width: usize, v: IVec2) -> usize {
    get_1d_from_2d(width, v.x as usize, v.y as usize)
}

/// Return the position from a array index. This is a wrapper around get_2d_from_1d.
/// # Example
/// ```
/// use bevy::prelude::*;
/// use bevy_flat_arrays::prelude::tools::get_1d_from_2d_ivec2;
/// use bevy_flat_arrays::prelude::tools::get_2d_from_1d_ivec2;
/// let v = IVec2 { x : 1, y : 2 };
/// let width = 3;
/// let i = get_1d_from_2d_ivec2(width, v);
/// let v1 = get_2d_from_1d_ivec2(width, i);
/// assert_eq!(v, v1);
/// ```
pub fn get_2d_from_1d_ivec2(width: usize, i: usize) -> IVec2 {
    let (x, y) = get_2d_from_1d(width, i);
    IVec2 {
        x: x as i32,
        y: y as i32,
    }
}

/// Map a world vector to a position on a predefined grid. Think
/// of an inventory hud with its tiles arranged in a grid. If the
/// layout of an 2d array matches the grid of this inventory we can
/// calculate the index of a tile from its screen position using this
/// function.
/// ```
/// use bevy::prelude::*;
/// use bevy_flat_arrays::prelude::tools::quantize_to_grid;
/// let v = Vec2 { x : 35.8277, y : 7.987278, };
/// let grid_size = 4.0;
/// let mapped = quantize_to_grid(v, grid_size);
/// assert_eq!(Vec2 { x : 32.0, y : 4.0 }, mapped); 
/// 
/// ```
pub fn map_to_grid_vec2(v : Vec2, grid_size : f32) -> IVec2 {
    let quantized = quantize_to_grid(v, grid_size);

    IVec2 {
        x : (quantized.x / grid_size) as i32,
        y : (quantized.y / grid_size) as i32,
    }
}

pub fn quantize_to_grid(v : Vec2, grid_size : f32) -> Vec2 {
    let x = (v.x / grid_size).floor() * grid_size;
    let y = (v.y / grid_size).floor() * grid_size;

    Vec2 { x, y, }
}

/// # Array2d
/// 
/// This array creates a 2 dimensional array that keeps its data in a cache friendly way.
/// This should reduce cache misses while iterating the array and reduce the number of 
/// indirections. This should result in an increase in performance when iterating
/// over the data.
/// 
/// # Traits and behaviour
/// 
/// Both a immutable and an mutable iterator are provided. However, both iterators
/// always map an index to a position and return the position as an ivec2 along with the
/// actual value. This means there is always a little computational overhead. To avoid this
/// overhead the index trait has been implemented. If one accesses the data via index, 
/// no additional comnputation takes place.
/// 
/// The memory for the array is allocated when a new array is created and can be resized
/// using the resize function. To make it easier to allocate memory, all types are required
/// to implement the Default trait. 
pub struct Array2d<T: std::default::Default> {
    width: usize,
    height: usize,
    array: Vec<T>,
}

impl<T: std::default::Default> Array2d<T> {
    /// Constructs a new array.
    pub fn new(width: usize, height: usize) -> Self {
        assert!(width > 0);
        assert!(height > 0);
        let mut r: Vec<T> = Vec::new();
        r.resize_with(width * height, || T::default());

        Array2d {
            width,
            height,
            array: r,
        }
    }


    /// Get the value for the given position.
    pub fn get(&self, v : IVec2) -> &T {
        let i = get_1d_from_2d_ivec2(self.width, v);
        assert!(i < self.len(), "Invalid index");
        &self.array[i]
    }

    /// Get a mutable reference for the given position.
    pub fn get_mut(&mut self, v : IVec2) -> &mut T {
        let i = get_1d_from_2d_ivec2(self.width, v);
        assert!(i < self.len(), "Invalid index");
        &mut self.array[i]
    }

    /// Update the value for the given position.
    pub fn set(&mut self, v : IVec2, value : T) {
        let i = get_1d_from_2d_ivec2(self.width, v);
        assert!(i < self.len(), "Invalid index");
        self.array[i] = value;
    }

    /// Resize this array to the given dimensions. Allocates 
    /// the needed memory right away.
    pub fn resize(&mut self, width : usize, heigth : usize) {
        self.height = heigth;
        self.width = width;
        self.array.resize_with(width * heigth, || T::default());
    }
    
    /// Returns the number of items inside this array holds.
    pub fn len(&self) -> usize {
        self.width * self.height
    }

    /// Implemented to silence the compiler. Always return false.
    pub fn is_empty(&self) -> bool {
        false
    }

    /// Creates a new immutable iterator.
    pub fn iter(&self) -> Array2dIter<'_, T> {
        Array2dIter {
            items: &self.array,
            cursor: 0,
            max: self.len(),
            width: self.width,
        }
    }

    /// Creates a new mutable iterator.
    fn iter_mut(&mut self) -> Array2dMutIter<'_, T> {
        let len = self.len();

        Array2dMutIter {
            items: &mut self.array,
            cursor: 0,
            max: len,
            width: self.width,
        }
    }
}

impl<T: std::default::Default> Index<usize> for Array2d<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.len());
        &self.array[index]
    }
}

impl<T: std::default::Default> IndexMut<usize> for Array2d<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        assert!(index < self.len());
        &mut self.array[index]
    }
}

pub struct Array2dIter<'a, T: std::default::Default> {
    items: &'a Vec<T>,
    cursor: usize,
    max: usize,
    width: usize,
}

impl<'a, T: std::default::Default> Iterator for Array2dIter<'a, T> {
    type Item = (IVec2, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let tmp = self.cursor;
        if tmp >= self.max {
            return None;
        }

        self.cursor += 1;
        let v = get_2d_from_1d_ivec2(self.width, tmp);

        Some((v, &self.items[tmp]))
    }
}

impl<'a, T: std::default::Default> IntoIterator for &'a Array2d<T> {
    type Item = (IVec2, &'a T);

    type IntoIter = Array2dIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Array2dMutIter<'a, T: std::default::Default> {
    items: &'a mut Vec<T>,
    cursor: usize,
    max: usize,
    width: usize,
}

impl<'a, T: std::default::Default> Iterator for Array2dMutIter<'a, T> {
    type Item = (IVec2, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        let tmp = self.cursor;
        self.cursor += 1;
        if tmp >= self.max {
            return None;
        }

        let v = get_2d_from_1d_ivec2(self.width, self.cursor);

        let pt = self.items.as_mut_ptr();
        unsafe { Some((v, &mut *pt)) }
    }
}

impl<'a, T: std::default::Default> IntoIterator for &'a mut Array2d<T> {
    type Item = (IVec2, &'a mut T);

    type IntoIter = Array2dMutIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_data_2d() -> Vec<(usize, usize, usize)> {
        vec![
            (4, 0, 0),
            (4, 1, 0),
            (4, 1, 1),
            (4, 2, 1),
            (4, 3, 1),
            (4, 1, 2),
            (4, 1, 3),
            (4, 3, 3),
            (8, 6, 7),
            (8, 0, 7),
            (8, 7, 7),
        ]
    }

    fn get_quantize_data() -> Vec<(f32, f32, f32, f32, f32)> {
        vec![ 
            ( 12.6,   8.4, 64.0,   0.0,  0.0),
            ( 67.2,  12.8, 64.0,  64.0,  0.0),
            (135.2,  63.9, 64.0, 128.0,  0.0),
            ( 17.2, 127.9, 64.0,   0.0, 64.0),
        ]
    }

    fn get_mapping_data() -> Vec<(f32, f32, f32, usize, usize)> {
        vec![
            (  0.0,  0.0, 64.0, 0, 0),
            ( 64.0,  0.0, 64.0, 1, 0),
            (128.0,  0.0, 64.0, 2, 0),
            (  0.0, 64.0, 64.0, 0, 1),
        ]
    }

    #[test]
    fn test_from_and_to_1d() {
        let data = get_data_2d();

        for (width, x1, y1) in data {
            let t = get_1d_from_2d(width, x1, y1);
            let (x2, y2) = get_2d_from_1d(width, t);

            assert_eq!(x1, x2);
            assert_eq!(y1, y2);
        }
    }

    #[test]
    fn test_from_and_to_1d_ivec2() {
        let data = get_data_2d();

        for (width, x1, y1) in data {
            let s1 = IVec2 {
                x: x1 as i32,
                y: y1 as i32,
            };

            let t = get_1d_from_2d_ivec2(width, s1);
            let s2 = get_2d_from_1d_ivec2(width, t);

            assert_eq!(s1, s2);
        }
    }

    #[test]
    fn test_into_iter() {
        let test: Array2d<u64> = Array2d::new(2, 2);
        assert_eq!(test.len(), 4);

        for (_pos, value) in &test {
            // Does this compile?
            assert_eq!(*value, 0);
        }
    }

    #[test]
    fn test_into_iter_mut() {
        let test: Array2d<i64> = Array2d::new(2, 2);
        assert_eq!(test.len(), 4);
        
        for (_pos, mut _value) in &test {
            // Does this compile?
            _value = &10;
        }
    }

    #[test]
    fn test_getter_setter() {
        let mut test: Array2d<usize> = Array2d::new(2, 2);
        assert_eq!(test.len(), 4);

        for i in 0..test.len() {
            test[i] = i;
            let comp = test[i];

            assert_eq!(i, comp);
        }
    }

    #[test]
    fn test_resize_array() {
        let mut test : Array2d<usize> = Array2d::new(2, 2);
        assert_eq!(test.len(), 4);
        test.resize(3, 3);
        assert_eq!(test.len(), 9);
    }

    #[test]
    fn test_getter_and_setter() {
        let mut test : Array2d<usize> = Array2d::new(4, 4);
        assert_eq!(test.len(), 16);

        let mut pos = IVec2{ x : 0, y : 0};
        assert_eq!(*test.get(pos), 0);
        test.set(pos, 1);
        assert_eq!(*test.get(pos), 1);

        pos = IVec2{ x : 3, y : 3};
        assert_eq!(*test.get(pos), 0);
        test.set(pos, 64);
        assert_eq!(*test.get(pos), 64);
    }

    #[test]
    fn test_quantize_element() {
        let data = get_quantize_data();
        for (x, y, size, x1, y1) in data {
            let vec = Vec2{ x, y };
            let r = quantize_to_grid(vec, size);

            assert_eq!(r.x, x1);
            assert_eq!(r.y, y1);
        }
    }

    #[test]
    fn test_map_element() {
        let data = get_mapping_data();
        for (x, y, size, x1, y1) in data {
            let vec = Vec2{ x, y };
            let r = map_to_grid_vec2(vec, size);

            assert_eq!(r.x, x1 as i32);
            assert_eq!(r.y, y1 as i32);
        }
    }
}
