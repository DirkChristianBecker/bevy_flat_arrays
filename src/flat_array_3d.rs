use std::ops::{Index, IndexMut};
use bevy::prelude::*;

/// Get the array index from a 3 point. This is the inverse operation to
/// get_3d_from_1d.
/// 
/// # Example 
/// ```
/// use bevy_flat_arrays::prelude::tools::get_1d_from_3d;
/// use bevy_flat_arrays::prelude::tools::get_3d_from_1d;
/// let x = 1;
/// let y = 0;
/// let z = 0;
/// let height = 2;
/// let width = 2;
/// let i = get_1d_from_3d(height, width, x, y, z);
/// let (x1, y1, z1) = get_3d_from_1d(height, width, i);
/// assert_eq!(x, x1);
/// assert_eq!(y, y1);
/// assert_eq!(z, z1);
/// ```
pub fn get_1d_from_3d(max_x: usize, max_y: usize, x: usize, y: usize, z: usize) -> usize {
    (z * max_x * max_y) + (y * max_x) + x
}

/// Get the position from the array index. This is the inverse operation to
/// get_1d_from_3d.
/// 
/// # Example 
/// ```
/// use bevy_flat_arrays::prelude::tools::get_1d_from_3d;
/// use bevy_flat_arrays::prelude::tools::get_3d_from_1d;
/// let x = 1;
/// let y = 0;
/// let z = 0;
/// let height = 2;
/// let width = 2;
/// let i = get_1d_from_3d(height, width, x, y, z);
/// let (x1, y1, z1) = get_3d_from_1d(height, width, i);
/// assert_eq!(x, x1);
/// assert_eq!(y, y1);
/// assert_eq!(z, z1);
/// ```
pub fn get_3d_from_1d(max_x: usize, max_y: usize, idx: usize) -> (usize, usize, usize) {
    let z = idx / (max_x * max_y);
    let idx2 = idx - (z * max_x * max_y);
    let y = idx2 / max_x;
    let x = idx2 % max_x;

    (x, y, z)
}

/// Get the array index from a ivec3. This is a wrapper around get_1d_from_3d.
pub fn get_1d_from_3d_ivec3(max_x: usize, max_y: usize, v: IVec3) -> usize {
    let x = v.x as usize;
    let y = v.y as usize;
    let z = v.z as usize;

    get_1d_from_3d(max_x, max_y, x, y, z)
}

/// Get the position for an array index. This is a wrapper around get_3d_from_1d.
pub fn get_3d_from_1d_ivec3(max_x: usize, max_y: usize, idx: usize) -> IVec3 {
    let (x, y, z) = get_3d_from_1d(max_x, max_y, idx);
    IVec3 {
        x: x as i32,
        y: y as i32,
        z: z as i32,
    }
}

/// Map a world vector to a position on a predefined grid. Think
/// of a voxel world. e.g. if we want to map a collider position
/// from a raycast to a voxel inside a world we can use this function
/// to do so.
/// 
/// ```
/// use bevy::prelude::*;
/// use bevy_flat_arrays::prelude::tools::map_to_grid_vec3;
/// let v = Vec3 { x : 35.8277, y : 7.987278, z : 2.0993 };
/// let grid_size = 4.0;
/// let mapped = map_to_grid_vec3(v, grid_size);
/// assert_eq!(IVec3 { x : 32, y : 4, z : 0 }, mapped); 
/// 
/// ```
pub fn map_to_grid_vec3(v : Vec3, grid_size : f32) -> IVec3 {
    let x = ((v.x / grid_size).floor() * grid_size) as i32;
    let y = ((v.y / grid_size).floor() * grid_size) as i32;
    let z = ((v.z / grid_size).floor() * grid_size) as i32;

    IVec3 { x, y, z, }
}


