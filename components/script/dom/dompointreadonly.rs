/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::DOMPointReadOnlyBinding::DOMPointReadOnlyMethods;
use dom::bindings::error::Fallible;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::Root;
use dom::bindings::magic::alloc_dom_object;
use std::cell::Cell;

// http://dev.w3.org/fxtf/geometry/Overview.html#dompointreadonly
magic_dom_struct! {
    pub struct DOMPointReadOnly {
        x: Mut<f64>,
        y: Mut<f64>,
        z: Mut<f64>,
        w: Mut<f64>,
    }
}

impl DOMPointReadOnly {
    pub fn new_inherited(&mut self, x: f64, y: f64, z: f64, w: f64) {
        self.x.init(x);
        self.y.init(y);
        self.z.init(z);
        self.w.init(w);
    }

    pub fn new(global: GlobalRef, x: f64, y: f64, z: f64, w: f64) -> Root<DOMPointReadOnly> {
        let mut obj = alloc_dom_object::<DOMPointReadOnly>(global);
        obj.new_inherited(x, y, z, w);
        obj.into_root()
    }

    pub fn Constructor(global: GlobalRef,
                        x: f64, y: f64, z: f64, w: f64) -> Fallible<Root<DOMPointReadOnly>> {
        Ok(DOMPointReadOnly::new(global, x, y, z, w))
    }
}

impl DOMPointReadOnlyMethods for DOMPointReadOnly {
    // https://dev.w3.org/fxtf/geometry/Overview.html#dom-dompointreadonly-x
    fn X(&self) -> f64 {
        self.x.get()
    }

    // https://dev.w3.org/fxtf/geometry/Overview.html#dom-dompointreadonly-y
    fn Y(&self) -> f64 {
        self.y.get()
    }

    // https://dev.w3.org/fxtf/geometry/Overview.html#dom-dompointreadonly-z
    fn Z(&self) -> f64 {
        self.z.get()
    }

    // https://dev.w3.org/fxtf/geometry/Overview.html#dom-dompointreadonly-w
    fn W(&self) -> f64 {
        self.w.get()
    }
}

pub trait DOMPointWriteMethods {
    fn SetX(&self, value: f64);
    fn SetY(&self, value: f64);
    fn SetZ(&self, value: f64);
    fn SetW(&self, value: f64);
}

impl DOMPointWriteMethods for DOMPointReadOnly {
    fn SetX(&self, value: f64) {
        self.x.set(value);
    }

    fn SetY(&self, value: f64) {
        self.y.set(value);
    }

    fn SetZ(&self, value: f64) {
        self.z.set(value);
    }

    fn SetW(&self, value: f64) {
        self.w.set(value);
    }
}
