pub mod vec;
pub use self::vec::vec_no_init;
pub use self::vec::vec_init;
pub use self::vec::vec_random_init;
pub use self::vec::vec_zeroes;
pub use self::vec::no_split;

pub mod reduce;
pub use self::reduce::reduce;

pub mod scan;
pub use self::scan::scan;
pub mod par_scan;
pub use self::par_scan::par_scan;
pub use self::par_scan::par_scan_inplace;

mod map;
pub use self::map::map;
mod par_map;
pub use self::par_map::sqrt_splits_par_map;
pub use self::par_map::n_splits_par_map;
pub use self::par_map::rayon_par_map;
pub use self::par_map::par_map;

mod filter;
pub use self::filter::filter;
mod par_filter;
pub use self::par_filter::par_filter;
pub use self::par_filter::non_inplace_par_filter;
pub use self::par_filter::rayon_par_filter;

mod flatten;
pub use self::flatten::flatten;
mod par_flatten;
pub use self::par_flatten::par_flatten;
pub use self::par_flatten::rayon_par_flatten;

mod par_bucket_transpose;
pub use self::par_bucket_transpose::par_buckets_transpose;

mod par_block_transpose;
pub use self::par_block_transpose::par_block_transpose;

mod par_transpose;
pub use self::par_transpose::par_transpose;

mod utils;
pub use self::utils::num_blocks;
pub use self::utils::par_for;
pub use self::utils::single_sliced_for;
pub use self::utils::double_sliced_for;
pub use self::utils::par_copy;
pub use self::utils::p_split3;