/// # Array3d
/// 
/// This array creates a 3 dimensional array that keeps its data in a cache friendly way.
/// This should reduce cache misses while iterating the array and reduce the number of 
/// indirections. This should result in an increase in performance when iterating
/// over the data.
/// 
/// # Traits and behaviour
/// 
/// Both a immutable and an mutable iterator are provided. However, both iterators
/// always map an index to a position and return the position as an ivec3 along with the
/// actual value. This means there is always a little computational overhead. To avoid this
/// overhead the index trait has been implemented. If one accesses the data via index, 
/// no additional comnputation takes place.
/// 
/// The memory for the array is allocated when a new array is created and can be resized
/// using the resize function. To make it easier to allocate memory, all types are required
/// to implement the Default trait. 
pub struct Array3d<T: std::default::Default> {
    width: usize,
    height: usize,
    depth: usize,
    array: Vec<T>,
}

impl<T: std::default::Default> Array3d<T> {
    /// Constructs a new array.
    pub fn new(width: usize, height: usize, depth: usize) -> Self {
        assert!(width > 0);
        assert!(height > 0);
        assert!(depth > 0);

        let mut r: Vec<T> = Vec::new();
        r.resize_with(width * height * depth, || T::default());

        Array3d {
            width,
            height,
            depth,
            array: r,
        }
    }

    /// Resize this array to the given dimensions.
    pub fn resize(&mut self, width : usize, heigth : usize, depth : usize) {
        self.height = heigth;
        self.width = width;
        self.depth = depth;
        self.array.resize_with(width * heigth * depth, || T::default());
    }

    /// Returns the number of items inside this array holds.
    pub fn len(&self) -> usize {
        self.width * self.height * self.depth
    }

    /// Implemented to silence the compiler. Always return false.
    pub fn is_empty(&self) -> bool {
        false
    }

    /// Get the value for the given position.
    pub fn get(&self, v : IVec3) -> &T {
        let i = get_1d_from_3d_ivec3(self.width, self.height, v);
        assert!(i < self.len(), "Invalid index");
        &self.array[i]
    }

    /// Get a mutable reference for the given position.
    pub fn get_mut(&mut self, v : IVec3) -> &mut T {
        let i = get_1d_from_3d_ivec3(self.width, self.height, v);
        assert!(i < self.len(), "Invalid index");
        &mut self.array[i]
    }

    /// Update the value for the given position.
    pub fn set(&mut self, v : IVec3, value : T) {
        let i = get_1d_from_3d_ivec3(self.width, self.height, v);
        assert!(i < self.len(), "Invalid index");
        self.array[i] = value;
    }

    /// Creates a new immutable iterator.
    pub fn iter(&self) -> Array3dIter<'_, T> {
        Array3dIter {
            items: &self.array,
            cursor: 0,
            max: self.len(),
            width: self.width,
            height: self.height,
        }
    }

    /// Creates a new mutable iterator.
    fn iter_mut(&mut self) -> Array3dMutIter<'_, T> {
        let len = self.len();

        Array3dMutIter {
            items: &mut self.array,
            cursor: 0,
            max: len,
            width: self.width,
            height: self.height,
        }
    }
}

impl<T: std::default::Default> Index<usize> for Array3d<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.len());
        &self.array[index]
    }
}

impl<T: std::default::Default> IndexMut<usize> for Array3d<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        assert!(index < self.len());
        &mut self.array[index]
    }
}

pub struct Array3dIter<'a, T: std::default::Default> {
    items: &'a Vec<T>,
    cursor: usize,
    max: usize,
    width: usize,
    height: usize,
}

impl<'a, T: std::default::Default> Iterator for Array3dIter<'a, T> {
    type Item = (IVec3, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let tmp = self.cursor;
        if tmp >= self.max {
            return None;
        }

        self.cursor += 1;
        let v = get_3d_from_1d_ivec3(self.width, self.height, tmp);

        Some((v, &self.items[tmp]))
    }
}

