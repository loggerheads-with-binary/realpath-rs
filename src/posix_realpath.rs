use std::path::{PathBuf};

extern crate path_absolutize;
use path_absolutize::Absolutize;

#[no_mangle]
fn step_by_step_canonicalize(path: &mut PathBuf) {

    let mut components = path.components().collect::<Vec<_>>();
    let mut head = path.clone();
    let mut tail = PathBuf::new();
    let mut result = head.canonicalize(); 
    let mut component; 

    //Canonicalize the head 
    //If error is raised, head = head.parent 
    //And then loop again
    //If head.parent is None, then return the original path without mutation 
    //Otherwise, return the canonicalized head with the tail components attached 
    while result.is_err(){
        head = match head.parent(){
            Some(parent) => parent.to_path_buf(),
            None => return,
        };

        result = head.canonicalize();
        component = components.pop();

        match component{
            Some(component) => tail.push(component),
            None => return,
        } 
    }
    
    let result = result.unwrap().join(tail);
    *path = result;

}

pub fn realpath(path: &PathBuf) -> Result<PathBuf, std::io::Error> {
    
    let mut path = path.absolutize()?.to_path_buf();

    step_by_step_canonicalize(&mut path);
    Ok(path)
}
