pub mod scan;
pub use self::scan::scan;
pub mod par_scan;
pub use self::par_scan::par_scan;

mod map;
pub use self::map::map;
mod par_map;
pub use self::par_map::par_map_v1;
pub use self::par_map::par_map_v2;
pub use self::par_map::par_map_v3;
pub use self::par_map::par_map_v4;
pub use self::par_map::par_map_v5;

mod filter;
pub use self::filter::filter;
mod par_filter;
pub use self::par_filter::par_filter_v1;
//pub use self::par_filter::par_filter_v2;

mod flatten;
pub use self::flatten::flatten;
mod par_flatten;
pub use self::par_flatten::par_flatten;
pub use self::par_flatten::par_flatten_v2;

mod sample_sort;
pub use self::sample_sort::seq_sample_sort;


