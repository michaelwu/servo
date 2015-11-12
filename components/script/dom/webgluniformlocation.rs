/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

// https://www.khronos.org/registry/webgl/specs/latest/1.0/webgl.idl
use dom::bindings::codegen::Bindings::WebGLUniformLocationBinding;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::Root;
use dom::bindings::magic::alloc_dom_object;

magic_dom_struct! {
    pub struct WebGLUniformLocation {
        id: i32,
    }
}

impl WebGLUniformLocation {
    fn new_inherited(&mut self, id: i32) {
        self.id.init(id);
    }

    pub fn new(global: GlobalRef, id: i32) -> Root<WebGLUniformLocation> {
        let mut obj = alloc_dom_object::<WebGLUniformLocation>(global);
        obj.new_inherited(id);
        obj.into_root()
    }
}


impl WebGLUniformLocation {
    pub fn id(&self) -> i32 {
        self.id
    }
}
