#[cfg(target_arch = "wasm32")]
extern crate wee_alloc;
#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


pub mod proof_of_work;
pub use proof_of_work::*;