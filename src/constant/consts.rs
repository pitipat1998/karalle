use std::cmp::max;

pub const GRANULARITY: usize = 2000;
pub const QS_THRESHOLD: usize = 1 << 14;
pub const BLOCK_THRESHOLD: usize = 1;
pub const BUCKET_QUOTIENT: usize = 8;
pub const BLOCK_QUOTIENT: usize = 8;
pub const OVER_SAMPLE: usize = 8;
pub const LOG_BLOCK_SIZE: usize = 10;
pub const BLOCK_SIZE: usize = (1 << LOG_BLOCK_SIZE);
