//@ check-pass
#![warn(deref_into_dyn_supertrait)]

use std::ops::Deref;

trait Bar<T> {}

trait Foo: Bar<i32> {
    fn as_dyn_bar_u32<'a>(&self) -> &(dyn Bar<u32> + 'a);
}

impl<'a> Deref for dyn Foo + 'a {
    //~^ warn: this `Deref` implementation is covered by an implicit supertrait coercion
    type Target = dyn Bar<u32> + 'a;

    fn deref(&self) -> &Self::Target {
        self.as_dyn_bar_u32()
    }
}

fn main() {}
