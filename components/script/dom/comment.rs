/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::CommentBinding;
use dom::bindings::codegen::Bindings::WindowBinding::WindowMethods;
use dom::bindings::codegen::InheritTypes::{CharacterDataTypeId, CommentDerived};
use dom::bindings::codegen::InheritTypes::{EventTargetTypeId, NodeTypeId};
use dom::bindings::error::Fallible;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::Root;
use dom::bindings::utils::TopDOMClass;
use dom::characterdata::CharacterData;
use dom::document::Document;
use dom::eventtarget::EventTarget;
use dom::node::Node;
use util::str::DOMString;

/// An HTML comment.
magic_dom_struct! {
    pub struct Comment {
        characterdata: Base<CharacterData>,
    }
}

impl CommentDerived for EventTarget {
    fn is_comment(&self) -> bool {
        *self.type_id() == EventTargetTypeId::Node(NodeTypeId::CharacterData(CharacterDataTypeId::Comment))
    }
}

impl Comment {
    fn new_inherited(&mut self, text: DOMString, document: &Document) {
        self.characterdata.new_inherited(CharacterDataTypeId::Comment, text, document)
    }

    pub fn new(text: DOMString, document: &Document) -> Root<Comment> {
        let mut obj = Node::alloc_node::<Comment>(document);
        obj.new_inherited(text, document);
        obj.into_root()
    }

    pub fn Constructor(global: GlobalRef, data: DOMString) -> Fallible<Root<Comment>> {
        let document = global.as_window().Document();
        Ok(Comment::new(data, document.r()))
    }
}
