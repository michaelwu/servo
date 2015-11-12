/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::cell::DOMRefCell;
use dom::bindings::codegen::Bindings::HTMLDialogElementBinding;
use dom::bindings::codegen::Bindings::HTMLDialogElementBinding::HTMLDialogElementMethods;
use dom::bindings::js::Root;
use dom::document::Document;
use dom::htmlelement::HTMLElement;
use dom::node::Node;
use std::borrow::ToOwned;
use util::str::DOMString;

magic_dom_struct! {
    pub struct HTMLDialogElement {
        htmlelement: Base<HTMLElement>,
        return_value: Layout<DOMString>,
    }
}

impl HTMLDialogElement {
    fn new_inherited(&mut self, localName: DOMString,
                     prefix: Option<DOMString>,
                     document: &Document) {
        self.htmlelement.new_inherited(localName, prefix, document);
        self.return_value.init("".to_owned());
    }

    #[allow(unrooted_must_root)]
    pub fn new(localName: DOMString,
               prefix: Option<DOMString>,
               document: &Document) -> Root<HTMLDialogElement> {
        let mut obj = Node::alloc_node::<HTMLDialogElement>(document);
        obj.new_inherited(localName, prefix, document);
        obj.into_root()
    }
}

impl HTMLDialogElementMethods for HTMLDialogElement {
    // https://html.spec.whatwg.org/multipage/#dom-dialog-open
    make_bool_getter!(Open);

    // https://html.spec.whatwg.org/multipage/#dom-dialog-open
    make_bool_setter!(SetOpen, "open");

    // https://html.spec.whatwg.org/multipage/#dom-dialog-returnvalue
    fn ReturnValue(&self) -> DOMString {
        let return_value = self.return_value.get();
        return_value.clone()
    }

    // https://html.spec.whatwg.org/multipage/#dom-dialog-returnvalue
    fn SetReturnValue(&self, return_value: DOMString) {
        self.return_value.set(return_value);
    }
}
