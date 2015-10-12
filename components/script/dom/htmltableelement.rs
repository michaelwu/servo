/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use cssparser::RGBA;
use dom::attr::{Attr, AttrValue};
use dom::bindings::codegen::Bindings::HTMLTableElementBinding;
use dom::bindings::codegen::Bindings::HTMLTableElementBinding::HTMLTableElementMethods;
use dom::bindings::codegen::Bindings::NodeBinding::NodeMethods;
use dom::bindings::codegen::InheritTypes::{ElementCast, ElementTypeId, EventTargetTypeId};
use dom::bindings::codegen::InheritTypes::{HTMLElementCast, HTMLElementTypeId};
use dom::bindings::codegen::InheritTypes::{HTMLTableCaptionElementCast, HTMLTableElementDerived};
use dom::bindings::codegen::InheritTypes::{HTMLTableSectionElementDerived, NodeCast, NodeTypeId};
use dom::bindings::js::{Root, RootedReference};
use dom::bindings::utils::TopDOMClass;
use dom::document::Document;
use dom::element::AttributeMutation;
use dom::eventtarget::EventTarget;
use dom::htmlelement::HTMLElement;
use dom::htmltablecaptionelement::HTMLTableCaptionElement;
use dom::htmltablesectionelement::HTMLTableSectionElement;
use dom::node::{Node, document_from_node};
use dom::virtualmethods::VirtualMethods;
use std::cell::Cell;
use string_cache::Atom;
use util::str::{self, DOMString, LengthOrPercentageOrAuto};

magic_dom_struct! {
    pub struct HTMLTableElement {
        htmlelement: Base<HTMLElement>,
        background_color: Mut<Option<RGBA>>,
        border: Mut<Option<u32>>,
        cellspacing: Mut<Option<u32>>,
        width: Mut<LengthOrPercentageOrAuto>,
    }
}

impl HTMLTableElementDerived for EventTarget {
    fn is_htmltableelement(&self) -> bool {
        *self.type_id() ==
            EventTargetTypeId::Node(
                NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTableElement)))
    }
}

impl HTMLTableElement {
    fn new_inherited(&mut self, localName: DOMString, prefix: Option<DOMString>, document: &Document)
                     {
        self.htmlelement.new_inherited(HTMLElementTypeId::HTMLTableElement,
                                                    localName,
                                                    prefix,
                                                    document);
        self.background_color.init(None);
        self.border.init(None);
        self.cellspacing.init(None);
        self.width.init(LengthOrPercentageOrAuto::Auto);
    }

    #[allow(unrooted_must_root)]
    pub fn new(localName: DOMString, prefix: Option<DOMString>, document: &Document)
               -> Root<HTMLTableElement> {
        let mut obj = Node::alloc_node::<HTMLTableElement>(document);
        obj.new_inherited(localName, prefix, document);
        obj.into_root()
    }
}

impl HTMLTableElementMethods for HTMLTableElement {
    // https://html.spec.whatwg.org/multipage/#dom-table-caption
    fn GetCaption(&self) -> Option<Root<HTMLTableCaptionElement>> {
        let node = NodeCast::from_ref(self);
        node.children()
            .filter_map(|c| {
                HTMLTableCaptionElementCast::to_ref(c.r()).map(Root::from_ref)
            })
            .next()
    }

    // https://html.spec.whatwg.org/multipage/#dom-table-caption
    fn SetCaption(&self, new_caption: Option<&HTMLTableCaptionElement>) {
        let node = NodeCast::from_ref(self);

        if let Some(ref caption) = self.GetCaption() {
            NodeCast::from_ref(caption.r()).remove_self();
        }

        if let Some(caption) = new_caption {
            assert!(node.InsertBefore(NodeCast::from_ref(caption),
                                      node.GetFirstChild().as_ref().map(|n| n.r())).is_ok());
        }
    }

    // https://html.spec.whatwg.org/multipage/#dom-table-createcaption
    fn CreateCaption(&self) -> Root<HTMLElement> {
        let caption = match self.GetCaption() {
            Some(caption) => caption,
            None => {
                let caption = HTMLTableCaptionElement::new("caption".to_owned(),
                                                           None,
                                                           document_from_node(self).r());
                self.SetCaption(Some(caption.r()));
                caption
            }
        };
        HTMLElementCast::from_root(caption)
    }

    // https://html.spec.whatwg.org/multipage/#dom-table-deletecaption
    fn DeleteCaption(&self) {
        if let Some(caption) = self.GetCaption() {
            NodeCast::from_ref(caption.r()).remove_self();
        }
    }

    // https://html.spec.whatwg.org/multipage/#dom-table-createtbody
    fn CreateTBody(&self) -> Root<HTMLTableSectionElement> {
        let tbody = HTMLTableSectionElement::new("tbody".to_owned(),
                                                 None,
                                                 document_from_node(self).r());
        let node = NodeCast::from_ref(self);
        let last_tbody =
            node.rev_children()
                .filter_map(ElementCast::to_root)
                .find(|n| n.is_htmltablesectionelement() && n.local_name() == &atom!("tbody"));
        let reference_element =
            last_tbody.and_then(|t| NodeCast::from_root(t).GetNextSibling());

        assert!(node.InsertBefore(NodeCast::from_ref(tbody.r()),
                                  reference_element.r()).is_ok());
        tbody
    }

    // https://html.spec.whatwg.org/multipage/#dom-table-bgcolor
    make_getter!(BgColor);

    // https://html.spec.whatwg.org/multipage/#dom-table-bgcolor
    make_setter!(SetBgColor, "bgcolor");
}


impl HTMLTableElement {
    pub fn get_background_color(&self) -> Option<RGBA> {
        self.background_color.get()
    }

    pub fn get_border(&self) -> Option<u32> {
        self.border.get()
    }

    pub fn get_cellspacing(&self) -> Option<u32> {
        self.cellspacing.get()
    }

    pub fn get_width(&self) -> LengthOrPercentageOrAuto {
        self.width.get()
    }
}

impl VirtualMethods for HTMLTableElement {
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
            &atom!(border) => {
                // According to HTML5 § 14.3.9, invalid values map to 1px.
                self.border.set(mutation.new_value(attr).map(|value| {
                    str::parse_unsigned_integer(value.chars()).unwrap_or(1)
                }));
            }
            &atom!(cellspacing) => {
                self.cellspacing.set(mutation.new_value(attr).and_then(|value| {
                    str::parse_unsigned_integer(value.chars())
                }));
            },
            &atom!(width) => {
                let width = mutation.new_value(attr).map(|value| {
                    str::parse_length(&value)
                });
                self.width.set(width.unwrap_or(LengthOrPercentageOrAuto::Auto));
            },
            _ => {},
        }
    }

    fn parse_plain_attribute(&self, local_name: &Atom, value: DOMString) -> AttrValue {
        match local_name {
            &atom!("border") => AttrValue::from_u32(value, 1),
            _ => self.super_type().unwrap().parse_plain_attribute(local_name, value),
        }
    }
}
