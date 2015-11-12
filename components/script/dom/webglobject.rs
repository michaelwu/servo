/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

// https://www.khronos.org/registry/webgl/specs/latest/1.0/webgl.idl
use dom::bindings::codegen::Bindings::WebGLObjectBinding;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::Root;
use dom::bindings::magic::alloc_dom_object;

magic_dom_struct! {
    pub struct WebGLObject;
}

impl WebGLObject {
    pub fn new_inherited(&mut self) {
    }

    pub fn new(global: GlobalRef) -> Root<WebGLObject> {
        let mut obj = alloc_dom_object::<WebGLObject>(global);
        obj.new_inherited();
        obj.into_root()
    }
}
