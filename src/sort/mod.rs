mod quick_sort;
pub use self::quick_sort::quick_sort;

mod par_quick_sort;
pub use self::par_quick_sort::non_inplace_par_quicksort;
pub use self::par_quick_sort::par_quick_sort_slice;
pub use self::par_quick_sort::par_quicksort;
pub use self::par_quick_sort::rayon_par_quicksort;
pub use crate::primitive::par_copy;

mod sample_sort;
pub use self::sample_sort::seq_sample_sort;

mod par_sample_sort;
pub use self::par_sample_sort::par_samplesort;
