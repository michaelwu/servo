/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::DOMRectListBinding;
use dom::bindings::codegen::Bindings::DOMRectListBinding::DOMRectListMethods;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::DOMVec;
use dom::bindings::js::{JS, Root};
use dom::bindings::magic::alloc_dom_object;
use dom::domrect::DOMRect;
use dom::window::Window;

magic_dom_struct! {
    pub struct DOMRectList {
        rects: DOMVec<JS<DOMRect>>,
    }
}

impl DOMRectList {
    fn new_inherited<T>(&mut self, global: GlobalRef, rects: T)
                        where T: Iterator<Item=Root<DOMRect>> {
        self.rects.init(DOMVec::from_iter(global, rects.map(|r| JS::from_rooted(&r))));
    }

    pub fn new<T>(window: &Window, rects: T) -> Root<DOMRectList>
                  where T: Iterator<Item=Root<DOMRect>> {
        let mut obj = alloc_dom_object::<DOMRectList>(GlobalRef::Window(window));
        obj.new_inherited(GlobalRef::Window(window), rects);
        obj.into_root()
    }
}

impl DOMRectListMethods for DOMRectList {
    // https://drafts.fxtf.org/geometry/#dom-domrectlist-length
    fn Length(&self) -> u32 {
        self.rects.get().len() as u32
    }

    // https://drafts.fxtf.org/geometry/#dom-domrectlist-item
    fn Item(&self, index: u32) -> Option<Root<DOMRect>> {
        let rects = self.rects.get();
        rects.get(index).map(|rect| rect.root())
    }

    // check-tidy: no specs after this line
    fn IndexedGetter(&self, index: u32, found: &mut bool) -> Option<Root<DOMRect>> {
        *found = index < self.rects.get().len() as u32;
        self.Item(index)
    }
}
