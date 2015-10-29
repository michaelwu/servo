/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::DocumentTypeBinding;
use dom::bindings::codegen::Bindings::DocumentTypeBinding::DocumentTypeMethods;
use dom::bindings::codegen::InheritTypes::{DocumentTypeDerived, EventTargetTypeId};
use dom::bindings::codegen::InheritTypes::{NodeCast, NodeTypeId};
use dom::bindings::codegen::UnionTypes::NodeOrString;
use dom::bindings::error::ErrorResult;
use dom::bindings::js::Root;
use dom::bindings::utils::TopDOMClass;
use dom::document::Document;
use dom::eventtarget::EventTarget;
use dom::node::Node;
use std::borrow::ToOwned;
use util::str::DOMString;

// https://dom.spec.whatwg.org/#documenttype
/// The `DOCTYPE` tag.
magic_dom_struct! {
    pub struct DocumentType {
        node: Base<Node>,
        name: DOMString,
        public_id: DOMString,
        system_id: DOMString,
    }
}

impl DocumentTypeDerived for EventTarget {
    fn is_documenttype(&self) -> bool {
        *self.type_id() == EventTargetTypeId::Node(NodeTypeId::DocumentType)
    }
}

impl DocumentType {
    fn new_inherited(&mut self, name: DOMString,
                         public_id: Option<DOMString>,
                         system_id: Option<DOMString>,
                         document: &Document)
            {
        self.node.new_inherited(NodeTypeId::DocumentType, document);
        self.name.init(name);
        self.public_id.init(public_id.unwrap_or("".to_owned()));
        self.system_id.init(system_id.unwrap_or("".to_owned()));
    }
    #[allow(unrooted_must_root)]
    pub fn new(name: DOMString,
               public_id: Option<DOMString>,
               system_id: Option<DOMString>,
               document: &Document)
               -> Root<DocumentType> {
        let mut obj = Node::alloc_node::<DocumentType>(document);
        obj.new_inherited(name, public_id, system_id, document);
        obj.into_root()
    }

    #[inline]
    pub fn name(&self) -> DOMString {
        self.name.get()
    }

    #[inline]
    pub fn public_id(&self) -> DOMString {
        self.public_id.get()
    }

    #[inline]
    pub fn system_id(&self) -> DOMString {
        self.system_id.get()
    }
}

impl DocumentTypeMethods for DocumentType {
    // https://dom.spec.whatwg.org/#dom-documenttype-name
    fn Name(&self) -> DOMString {
        self.name.get()
    }

    // https://dom.spec.whatwg.org/#dom-documenttype-publicid
    fn PublicId(&self) -> DOMString {
        self.public_id.get()
    }

    // https://dom.spec.whatwg.org/#dom-documenttype-systemid
    fn SystemId(&self) -> DOMString {
        self.system_id.get()
    }

    // https://dom.spec.whatwg.org/#dom-childnode-before
    fn Before(&self, nodes: Vec<NodeOrString>) -> ErrorResult {
        NodeCast::from_ref(self).before(nodes)
    }

    // https://dom.spec.whatwg.org/#dom-childnode-after
    fn After(&self, nodes: Vec<NodeOrString>) -> ErrorResult {
        NodeCast::from_ref(self).after(nodes)
    }

    // https://dom.spec.whatwg.org/#dom-childnode-replacewith
    fn ReplaceWith(&self, nodes: Vec<NodeOrString>) -> ErrorResult {
        NodeCast::from_ref(self).replace_with(nodes)
    }

    // https://dom.spec.whatwg.org/#dom-childnode-remove
    fn Remove(&self) {
        let node = NodeCast::from_ref(self);
        node.remove_self();
    }
}
