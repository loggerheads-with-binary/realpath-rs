use crate::*; 
use std::path::PathBuf;

#[test]
#[cfg(windows)]

fn test_win(){
    let src = PathBuf::from(r"\\?\C:\Users\Public\Documents\");
    let dest : PathBuf = realpath(&src).unwrap(); 
    println!("{}" , dest.display());
}