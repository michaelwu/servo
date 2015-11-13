/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::cell::DOMRefCell;
use dom::bindings::codegen::Bindings::FormDataBinding;
use dom::bindings::codegen::Bindings::FormDataBinding::FormDataMethods;
use dom::bindings::codegen::UnionTypes::FileOrString;
use dom::bindings::codegen::UnionTypes::FileOrString::{eFile, eString};
use dom::bindings::conversions::Castable;
use dom::bindings::error::{Fallible};
use dom::bindings::global::{GlobalField, GlobalRef};
use dom::bindings::js::{JS, Root, DOMMap, DOMVec};
use dom::bindings::magic::alloc_dom_object;
use dom::blob::Blob;
use dom::file::File;
use dom::htmlformelement::HTMLFormElement;
use std::borrow::ToOwned;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use util::str::DOMString;

#[derive(Clone)]
#[must_root]
#[derive(HeapSizeOf)]
pub enum FormDatum {
    StringData(DOMString),
    FileData(JS<File>)
}

magic_dom_struct! {
    pub struct FormData {
        data: DOMMap<DOMVec<FormDatum>>,
        global: GlobalField,
        form: Option<JS<HTMLFormElement>>
    }
}

impl FormData {
    fn new_inherited(&mut self, form: Option<&HTMLFormElement>, global: GlobalRef) {
        self.data.init(DOMMap::new(global));
        self.global.init(GlobalField::from_rooted(&global));
        self.form.init(form.map(|f| JS::from_ref(f)));
    }

    pub fn new(form: Option<&HTMLFormElement>, global: GlobalRef) -> Root<FormData> {
        let mut obj = alloc_dom_object::<FormData>(global);
        obj.new_inherited(form, global);
        obj.into_root()
    }

    pub fn Constructor(global: GlobalRef, form: Option<&HTMLFormElement>) -> Fallible<Root<FormData>> {
        Ok(FormData::new(form, global))
    }
}

impl FormDataMethods for FormData {
    #[allow(unrooted_must_root)]
    // https://xhr.spec.whatwg.org/#dom-formdata-append
    fn Append(&self, name: DOMString, value: &Blob, filename: Option<DOMString>) {
        let file = FormDatum::FileData(JS::from_rooted(&self.get_file_from_blob(value, filename)));
        let data = self.data.get();
        match data.get(&name) {
            Some(v) => v.push(file),
            None => {
                let global = self.global.get().root();
                let list = DOMVec::new(global.r(), 1);
                list.set(0, file);
                data.set(&name, &list);
            }
        }
    }

    // https://xhr.spec.whatwg.org/#dom-formdata-append
    fn Append_(&self, name: DOMString, value: DOMString) {
        let data = self.data.get();
        match data.get(&name) {
            Some(v) => v.push(FormDatum::StringData(value)),
            None => {
                let global = self.global.get().root();
                let list = DOMVec::new(global.r(), 1);
                list.set(0, FormDatum::StringData(value));
                data.set(&name, &list);
            },
        }
    }

    // https://xhr.spec.whatwg.org/#dom-formdata-delete
    fn Delete(&self, name: DOMString) {
        self.data.get().remove(&name);
    }

    // https://xhr.spec.whatwg.org/#dom-formdata-get
    fn Get(&self, name: DOMString) -> Option<FileOrString> {
        let data = self.data.get();
        match data.get(&name) {
            Some(v) => {
                match v.get(0) {
                    Some(FormDatum::StringData(ref s)) => Some(eString(s.clone())),
                    Some(FormDatum::FileData(ref f)) => Some(eFile(f.root())),
                    None => None,
                }
            },
            None => None,
        }
    }

    // https://xhr.spec.whatwg.org/#dom-formdata-has
    fn Has(&self, name: DOMString) -> bool {
        self.data.get().has(&name)
    }

    // https://xhr.spec.whatwg.org/#dom-formdata-set
    fn Set_(&self, name: DOMString, value: DOMString) {
        let data = self.data.get();
        let global = self.global.get().root();
        let list = DOMVec::new(global.r(), 1);
        list.set(0, FormDatum::StringData(value));
        data.set(&name, &list);
    }

    #[allow(unrooted_must_root)]
    // https://xhr.spec.whatwg.org/#dom-formdata-set
    fn Set(&self, name: DOMString, value: &Blob, filename: Option<DOMString>) {
        let data = self.data.get();
        let global = self.global.get().root();
        let list = DOMVec::new(global.r(), 1);
        let file = FormDatum::FileData(JS::from_rooted(&self.get_file_from_blob(value, filename)));
        list.set(0, file);
        data.set(&name, &list);
    }
}


impl FormData {
    fn get_file_from_blob(&self, value: &Blob, filename: Option<DOMString>) -> Root<File> {
        let global = self.global.get().root();
        let f = value.downcast::<File>();
        let name = filename.unwrap_or(f.map(|inner| inner.name().clone()).unwrap_or("blob".to_owned()));
        File::new(global.r(), value, name)
    }
}
