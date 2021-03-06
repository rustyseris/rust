// Copyright 2013-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// revisions: ast mir
//[mir]compile-flags: -Z borrowck=compare

struct TrieMapIterator<'a> {
    node: &'a usize
}

fn main() {
    let a = 5;
    let _iter = TrieMapIterator{node: &a};
    _iter.node = & //[ast]~ ERROR cannot assign to immutable field `_iter.node`
                   //[mir]~^ ERROR cannot assign to immutable field `_iter.node` (Ast)
                   // FIXME Error for MIR
    panic!()
}
