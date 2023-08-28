//!## Usage
//!#no_run
//! ```rust 
//! use crate::realpath::realpath; 
//! use std::path::PathBuf; 

//! let src = PathBuf::from("Cargo.toml");
//! let dest : PathBuf = realpath(&src)?; 
//! println!("{} -> {}", src.display(), dest.display()); 
//! ```


mod util; 
pub use util::*; 

#[cfg(test)]
mod test; 