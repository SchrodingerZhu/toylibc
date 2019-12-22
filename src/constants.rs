#![allow(dead_code)]

use crate::types::*;

pub static NULL: intptr_t = 0;
pub static O_RDONLY: int_t = 0;
pub static ENOMEM: int_t = 12; /* Cannot allocate memory */
pub static EACCES: int_t = 13; /* Permission denied */
pub static EEXIST: int_t = 17; /* File exists */
pub static EISDIR: int_t = 21; /* Is a directory */
pub static EINVAL: int_t = 22; /* Invalid argument */
pub static ERANGE: int_t = 34; /* Result too large */