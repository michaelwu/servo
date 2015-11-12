/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::XMLHttpRequestUploadBinding;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::Root;
use dom::bindings::magic::alloc_dom_object;
use dom::xmlhttprequesteventtarget::XMLHttpRequestEventTarget;

magic_dom_struct! {
    pub struct XMLHttpRequestUpload {
        eventtarget: Base<XMLHttpRequestEventTarget>
    }
}

impl XMLHttpRequestUpload {
    fn new_inherited(&mut self) {
        self.eventtarget.new_inherited();
    }
    pub fn new(global: GlobalRef) -> Root<XMLHttpRequestUpload> {
        let mut obj = alloc_dom_object::<XMLHttpRequestUpload>(global);
        obj.new_inherited();
        obj.into_root()
    }
}
