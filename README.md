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

TBA



## Lesson Learned

* For parallel algorithms, sometimes we could not acheive the claimed span/depth due to overhead cost from OS and hardware
* Different CPU models and OSes can yield different results to the algorithms because of how threads are scheduled and cache size
* Rust is really good at preventing data race when it comes to parallel algorithms by prohibiting the use of shared memory between threads, but looking from another angle, this could be a limitation to rust since there are some parallel algorithms that require an access to the whole data structure in order to achieve better performance or span/depth

## Usage

```python
import foobar

foobar.pluralize('word') # returns 'words'
foobar.pluralize('goose') # returns 'geese'
foobar.singularize('phenomena') # returns 'phenomenon'
```

