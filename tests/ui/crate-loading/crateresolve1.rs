//@ aux-build:crateresolve1-1.rs
//@ aux-build:crateresolve1-2.rs
//@ aux-build:crateresolve1-3.rs

//@ normalize-stderr: "\.nll/" -> "/"
//@ normalize-stderr: "\\\?\\" -> ""
//@ normalize-stderr: "(lib)?crateresolve1-([123])\.[a-z]+" -> "libcrateresolve1-$2.somelib"

// NOTE: This test is duplicated at `tests/ui/error-codes/E0464.rs`.

extern crate crateresolve1;
//~^ ERROR multiple candidates for `rlib` dependency `crateresolve1` found

fn main() {}
