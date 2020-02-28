use std::error::Error;

use super::aux;

pub fn main(_args: Vec<String>) -> Result<(), Box<dyn Error>> {
    aux::abort()
}
