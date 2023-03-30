# realpath-rs
A cross-platform Rust equivalent of python3's `os.path.realpath`

## Usage

```rust 
use realpath::realpath; 
use std::path::PathBuf; 

let src = PathBuf::from("Cargo.toml");
let dest : PathBuf = realpath(&src)?; 
println!("{} -> {}", src, dest); 
```