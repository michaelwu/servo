/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::ProcessingInstructionBinding;
use dom::bindings::codegen::Bindings::ProcessingInstructionBinding::ProcessingInstructionMethods;
use dom::bindings::js::Root;
use dom::characterdata::CharacterData;
use dom::document::Document;
use dom::node::Node;
use util::str::DOMString;

/// An HTML processing instruction node.
magic_dom_struct! {
    pub struct ProcessingInstruction {
        characterdata: Base<CharacterData>,
        target: DOMString,
    }
}

impl ProcessingInstruction {
    fn new_inherited(&mut self, target: DOMString, data: DOMString, document: &Document) {
        self.characterdata.new_inherited(data, document);
        self.target.init(target);
    }

    pub fn new(target: DOMString, data: DOMString, document: &Document) -> Root<ProcessingInstruction> {
        let mut obj = Node::alloc_node::<ProcessingInstruction>(document);
        obj.new_inherited(target, data, document);
        obj.into_root()
    }
}


impl ProcessingInstruction {
    pub fn target(&self) -> DOMString {
        self.target.get()
    }
}

impl ProcessingInstructionMethods for ProcessingInstruction {
    // https://dom.spec.whatwg.org/#dom-processinginstruction-target
    fn Target(&self) -> DOMString {
        self.target.get()
    }
}
