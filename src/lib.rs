//! ## Usage
//!
//! ```rust 
//! use realpath::realpath; 
//! use std::path::PathBuf; 

//! let src = PathBuf::from("Cargo.toml");
//! let dest : PathBuf = realpath(&src)?; 
//! println!("{} -> {}", src, dest); 
//! ```


mod util; 
pub use util::realpath; 