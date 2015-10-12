/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::HTMLVideoElementBinding;
use dom::bindings::codegen::InheritTypes::{ElementTypeId, EventTargetTypeId};
use dom::bindings::codegen::InheritTypes::{HTMLElementTypeId, HTMLMediaElementTypeId};
use dom::bindings::codegen::InheritTypes::{HTMLVideoElementDerived, NodeTypeId};
use dom::bindings::js::Root;
use dom::bindings::utils::TopDOMClass;
use dom::document::Document;
use dom::eventtarget::EventTarget;
use dom::htmlmediaelement::HTMLMediaElement;
use dom::node::Node;
use util::str::DOMString;

magic_dom_struct! {
    pub struct HTMLVideoElement {
        htmlmediaelement: Base<HTMLMediaElement>
    }
}

impl HTMLVideoElementDerived for EventTarget {
    fn is_htmlvideoelement(&self) -> bool {
        *self.type_id() == EventTargetTypeId::Node(NodeTypeId::Element(
                                                   ElementTypeId::HTMLElement(
                                                   HTMLElementTypeId::HTMLMediaElement(
                                                   HTMLMediaElementTypeId::HTMLVideoElement))))
    }
}

impl HTMLVideoElement {
    fn new_inherited(&mut self, localName: DOMString, prefix: Option<DOMString>, document: &Document) {
        self.htmlmediaelement.new_inherited(HTMLMediaElementTypeId::HTMLVideoElement, localName, prefix, document)
    }

    #[allow(unrooted_must_root)]
    pub fn new(localName: DOMString,
               prefix: Option<DOMString>,
               document: &Document) -> Root<HTMLVideoElement> {
        let mut obj = Node::alloc_node::<HTMLVideoElement>(document);
        obj.new_inherited(localName, prefix, document);
        obj.into_root()
    }
}
