# realpath-rs
A cross-platform Rust equivalent of python3's `os.path.realpath`

## Usage

```rust 
use realpath::*; 
use std::path::PathBuf; 

let src = PathBuf::from("Cargo.toml");
let dest : PathBuf = realpath(&src)?; 
println!("{} -> {}", src, dest.display()); //Returns Cargo.toml -> /path/to/Cargo.toml on linux and Drive:\path\to\Cargo.toml on windows

//For windows 
let src = PathBuf::from(r"Doge.exe");
let dest : PathBuf = realpath_win(&src , false)?; 
println!("{}" , dest.display()); //Returns \\?\Drive:\path\to\Doge.exe
let dest : PathBuf = realpath_win(&src , true)?;
println!("{}" , dest.display()); //Returns Drive:\path\to\Doge.exe

//Inner functionality 
let dest = realpath_og(&src)?;
println!("{}" , dest.display()); //Returns /path/to/Cargo.toml on linux and \\?\Drive:\path\to\Cargo.toml on windows
``` 
