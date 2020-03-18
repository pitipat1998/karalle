mod quick_sort;
pub use self::quick_sort::quick_sort;

mod par_quick_sort;
pub use self::par_quick_sort::par_quick_sort;
pub use self::par_quick_sort::par_quick_sort_v2;
pub use self::par_quick_sort::par_quick_sort_v3;
pub use crate::primitive::par_copy;

mod sample_sort;
pub use self::sample_sort::seq_sample_sort;

mod par_sample_sort;
pub use self::par_sample_sort::par_sample_sort;
