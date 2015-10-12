/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use cssparser::RGBA;
use dom::attr::Attr;
use dom::bindings::codegen::Bindings::HTMLTableRowElementBinding::{self, HTMLTableRowElementMethods};
use dom::bindings::codegen::Bindings::NodeBinding::NodeMethods;
use dom::bindings::codegen::InheritTypes::{ElementTypeId, EventTargetTypeId, HTMLElementCast};
use dom::bindings::codegen::InheritTypes::{HTMLElementTypeId, HTMLTableDataCellElementDerived};
use dom::bindings::codegen::InheritTypes::{HTMLTableHeaderCellElementDerived, HTMLTableRowElementDerived};
use dom::bindings::codegen::InheritTypes::{NodeCast, NodeTypeId};
use dom::bindings::js::{JS, Root, RootedReference};
use dom::bindings::utils::TopDOMClass;
use dom::document::Document;
use dom::element::{AttributeMutation, Element};
use dom::eventtarget::EventTarget;
use dom::htmlcollection::{CollectionFilter, HTMLCollection};
use dom::htmlelement::HTMLElement;
use dom::node::{Node, window_from_node};
use dom::virtualmethods::VirtualMethods;
use std::cell::Cell;
use util::str::{self, DOMString};


#[derive(JSTraceable)]
struct CellsFilter;
impl CollectionFilter for CellsFilter {
    fn filter(&self, elem: &Element, root: &Node) -> bool {
        (elem.is_htmltableheadercellelement() || elem.is_htmltabledatacellelement())
            && NodeCast::from_ref(elem).GetParentNode().r() == Some(root)
    }
}

magic_dom_struct! {
    pub struct HTMLTableRowElement {
        htmlelement: Base<HTMLElement>,
        cells: Mut<Option<JS<HTMLCollection>>>,
        background_color: Mut<Option<RGBA>>,
    }
}

impl HTMLTableRowElementDerived for EventTarget {
    fn is_htmltablerowelement(&self) -> bool {
        *self.type_id() ==
            EventTargetTypeId::Node(
                NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTableRowElement)))
    }
}

impl HTMLTableRowElement {
    fn new_inherited(&mut self, localName: DOMString, prefix: Option<DOMString>, document: &Document)
                     {
        self.htmlelement.new_inherited(HTMLElementTypeId::HTMLTableRowElement,
                                                    localName,
                                                    prefix,
                                                    document);
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
        self.background_color.get()
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
            HTMLCollection::create(window.r(), NodeCast::from_ref(self), filter)
        })
    }
}

impl VirtualMethods for HTMLTableRowElement {
    fn super_type<'b>(&'b self) -> Option<&'b VirtualMethods> {
        let htmlelement: &HTMLElement = HTMLElementCast::from_ref(self);
        Some(htmlelement as &VirtualMethods)
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
