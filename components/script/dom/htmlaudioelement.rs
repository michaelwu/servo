/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::HTMLAudioElementBinding;
use dom::bindings::codegen::InheritTypes::{ElementTypeId, EventTargetTypeId};
use dom::bindings::codegen::InheritTypes::{HTMLAudioElementDerived, HTMLElementTypeId};
use dom::bindings::codegen::InheritTypes::{HTMLMediaElementTypeId, NodeTypeId};
use dom::bindings::js::Root;
use dom::bindings::utils::TopDOMClass;
use dom::document::Document;
use dom::eventtarget::EventTarget;
use dom::htmlmediaelement::HTMLMediaElement;
use dom::node::Node;
use util::str::DOMString;

magic_dom_struct! {
    pub struct HTMLAudioElement {
        htmlmediaelement: Base<HTMLMediaElement>
    }
}

impl HTMLAudioElementDerived for EventTarget {
    fn is_htmlaudioelement(&self) -> bool {
        *self.type_id() == EventTargetTypeId::Node(NodeTypeId::Element(
                                                   ElementTypeId::HTMLElement(
                                                   HTMLElementTypeId::HTMLMediaElement(
                                                   HTMLMediaElementTypeId::HTMLAudioElement))))
    }
}

impl HTMLAudioElement {
    fn new_inherited(&mut self, localName: DOMString,
                     prefix: Option<DOMString>,
                     document: &Document) {
        self.htmlmediaelement.new_inherited(HTMLMediaElementTypeId::HTMLAudioElement, localName, prefix, document)
    }

    #[allow(unrooted_must_root)]
    pub fn new(localName: DOMString,
               prefix: Option<DOMString>,
               document: &Document) -> Root<HTMLAudioElement> {
        let mut obj = Node::alloc_node::<HTMLAudioElement>(document);
        obj.new_inherited(localName, prefix, document);
        obj.into_root()
    }
}
