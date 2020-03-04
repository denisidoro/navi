use std::error::Error;

use super::aux;

pub fn main(_func: String, _args: Vec<String>) -> Result<(), Box<dyn Error>> {
    aux::abort("calling `navi fn`", 201)
}
