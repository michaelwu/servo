/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::FileListBinding;
use dom::bindings::codegen::Bindings::FileListBinding::FileListMethods;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::{JS, Root};
use dom::bindings::js::DOMVec;
use dom::bindings::magic::alloc_dom_object;
use dom::file::File;
use dom::window::Window;

// https://w3c.github.io/FileAPI/#dfn-filelist
magic_dom_struct! {
    pub struct FileList {
        list: DOMVec<JS<File>>
    }
}

impl FileList {
    fn new_inherited(&mut self, files: DOMVec<JS<File>>) {
        self.list.init(files);
    }

    pub fn new(window: &Window, files: DOMVec<JS<File>>) -> Root<FileList> {
        let mut obj = alloc_dom_object::<FileList>(GlobalRef::Window(window));
        obj.new_inherited(files);
        obj.into_root()
    }
}

impl FileListMethods for FileList {
    // https://w3c.github.io/FileAPI/#dfn-length
    fn Length(&self) -> u32 {
        self.list.get().len() as u32
    }

    // https://w3c.github.io/FileAPI/#dfn-item
    fn Item(&self, index: u32) -> Option<Root<File>> {
        self.list.get().get(index).map(|item| item.root())
    }

    // check-tidy: no specs after this line
    fn IndexedGetter(&self, index: u32, found: &mut bool) -> Option<Root<File>> {
        let item = self.Item(index);
        *found = item.is_some();
        item
    }
}
