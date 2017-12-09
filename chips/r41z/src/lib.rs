#![feature(asm,concat_idents,const_fn,const_cell_new)]
#![no_std]

#![crate_name = "r41z"]
#![crate_type = "rlib"]
extern crate cortexm0;
#[allow(unused_imports)]
#[macro_use(debug)]
extern crate kernel;