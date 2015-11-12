/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::HTMLTrackElementBinding;
use dom::bindings::js::Root;
use dom::document::Document;
use dom::htmlelement::HTMLElement;
use dom::node::Node;
use util::str::DOMString;

magic_dom_struct! {
    pub struct HTMLTrackElement {
        htmlelement: Base<HTMLElement>,
    }
}

impl HTMLTrackElement {
    fn new_inherited(&mut self, localName: DOMString, prefix: Option<DOMString>, document: &Document) {
        self.htmlelement.new_inherited(localName, prefix, document)
    }

    #[allow(unrooted_must_root)]
    pub fn new(localName: DOMString,
               prefix: Option<DOMString>,
               document: &Document) -> Root<HTMLTrackElement> {
        let mut obj = Node::alloc_node::<HTMLTrackElement>(document);
        obj.new_inherited(localName, prefix, document);
        obj.into_root()
    }
}
