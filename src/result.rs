use std::io;
use std::result::Result as StdResult;

pub type Result<T, E = io::Error> = StdResult<T, E>;
