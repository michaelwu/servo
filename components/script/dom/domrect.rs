/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use app_units::Au;
use dom::bindings::codegen::Bindings::DOMRectBinding;
use dom::bindings::codegen::Bindings::DOMRectBinding::DOMRectMethods;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::Root;
use dom::bindings::num::Finite;
use dom::bindings::magic::alloc_dom_object;
use dom::window::Window;

magic_dom_struct! {
    pub struct DOMRect {
        top: f32,
        bottom: f32,
        left: f32,
        right: f32,
    }
}

impl DOMRect {
    fn new_inherited(&mut self, top: Au, bottom: Au,
                         left: Au, right: Au) {
        self.top.init(top.to_nearest_px() as f32);
        self.bottom.init(bottom.to_nearest_px() as f32);
        self.left.init(left.to_nearest_px() as f32);
        self.right.init(right.to_nearest_px() as f32);
    }

    pub fn new(window: &Window,
               top: Au, bottom: Au,
               left: Au, right: Au) -> Root<DOMRect> {
        let mut obj = alloc_dom_object::<DOMRect>(GlobalRef::Window(window));
        obj.new_inherited(top, bottom, left, right);
        obj.into_root()
    }
}

impl DOMRectMethods for DOMRect {
    // https://drafts.fxtf.org/geometry/#dom-domrectreadonly-top
    fn Top(&self) -> Finite<f32> {
        Finite::wrap(self.top.get())
    }

    // https://drafts.fxtf.org/geometry/#dom-domrectreadonly-bottom
    fn Bottom(&self) -> Finite<f32> {
        Finite::wrap(self.bottom.get())
    }

    // https://drafts.fxtf.org/geometry/#dom-domrectreadonly-left
    fn Left(&self) -> Finite<f32> {
        Finite::wrap(self.left.get())
    }

    // https://drafts.fxtf.org/geometry/#dom-domrectreadonly-right
    fn Right(&self) -> Finite<f32> {
        Finite::wrap(self.right.get())
    }

    // https://drafts.fxtf.org/geometry/#dom-domrectreadonly-width
    fn Width(&self) -> Finite<f32> {
        let result = (self.right.get() - self.left.get()).abs();
        Finite::wrap(result)
    }

    // https://drafts.fxtf.org/geometry/#dom-domrectreadonly-height
    fn Height(&self) -> Finite<f32> {
        let result = (self.bottom.get() - self.top.get()).abs();
        Finite::wrap(result)
    }
}
