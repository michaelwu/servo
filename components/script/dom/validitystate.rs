/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::ValidityStateBinding;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::Root;
use dom::bindings::magic::alloc_dom_object;
use dom::window::Window;

// https://html.spec.whatwg.org/multipage/#validitystate
magic_dom_struct! {
    pub struct ValidityState {
        state: u8,
    }
}

impl ValidityState {
    fn new_inherited(&mut self) {
        self.state.init(0);
    }

    pub fn new(window: &Window) -> Root<ValidityState> {
        let mut obj = alloc_dom_object::<ValidityState>(GlobalRef::Window(window));
        obj.new_inherited();
        obj.into_root()
    }
}
