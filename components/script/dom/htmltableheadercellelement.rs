/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::HTMLTableHeaderCellElementBinding;
use dom::bindings::js::Root;
use dom::document::Document;
use dom::htmltablecellelement::HTMLTableCellElement;
use dom::node::Node;
use util::str::DOMString;

magic_dom_struct! {
    pub struct HTMLTableHeaderCellElement {
        htmltablecellelement: Base<HTMLTableCellElement>,
    }
}

impl HTMLTableHeaderCellElement {
    fn new_inherited(&mut self, localName: DOMString,
                     prefix: Option<DOMString>,
                     document: &Document) {
        self.htmltablecellelement.new_inherited(localName, prefix, document)
    }

    #[allow(unrooted_must_root)]
    pub fn new(localName: DOMString,
               prefix: Option<DOMString>,
               document: &Document) -> Root<HTMLTableHeaderCellElement> {
        let mut obj = Node::alloc_node::<HTMLTableHeaderCellElement>(document);
        obj.new_inherited(localName, prefix, document);
        obj.into_root()
    }
}
