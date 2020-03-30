mod big_map;
mod map;
mod flatten;
mod quick_sort;
mod scan;
mod sample_sort;
mod merge_sort;
mod sort;
mod filter;

pub use self::map::run_map_benchmark;
pub use self::flatten::run_flatten_benchmark;
pub use self::quick_sort::run_quick_sort_benchmark;
pub use self::scan::run_scan_benchmark;
pub use self::sample_sort::run_sample_sort_benchmark;
pub use self::merge_sort::run_merge_sort_benchmark;
pub use self::big_map::big_map_seq;
pub use self::sort::run_sorting_benchmark;