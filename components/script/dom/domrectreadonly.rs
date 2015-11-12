/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::DOMRectReadOnlyBinding::DOMRectReadOnlyMethods;
use dom::bindings::error::Fallible;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::Root;
use dom::bindings::magic::alloc_dom_object;
use std::cell::Cell;

magic_dom_struct! {
    pub struct DOMRectReadOnly {
        x: Mut<f64>,
        y: Mut<f64>,
        width: Mut<f64>,
        height: Mut<f64>,
    }
}

impl DOMRectReadOnly {
    pub fn new_inherited(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.x.init(x);
        self.y.init(y);
        self.width.init(width);
        self.height.init(height);
    }

    pub fn new(global: GlobalRef, x: f64, y: f64, width: f64, height: f64) -> Root<DOMRectReadOnly> {
        let mut obj = alloc_dom_object::<DOMRectReadOnly>(global);
        obj.new_inherited(x, y, width, height);
        obj.into_root()
    }

    pub fn Constructor(global: GlobalRef,
                        x: f64, y: f64, width: f64, height: f64) -> Fallible<Root<DOMRectReadOnly>> {
        Ok(DOMRectReadOnly::new(global, x, y, width, height))
    }

    pub fn set_x(&self, value: f64) {
        self.x.set(value);
    }

    pub fn set_y(&self, value: f64) {
        self.y.set(value);
    }

    pub fn set_width(&self, value: f64) {
        self.width.set(value);
    }

    pub fn set_height(&self, value: f64) {
        self.height.set(value);
    }
}

impl DOMRectReadOnlyMethods for DOMRectReadOnly {
    // https://drafts.fxtf.org/geometry/#dom-domrectreadonly-x
    fn X(&self) -> f64 {
        self.x.get()
    }

    // https://drafts.fxtf.org/geometry/#dom-domrectreadonly-y
    fn Y(&self) -> f64 {
        self.y.get()
    }

    // https://drafts.fxtf.org/geometry/#dom-domrectreadonly-width
    fn Width(&self) -> f64 {
        self.width.get()
    }

    // https://drafts.fxtf.org/geometry/#dom-domrectreadonly-height
    fn Height(&self) -> f64 {
        self.height.get()
    }

    // https://drafts.fxtf.org/geometry/#dom-domrectreadonly-top
    fn Top(&self) -> f64 {
        let height = self.height.get();
        if height >= 0f64 { self.y.get() } else { self.y.get() + height }
    }

    // https://drafts.fxtf.org/geometry/#dom-domrectreadonly-right
    fn Right(&self) -> f64 {
        let width = self.width.get();
        if width < 0f64 { self.x.get() } else { self.x.get() + width }
    }

    // https://drafts.fxtf.org/geometry/#dom-domrectreadonly-bottom
    fn Bottom(&self) -> f64 {
        let height = self.height.get();
        if height < 0f64 { self.y.get() } else { self.y.get() + height }
    }

    // https://drafts.fxtf.org/geometry/#dom-domrectreadonly-left
    fn Left(&self) -> f64 {
        let width = self.width.get();
        if width >= 0f64 { self.x.get() } else { self.x.get() + width }
    }
}
