#[cfg(target_os = "windows")]
#[path = "windows_realpath.rs"]
mod windows_realpath;
#[cfg(target_os = "windows")]
pub use windows_realpath::realpath;

//Posix systems and derivatives
#[cfg(any(
    target_os = "linux",
    target_os = "macos",
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "openbsd",
    target_os = "netbsd"
))]
#[path = "posix_realpath.rs"]
mod posix_realpath;
#[cfg(any(
    target_os = "linux",
    target_os = "macos",
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "openbsd",
    target_os = "netbsd"
))]
pub use posix_realpath::realpath;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};

    #[test]
    #[cfg(target_os = "windows")]
    fn test_realpath() {
        let mut path = PathBuf::from("C:\\Users\\");
        path.push("user");
        path.push("Desktop");
        path.push("test.txt");
        let path = realpath(&path).unwrap();
        println!("{:?}", path);
    }

    #[test]
    #[cfg(any(
        target_os = "linux",
        target_os = "macos",
        target_os = "freebsd",
        target_os = "dragonfly",
        target_os = "openbsd",
        target_os = "netbsd"
    ))]
    fn test_realpath() {
        //Check if /usr/bin and /bin point to the same target using realpath
        let path = PathBuf::from("/usr/bin");
        let path = realpath(&path).unwrap();
        let path2 = PathBuf::from("/bin");
        let path2 = realpath(&path2).unwrap();
        assert_eq!(path, path2);
    }
}
