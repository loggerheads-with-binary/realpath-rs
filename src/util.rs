use std::path::PathBuf;

extern crate path_absolutize;
use path_absolutize::Absolutize;

#[no_mangle]
///Mutates the path in-place
///Canonicalizes the entire path, if it is possible 
///If it is not possible, then it canonicalizes the head of the path and then appends the tail to it 
///Repeats the process until either the head is successfully canonicalized or the head is the root directory  
fn step_by_step_canonicalize(path: &mut PathBuf) {

    let mut components = path.components().collect::<Vec<_>>();
    let mut head = path.clone();
    let mut tail = PathBuf::new();
    let mut result = head.canonicalize(); 
    let mut component; 
    let mut cx; 

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
            Some(component) => {
                //tail = component\tail
                cx = PathBuf::from(component.as_os_str());
                tail = cx.join(tail);
            }
            None => return,
        } 
    }
    
    let result = result.unwrap().join(tail);
    let result = result.absolutize().unwrap().to_path_buf();
    
    //Remove last / from the result 

    *path = result;

}


///Takes a PathBuf as input 
///Normalizes it, removes any trailing slashes, intermediate dots, and such 
///Traces symlinks and processes as far as can be canonicalized
///If only a part of the path is canonicalize-able, then the remaining part is appended as is to the canonicalized part
///Returns the canonicalized path as a PathBuf
///Returns an Error if there is some problem with absolutization/canonicalization
///Error => std::io::Error
pub fn realpath(path: &PathBuf) -> Result<PathBuf, std::io::Error> {
    
    let mut path = path.absolutize()?.to_path_buf();

    step_by_step_canonicalize(&mut path);
    Ok(path)
}
