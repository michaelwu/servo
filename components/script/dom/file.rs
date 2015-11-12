/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::FileBinding;
use dom::bindings::codegen::Bindings::FileBinding::FileMethods;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::Root;
use dom::bindings::magic::alloc_dom_object;
use dom::blob::Blob;
use util::str::DOMString;

magic_dom_struct! {
    pub struct File {
        blob: Base<Blob>,
        name: DOMString,
    }
}

impl File {
    fn new_inherited(&mut self, global: GlobalRef,
                     _file_bits: &Blob, name: DOMString) {
        //TODO: get type from the underlying filesystem instead of "".to_string()
        self.blob.new_inherited(global, None, "");
        self.name.init(name);
        // XXXManishearth Once Blob is able to store data
        // the relevant subfields of file_bits should be copied over
    }

    pub fn new(global: GlobalRef, file_bits: &Blob, name: DOMString) -> Root<File> {
        let mut obj = alloc_dom_object::<File>(global);
        obj.new_inherited(global, file_bits, name);
        obj.into_root()
    }

    pub fn name(&self) -> &DOMString {
        &self.name
    }
}

impl FileMethods for File {
    // https://w3c.github.io/FileAPI/#dfn-name
    fn Name(&self) -> DOMString {
        self.name.clone()
    }
}