impl<'a, T: std::default::Default> IntoIterator for &'a Array3d<T> {
    type Item = (IVec3, &'a T);

    type IntoIter = Array3dIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Array3dMutIter<'a, T: std::default::Default> {
    items: &'a mut Vec<T>,
    cursor: usize,
    max: usize,
    width: usize,
    height: usize,
}

impl<'a, T: std::default::Default> Iterator for Array3dMutIter<'a, T> {
    type Item = (IVec3, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        let tmp = self.cursor;
        self.cursor += 1;
        if tmp >= self.max {
            return None;
        }

        let v = get_3d_from_1d_ivec3(self.width, self.height, self.cursor);

        let pt = self.items.as_mut_ptr();
        unsafe { Some((v, &mut *pt)) }
    }
}

impl<'a, T: std::default::Default> IntoIterator for &'a mut Array3d<T> {
    type Item = (IVec3, &'a mut T);

    type IntoIter = Array3dMutIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_data_3d() -> Vec<(usize, usize, usize, usize, usize)> {
        vec![
            (4, 4, 0, 0, 0),
            (4, 4, 1, 0, 0),
            (4, 4, 1, 1, 0),
            (4, 4, 1, 1, 1),
            (4, 4, 2, 1, 0),
            (4, 4, 2, 1, 1),
            (4, 4, 2, 2, 1),
            (4, 4, 2, 2, 2),
            (4, 4, 3, 2, 1),
            (4, 4, 3, 2, 2),
            (4, 4, 3, 3, 2),
            (4, 4, 3, 3, 3),
        ]
    }

    #[test]
    fn test_from_and_to_1d() {
        let data = get_data_3d();

        for (width, height, x1, y1, z1) in data {
            let t = get_1d_from_3d(width, height, x1, y1, z1);
            let (x2, y2, z2) = get_3d_from_1d(width, height, t);

            assert_eq!(x1, x2);
            assert_eq!(y1, y2);
            assert_eq!(z1, z2);
        }
    }

    #[test]
    fn test_from_and_to_1d_ivec() {
        let data = get_data_3d();

        for (width, height, x1, y1, z1) in data {
            let v1 = IVec3 { x : x1 as i32, y : y1 as i32, z : z1 as i32 };
            let t = get_1d_from_3d_ivec3(width, height, v1);
            let v2 = get_3d_from_1d_ivec3(width, height, t);

            assert_eq!(v1, v2);
        }
    }

    #[test]
    fn test_into_iter() {
        let test: Array3d<u64> = Array3d::new(2, 2, 2);
        assert_eq!(test.len(), 8);

        for (_pos, value) in &test {
            // Does this compile?
            assert_eq!(*value, 0);
        }
    }

    #[test]
    fn test_into_iter_mut() {
        let test: Array3d<i64> = Array3d::new(2, 2, 2);
        assert_eq!(test.len(), 8);
        
        for (_pos, mut _value) in &test {
            // Does this compile?
            _value = &10;
        }
    }

    #[test]
    fn test_getter_setter() {
        let mut test: Array3d<usize> = Array3d::new(2, 2, 2);
        assert_eq!(test.len(), 8);

        for i in 0..test.len() {
            test[i] = i;
            let comp = test[i];

            assert_eq!(i, comp);
        }
    }

    #[test]
    fn test_resize_array() {
        let mut test : Array3d<usize> = Array3d::new(2, 2, 2);
        assert_eq!(test.len(), 8);
        test.resize(3, 3, 3);
        assert_eq!(test.len(), 27);
    }

    #[test]
    fn test_getter_and_setter() {
        let mut test : Array3d<usize> = Array3d::new(4, 4, 4);
        assert_eq!(test.len(), 64);

        let mut pos = IVec3{ x : 0, y : 0, z : 0};
        assert_eq!(*test.get(pos), 0);
        test.set(pos, 1);
        assert_eq!(*test.get(pos), 1);

        pos = IVec3{ x : 3, y : 3, z : 3};
        assert_eq!(*test.get(pos), 0);
        test.set(pos, 64);
        assert_eq!(*test.get(pos), 64);
    }
}
