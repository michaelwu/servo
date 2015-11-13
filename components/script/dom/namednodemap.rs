/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::attr::Attr;
use dom::bindings::codegen::Bindings::NamedNodeMapBinding;
use dom::bindings::codegen::Bindings::NamedNodeMapBinding::NamedNodeMapMethods;
use dom::bindings::error::{Error, Fallible};
use dom::bindings::global::GlobalRef;
use dom::bindings::js::{JS, Root};
use dom::bindings::magic::alloc_dom_object;
use dom::bindings::utils::namespace_from_domstring;
use dom::element::Element;
use dom::window::Window;
use string_cache::Atom;
use util::str::DOMString;

magic_dom_struct! {
    pub struct NamedNodeMap {
        owner: JS<Element>,
    }
}

impl NamedNodeMap {
    fn new_inherited(&mut self, elem: &Element) {
        self.owner.init(JS::from_ref(elem));
    }

    pub fn new(window: &Window, elem: &Element) -> Root<NamedNodeMap> {
        let mut obj = alloc_dom_object::<NamedNodeMap>(GlobalRef::Window(window));
        obj.new_inherited(elem);
        obj.into_root()
    }
}

impl NamedNodeMapMethods for NamedNodeMap {
    // https://dom.spec.whatwg.org/#dom-namednodemap-length
    fn Length(&self) -> u32 {
        let owner = self.owner.get().root();
        // FIXME(https://github.com/rust-lang/rust/issues/23338)
        let owner = owner.r();
        let attrs = owner.attrs();
        attrs.len() as u32
    }

    // https://dom.spec.whatwg.org/#dom-namednodemap-item
    fn Item(&self, index: u32) -> Option<Root<Attr>> {
        let owner = self.owner.get().root();
        // FIXME(https://github.com/rust-lang/rust/issues/23338)
        let owner = owner.r();
        let attrs = owner.attrs();
        attrs.get(index).map(|t| t.root())
    }

    // https://dom.spec.whatwg.org/#dom-namednodemap-getnameditem
    fn GetNamedItem(&self, name: DOMString) -> Option<Root<Attr>> {
        let owner = self.owner.get().root();
        // FIXME(https://github.com/rust-lang/rust/issues/23338)
        let owner = owner.r();
        owner.get_attribute_by_name(name)
    }

    // https://dom.spec.whatwg.org/#dom-namednodemap-getnameditemns
    fn GetNamedItemNS(&self, namespace: Option<DOMString>, local_name: DOMString)
                     -> Option<Root<Attr>> {
        let owner = self.owner.get().root();
        // FIXME(https://github.com/rust-lang/rust/issues/23338)
        let owner = owner.r();
        let ns = namespace_from_domstring(namespace);
        owner.get_attribute(&ns, &Atom::from_slice(&local_name))
    }

    // https://dom.spec.whatwg.org/#dom-namednodemap-removenameditem
    fn RemoveNamedItem(&self, name: DOMString) -> Fallible<Root<Attr>> {
        let owner = self.owner.get().root();
        // FIXME(https://github.com/rust-lang/rust/issues/23338)
        let owner = owner.r();
        let name = owner.parsed_name(name);
        owner.remove_attribute_by_name(&name).ok_or(Error::NotFound)
    }

    // https://dom.spec.whatwg.org/#dom-namednodemap-removenameditemns
    fn RemoveNamedItemNS(&self, namespace: Option<DOMString>, local_name: DOMString)
                      -> Fallible<Root<Attr>> {
        let owner = self.owner.get().root();
        // FIXME(https://github.com/rust-lang/rust/issues/23338)
        let owner = owner.r();
        let ns = namespace_from_domstring(namespace);
        owner.remove_attribute(&ns, &Atom::from_slice(&local_name)).ok_or(Error::NotFound)
    }

    // https://dom.spec.whatwg.org/#dom-namednodemap-item
    fn IndexedGetter(&self, index: u32, found: &mut bool) -> Option<Root<Attr>> {
        let item = self.Item(index);
        *found = item.is_some();
        item
    }

    // check-tidy: no specs after this line
    fn NamedGetter(&self, name: DOMString, found: &mut bool) -> Option<Root<Attr>> {
        let item = self.GetNamedItem(name);
        *found = item.is_some();
        item
    }

    fn SupportedPropertyNames(&self) -> Vec<DOMString> {
        // FIXME: unimplemented (https://github.com/servo/servo/issues/7273)
        vec![]
    }
}
