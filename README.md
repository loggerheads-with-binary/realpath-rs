# realpath-rs
A cross-platform Rust equivalent of python3's `os.path.realpath`

## Usage

```rust 
use realpath::realpath; 
use std::path::PathBuf; 

let src = PathBuf::from("Cargo.toml");
let dest : PathBuf = realpath(&src)?; 
println!("{} -> {}", src, dest.display()); 

//For windows 
let src = PathBuf::from(r"Doge.exe");
let dest : PathBuf = realpath_win(&src , false)?; 
println!("{}" , dest.display()); //Returns \\?\Drive:\path\to\Doge.exe
let dest : PathBuf = realpath_win(&src , true)?;
println!("{}" , dest.display()); //Returns Drive:\path\to\Doge.exe
``` 
