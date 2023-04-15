//! ## Usage
//!#no_run
//! ```rust 
//! use realpath::realpath; 
//! use std::path::PathBuf; 

//! let src = PathBuf::from("Cargo.toml");
//! let dest : PathBuf = realpath(&src)?; 
//! println!("{} -> {}", src, dest.display()); 
//! ```


mod util; 
pub use util::*; 