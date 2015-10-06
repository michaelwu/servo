/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! Routines for handling measuring the memory usage of arbitrary DOM nodes.

use dom::bindings::codegen::InheritTypes::*;
use dom::bindings::utils::TopDOMClass;
use dom::eventtarget::EventTarget;
use libc;
use util::mem::{HeapSizeOf, heap_size_of};

// This is equivalent to measuring a Box<T>, except that DOM objects lose their
// associated box in order to stash their pointers in a reserved slot of their
// JS reflector. It is assumed that the caller passes a pointer to the most-derived
// type that this pointer represents, or the actual heap usage of the pointee will
// be under-reported.
fn heap_size_of_self_and_children<T: HeapSizeOf>(obj: &T) -> usize {
    heap_size_of(obj as *const T as *const libc::c_void) + obj.heap_size_of_children()
}

pub fn heap_size_of_eventtarget(target: &EventTarget) -> usize {
    0
}
