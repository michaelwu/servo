/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

// https://www.khronos.org/registry/webgl/specs/latest/1.0/webgl.idl
use dom::bindings::codegen::Bindings::WebGLShaderPrecisionFormatBinding;
use dom::bindings::codegen::Bindings::WebGLShaderPrecisionFormatBinding::WebGLShaderPrecisionFormatMethods;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::Root;
use dom::bindings::magic::alloc_dom_object;

magic_dom_struct! {
    pub struct WebGLShaderPrecisionFormat {
        range_min: i32,
        range_max: i32,
        precision: i32,
    }
}

impl WebGLShaderPrecisionFormat {
    fn new_inherited(&mut self, range_min: i32, range_max: i32, precision: i32) {
        self.range_min.init(range_min);
        self.range_max.init(range_max);
        self.precision.init(precision);
    }

    pub fn new(global: GlobalRef,
               range_min: i32,
               range_max: i32,
               precision: i32) -> Root<WebGLShaderPrecisionFormat> {
        let mut obj = alloc_dom_object::<WebGLShaderPrecisionFormat>(global);
        obj.new_inherited(range_min, range_max, precision);
        obj.into_root()
    }
}

impl WebGLShaderPrecisionFormatMethods for WebGLShaderPrecisionFormat {
    // https://www.khronos.org/registry/webgl/specs/1.0/#5.12.1
    fn RangeMin(&self) -> i32 {
        self.range_min
    }

    // https://www.khronos.org/registry/webgl/specs/1.0/#5.12.1
    fn RangeMax(&self) -> i32 {
        self.range_max
    }

    // https://www.khronos.org/registry/webgl/specs/1.0/#5.12.1
    fn Precision(&self) -> i32 {
        self.precision
    }
}
