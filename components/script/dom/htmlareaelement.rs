/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::attr::AttrValue;
use dom::bindings::codegen::Bindings::HTMLAreaElementBinding;
use dom::bindings::codegen::Bindings::HTMLAreaElementBinding::HTMLAreaElementMethods;
use dom::bindings::conversions::Castable;
use dom::bindings::js::{JS, Root};
use dom::document::Document;
use dom::domtokenlist::DOMTokenList;
use dom::htmlelement::HTMLElement;
use dom::node::Node;
use dom::virtualmethods::VirtualMethods;
use std::default::Default;
use string_cache::Atom;
use util::str::DOMString;

magic_dom_struct! {
    pub struct HTMLAreaElement {
        htmlelement: Base<HTMLElement>,
        rel_list: Mut<Option<JS<DOMTokenList>>>,
    }
}

impl HTMLAreaElement {
    fn new_inherited(&mut self, localName: DOMString, prefix: Option<DOMString>, document: &Document) {
        self.htmlelement.new_inherited(localName, prefix, document);
        self.rel_list.init(Default::default());
    }

    #[allow(unrooted_must_root)]
    pub fn new(localName: DOMString,
               prefix: Option<DOMString>,
               document: &Document) -> Root<HTMLAreaElement> {
        let mut obj = Node::alloc_node::<HTMLAreaElement>(document);
        obj.new_inherited(localName, prefix, document);
        obj.into_root()
    }
}

impl VirtualMethods for HTMLAreaElement {
    fn super_type(&self) -> Option<&VirtualMethods> {
        Some(self.upcast::<HTMLElement>() as &VirtualMethods)
    }

    fn parse_plain_attribute(&self, name: &Atom, value: DOMString) -> AttrValue {
        match name {
            &atom!("rel") => AttrValue::from_serialized_tokenlist(value),
            _ => self.super_type().unwrap().parse_plain_attribute(name, value),
        }
    }
}

impl HTMLAreaElementMethods for HTMLAreaElement {
    // https://html.spec.whatwg.org/multipage/#dom-area-rellist
    fn RelList(&self) -> Root<DOMTokenList> {
        self.rel_list.or_init(|| {
            DOMTokenList::new(self.upcast(), &atom!("rel"))
        })
    }
}
