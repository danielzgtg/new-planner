#![feature(array_chunks)]
#![feature(array_methods)]

mod models;
mod loader;
mod analysis;

pub use loader::read_data;
pub use loader::load_data;
pub use analysis::print_analysis;
