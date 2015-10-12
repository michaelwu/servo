/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::HTMLLIElementBinding;
use dom::bindings::codegen::InheritTypes::{ElementTypeId, EventTargetTypeId, HTMLElementTypeId};
use dom::bindings::codegen::InheritTypes::{HTMLLIElementDerived, NodeTypeId};
use dom::bindings::js::Root;
use dom::bindings::utils::TopDOMClass;
use dom::document::Document;
use dom::eventtarget::EventTarget;
use dom::htmlelement::HTMLElement;
use dom::node::Node;
use util::str::DOMString;

magic_dom_struct! {
    pub struct HTMLLIElement {
        htmlelement: Base<HTMLElement>,
    }
}

impl HTMLLIElementDerived for EventTarget {
    fn is_htmllielement(&self) -> bool {
        *self.type_id() ==
            EventTargetTypeId::Node(
                NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLLIElement)))
    }
}

impl HTMLLIElement {
    fn new_inherited(&mut self, localName: DOMString, prefix: Option<DOMString>, document: &Document) {
        self.htmlelement.new_inherited(HTMLElementTypeId::HTMLLIElement, localName, prefix, document)
    }

    #[allow(unrooted_must_root)]
    pub fn new(localName: DOMString,
               prefix: Option<DOMString>,
               document: &Document) -> Root<HTMLLIElement> {
        let mut obj = Node::alloc_node::<HTMLLIElement>(document);
        obj.new_inherited(localName, prefix, document);
        obj.into_root()
    }
}
