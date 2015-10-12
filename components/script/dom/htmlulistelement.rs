/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::HTMLUListElementBinding;
use dom::bindings::codegen::InheritTypes::{ElementTypeId, EventTargetTypeId, HTMLElementTypeId};
use dom::bindings::codegen::InheritTypes::{HTMLUListElementDerived, NodeTypeId};
use dom::bindings::js::Root;
use dom::bindings::utils::TopDOMClass;
use dom::document::Document;
use dom::eventtarget::EventTarget;
use dom::htmlelement::HTMLElement;
use dom::node::Node;
use util::str::DOMString;

magic_dom_struct! {
    pub struct HTMLUListElement {
        htmlelement: Base<HTMLElement>
    }
}

impl HTMLUListElementDerived for EventTarget {
    fn is_htmlulistelement(&self) -> bool {
        *self.type_id() ==
            EventTargetTypeId::Node(
                NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLUListElement)))
    }
}

impl HTMLUListElement {
    fn new_inherited(&mut self, localName: DOMString, prefix: Option<DOMString>, document: &Document) {
        self.htmlelement.new_inherited(HTMLElementTypeId::HTMLUListElement, localName, prefix, document)
    }

    #[allow(unrooted_must_root)]
    pub fn new(localName: DOMString,
               prefix: Option<DOMString>,
               document: &Document) -> Root<HTMLUListElement> {
        let mut obj = Node::alloc_node::<HTMLUListElement>(document);
        obj.new_inherited(localName, prefix, document);
        obj.into_root()
    }
}
