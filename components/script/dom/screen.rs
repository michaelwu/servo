/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::ScreenBinding;
use dom::bindings::codegen::Bindings::ScreenBinding::ScreenMethods;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::Root;
use dom::bindings::magic::alloc_dom_object;
use dom::window::Window;

magic_dom_struct! {
    pub struct Screen;
}

impl Screen {
    fn new_inherited(&mut self) {
    }

    pub fn new(window: &Window) -> Root<Screen> {
        let mut obj = alloc_dom_object::<Screen>(GlobalRef::Window(window));
        obj.new_inherited();
        obj.into_root()
    }
}

impl ScreenMethods for Screen {
    // https://drafts.csswg.org/cssom-view/#dom-screen-colordepth
    fn ColorDepth(&self) -> u32 {
        24
    }

    // https://drafts.csswg.org/cssom-view/#dom-screen-pixeldepth
    fn PixelDepth(&self) -> u32 {
        24
    }
}
