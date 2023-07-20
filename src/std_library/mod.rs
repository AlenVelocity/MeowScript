use std::collections::HashMap;

use crate::evaluation::object::Object;
pub mod util;
pub mod array;
pub mod fs;
pub mod string;
pub mod math;
/// Function to load a standard library 
/// # Arguments
/// * `lib` - The name of the library to load.
/// # Returns
/// `HashMap<String, Object>` - The environment with the library loaded.

pub struct Res {
    pub globals: HashMap<String, Object>,
    pub raw: Option<String>,
}
pub fn get_std_lib(lib: String) -> Option<Res> {
    match lib.as_str() {
        "nya:clawtility" => Some(util::add_globals()),
        "nya:furrball" => Some(array::add_globals()),
        "nya:whiskers" => Some(string::add_globals()),
        "nya:scratchpad" => Some(fs::add_globals()),
        "nya:catculator" => Some(math::add_globals()),
        //"nya:meowternet" => Some(http::add_globals()),
        _ => None,
    }
}
