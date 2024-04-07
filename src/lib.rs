mod flat_array_2d;
mod flat_array_3d;

/// This library implements 2 and 3 dimensional arrays that keep their data
/// sequentially in memory and can be accessed using bevy vecs.
pub mod prelude {
    pub mod tools {
        use crate::flat_array_2d;
        use crate::flat_array_3d;

        // 2d
        pub use flat_array_2d::get_1d_from_2d;
        pub use flat_array_2d::get_1d_from_2d_ivec2;
        pub use flat_array_2d::get_2d_from_1d;
        pub use flat_array_2d::get_2d_from_1d_ivec2;
        pub use flat_array_2d::quantize_to_grid;
        pub use flat_array_2d::map_to_grid_vec2;

        // 3d
        pub use flat_array_3d::get_1d_from_3d;
        pub use flat_array_3d::get_1d_from_3d_ivec3;
        pub use flat_array_3d::get_3d_from_1d;
        pub use flat_array_3d::get_3d_from_1d_ivec3;
        pub use flat_array_3d::map_to_grid_vec3;
    }

    use crate::flat_array_2d;
    use crate::flat_array_3d;

    pub use flat_array_2d::Array2d;
    pub use flat_array_3d::Array3d;
}
