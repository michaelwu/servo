/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use cssparser::RGBA;
use dom::attr::Attr;
use dom::bindings::codegen::Bindings::HTMLTableRowElementBinding::{self, HTMLTableRowElementMethods};
use dom::bindings::codegen::Bindings::NodeBinding::NodeMethods;
use dom::bindings::conversions::Castable;
use dom::bindings::error::{ErrorResult, Fallible};
use dom::bindings::js::{JS, Root, RootedReference};
use dom::document::Document;
use dom::element::{AttributeMutation, Element};
use dom::htmlcollection::{CollectionFilter, HTMLCollection};
use dom::htmlelement::HTMLElement;
use dom::htmltabledatacellelement::HTMLTableDataCellElement;
use dom::htmltableheadercellelement::HTMLTableHeaderCellElement;
use dom::node::{Node, window_from_node};
use dom::virtualmethods::VirtualMethods;
use std::cell::Cell;
use util::str::{self, DOMString};


#[derive(JSTraceable)]
struct CellsFilter;
impl CollectionFilter for CellsFilter {
    fn filter(&self, elem: &Element, root: &Node) -> bool {
        (elem.is::<HTMLTableHeaderCellElement>() || elem.is::<HTMLTableDataCellElement>())
            && elem.upcast::<Node>().GetParentNode().r() == Some(root)
    }
}

magic_dom_struct! {
    pub struct HTMLTableRowElement {
        htmlelement: Base<HTMLElement>,
        cells: Mut<Option<JS<HTMLCollection>>>,
        background_color: Layout<Option<RGBA>>,
    }
}

impl HTMLTableRowElement {
    fn new_inherited(&mut self, localName: DOMString, prefix: Option<DOMString>, document: &Document)
                     {
        self.htmlelement.new_inherited(localName, prefix, document);
        self.cells.init(Default::default());
        self.background_color.init(None);
    }

    #[allow(unrooted_must_root)]
    pub fn new(localName: DOMString, prefix: Option<DOMString>, document: &Document)
               -> Root<HTMLTableRowElement> {
        let mut obj = Node::alloc_node::<HTMLTableRowElement>(document);
        obj.new_inherited(localName, prefix, document);
        obj.into_root()
    }

    pub fn get_background_color(&self) -> Option<RGBA> {
        self.background_color.layout_get()
    }
}

impl HTMLTableRowElementMethods for HTMLTableRowElement {
    // https://html.spec.whatwg.org/multipage/#dom-tr-bgcolor
    make_getter!(BgColor);

    // https://html.spec.whatwg.org/multipage/#dom-tr-bgcolor
    make_setter!(SetBgColor, "bgcolor");

    // https://html.spec.whatwg.org/multipage/#dom-tr-cells
    fn Cells(&self) -> Root<HTMLCollection> {
        self.cells.or_init(|| {
            let window = window_from_node(self);
            let filter = box CellsFilter;
            HTMLCollection::create(window.r(), self.upcast(), filter)
        })
    }

    // https://html.spec.whatwg.org/multipage/#dom-tr-insertcell
    fn InsertCell(&self, index: i32) -> Fallible<Root<HTMLElement>> {
        let node = self.upcast::<Node>();
        node.insert_cell_or_row(
            index,
            || self.Cells(),
            || HTMLTableDataCellElement::new("td".to_owned(), None, node.owner_doc().r()))
    }

    // https://html.spec.whatwg.org/multipage/#dom-tr-deletecell
    fn DeleteCell(&self, index: i32) -> ErrorResult {
        let node = self.upcast::<Node>();
        node.delete_cell_or_row(
            index,
            || self.Cells(),
            |n| n.is::<HTMLTableDataCellElement>())
    }
}

impl VirtualMethods for HTMLTableRowElement {
    fn super_type(&self) -> Option<&VirtualMethods> {
        Some(self.upcast::<HTMLElement>() as &VirtualMethods)
    }

    fn attribute_mutated(&self, attr: &Attr, mutation: AttributeMutation) {
        self.super_type().unwrap().attribute_mutated(attr, mutation);
        match attr.local_name() {
            &atom!(bgcolor) => {
                self.background_color.set(mutation.new_value(attr).and_then(|value| {
                    str::parse_legacy_color(&value).ok()
                }));
            },
            _ => {},
        }
    }
}
