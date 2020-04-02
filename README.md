# Karalle

A collection of parallel algorithms written in [rust](rust-lang.org) with [rayon](https://github.com/rayon-rs/rayon) as underlying thread-pool. Algorithms inspired by [Introduction to Parallel Algorithm](https://ldhulipala.github.io/notes/parallel.pdf?fbclid=IwAR2G9v8SgBawI93MvSo3ZFNjkOMQ2U5SyKgEvQpVVQBsWBmWeppWDVdE4pM), [Low Depth Cache-Oblivious Algorithms](https://kilthub.cmu.edu/articles/Low_Depth_Cache-Oblivious_Algorithms/6607052/1?fbclid=IwAR0h2mUKS5U6vh7yddDDs5hOF8iKqDhjxxqmTV8mfosK4cqnF7qRcC_bbFg), and [pbbs](https://github.com/cmuparlay/pbbslib)

## Motivation

Improves performance of rayon's current state of the art from the knowledge learned from Contemporary Algorithm and Organization of Programming Language classes.

## Project Structure

All the codes for algorithms are located in `src` directory. list of project's dependencies are located in `Cargo.toml` 

* `src/benchmark` consists of benchmark files for all main algorithms
* `src/constant` consists of constants use for this project (eg. `THRESHOLD`)
* `src/primitive` consists of parallel algorithms necessary for constructing the sorting algorithms (eg. `par_scan`, `par_filter`, `par_map`, `par_flatten`, `par_bucket_transpose`)
* `src/sort` consists of parallel sorting algorithms (eg. `par_quick_sort`, `par_sample_sort`)
* `src/util` consists of tools for reading files and generating test data
* `src/lib.rs` contains unit tests for all main algorithms
* `src/main.rs` contains logic to handle command-line arguments for benchmarking

## Methodology

For unit testing, we run our algorithms along side with rust's standard algorithms to check the correctness. We benchmark our algorithms against rayon's algorithms to compare the performance and the scalability. We run benchmark on 24 threads CPU and 12GB RAM VM. Our test data size is ranged from $2^1$ to $2^28$ of `i16` integer. 

## Result

We mostly win most of the case for each algorithm!! (comparison graphs are in the slides)

## Lesson Learned

* For parallel algorithms, sometimes we could not acheive the claimed span/depth due to overhead cost from OS and hardware
* Different CPU models and OSes can yield different results to the algorithms because of how threads are scheduled and cache size
* Rust is really good at preventing data race when it comes to parallel algorithms by prohibiting the use of shared memory between threads, but looking from another angle, this could be a limitation to rust since there are some parallel algorithms that require an access to the whole data structure in order to achieve better performance or span/depth

## Usage

```rust
// Import all primitives
use crate::primitive::*;

// parallel scan (non in-place)
// similar to fold, but collectively place results from the function in the result array
let (prefx_sums, total) = par_scan( 		     
  &mut vec![1, 2, 3],										     // input array
  |a: &i32, b: &i32| -> i32 { *a + *b },     // function to apply
  &0, 																	     // initial value
);
// ([0, 1, 3], 6)

// parallel scan (in-place)
// similar to fold, but collectively place results from the function in the original array
let total = par_scan_inplace(					       
  &mut vec![1, 2, 3],												 // input array
  |a: &i32, b: &i32| -> i32 { *a + *b },		 // function to apply
  &0,																				 // initial value
);
// ([0, 1, 3], 6)

// parallel map
// apply the function to each elements and return a new array
let result_arr = par_map(
  &vec![1, 2, 3], 																								// input array
  &|i: usize, a: &i32| -> i32 { if *a <= 90 { 1 } else { 0 } }	  // function to apply
);
// [0, 0, 0]

// par_flatten
// reduce the dimensions to just 1D on a new array
let arr1 = vec![1, 2, 3];
let arr2 = vec![4, 5, 6];
let arr3 = vec![7, 8, 9];
let result_arr = par_flatten(
  &vec![arr1, arr2, arr3] 											// input array of arrays
);
// [1, 2, 3, 4, 5, 6, 7, 8, 9]

// par_filter
// apply function to each element keeping elements that return true and return a new array
let result_arr: Vec<i32> = par_filter(
  &vec![1, 2, 3], 															// input array
  &|i: usize, a: &i32| -> bool { *a < 3 }				// filter function
);
// [1, 2]

///////////////////////////////////////////////////////////////////////////////////////////////

// import all sorting algorithms
use crate::sort::*;

// quicksort (non in-place)
// sort the given array and return a new array
let sorted_arr = non_inplace_par_quicksort(
  &vec![2, 1, 3], 														// original array
  |a: &i32, b: &i32| -> i32 { *a - *b }				// compare function
);
// [1, 2, 3]

// quicksort (in-place)
// sort within the original array
let mut original_arr = vec![2, 1, 3]; 
par_quicksort(
  &mut original_arr, 												// original array
  |a: &i32, b: &i32| -> i32 { *a - *b }			// compare function
);
// [1, 2, 3]

// samplesort (better version of quicksort, more cache friendly, less variance of subtree)
// sort within the original array
let mut original_arr = vec![2, 1, 3]
par_samplesort(
  &mut original_arr, 												// original array
  &|a: &i32, b: &i32| -> i32 { *a - *b }		// compare function
);
// [1, 2, 3]

```



## Running Benchmark

```zsh
cd karalle
KTHREAD=<number-of-threads> KROUND=<number-of-rounds> KSIZE=<benchmark-input-size> \
KTYPE=<all|map|bm|filter|flatten|scan|qs|ss|ms|sort> cargo run --release
```

* KTHREAD: number of threads will be using in the benchmark
* KROUND: number of rounds for each input size
* KSIZE: maximum data size `x` (ranged from `2^1` to  `2^x` increment `x` by `1`)
* KTYPE: type of bechmark to be run
  * all: run every benchmark
  * map: map benchmark (`sort_splits_par_map`, `par_map`, `rayon_par_map`)
  * bm: map benchmark on big files
  * filter: filter benchmark (`par_filter`, `rayon_par_filter`)
  * flatten: flatten benchmark (`par_flatten`, `rayon_par_flatten`)
  * qs: quicksort benchmark (`non_inplace_par_quicksort`, `par_quicksort`, `rayon_par_quicksort`)
  * ss: samplesort benchmark (`seq_sample_sort`, `par_samplesort`)
  * ms: mergesort benchmark (`rayon_par_mergesort`)
  * sort: all sorting algorithms benchmark (`non_inplace_par_quicksort`, `par_quicksort`, `rayon_par_quicksort`,`rayon_par_mergesort`, `seq_sample_sort`,`par_samplesort`)