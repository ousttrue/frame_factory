// this is generated.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
use std::ffi::c_void;
extern crate va_list;
use super::*;
{% for define in defines %}
{{define}}
{% endfor %}
