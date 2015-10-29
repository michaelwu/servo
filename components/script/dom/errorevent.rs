/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::cell::DOMRefCell;
use dom::bindings::codegen::Bindings::ErrorEventBinding;
use dom::bindings::codegen::Bindings::ErrorEventBinding::ErrorEventMethods;
use dom::bindings::codegen::Bindings::EventBinding::EventMethods;
use dom::bindings::codegen::InheritTypes::{ErrorEventDerived, EventCast, EventTypeId};
use dom::bindings::error::Fallible;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::{Root};
use dom::bindings::trace::JSTraceable;
use dom::bindings::magic::alloc_dom_object;
use dom::bindings::utils::TopDOMClass;
use dom::event::{Event, EventBubbles, EventCancelable};
use js::jsapi::{HandleValue, JSContext};
use js::jsval::{JSVal, UndefinedValue};
use std::borrow::ToOwned;
use std::cell::Cell;
use util::str::DOMString;

magic_dom_struct! {
    pub struct ErrorEvent {
        event: Base<Event>,
        message: Layout<DOMString>,
        filename: Layout<DOMString>,
        lineno: Mut<u32>,
        colno: Mut<u32>,
        #[ignore_heap_size_of = "Defined in rust-mozjs"]
        error: Mut<JSVal>,
    }
}

impl ErrorEventDerived for Event {
    fn is_errorevent(&self) -> bool {
        *self.type_id() == EventTypeId::ErrorEvent
    }
}

impl ErrorEvent {
    fn new_inherited(&mut self) {
        self.event.new_inherited();
        self.message.init("".to_owned());
        self.filename.init("".to_owned());
        self.lineno.init(0);
        self.colno.init(0);
        self.error.init(UndefinedValue());
    }

    pub fn new_uninitialized(global: GlobalRef) -> Root<ErrorEvent> {
        let mut obj = alloc_dom_object::<ErrorEvent>(global);
        obj.new_inherited();
        obj.into_root()
    }

    pub fn new(global: GlobalRef,
               type_: DOMString,
               bubbles: EventBubbles,
               cancelable: EventCancelable,
               message: DOMString,
               filename: DOMString,
               lineno: u32,
               colno: u32,
               error: HandleValue) -> Root<ErrorEvent> {
        let ev = ErrorEvent::new_uninitialized(global);
        {
            let event = EventCast::from_ref(ev.r());
            event.InitEvent(type_, bubbles == EventBubbles::Bubbles,
                            cancelable == EventCancelable::Cancelable);
            ev.message.set(message);
            ev.filename.set(filename);
            ev.lineno.set(lineno);
            ev.colno.set(colno);
        }
        ev.error.set(error.get());
        ev
    }

    #[allow(unsafe_code)]
    pub fn Constructor(global: GlobalRef,
                       type_: DOMString,
                       init: &ErrorEventBinding::ErrorEventInit) -> Fallible<Root<ErrorEvent>>{
        let msg = match init.message.as_ref() {
            Some(message) => message.clone(),
            None => "".to_owned(),
        };

        let file_name = match init.filename.as_ref() {
            None => "".to_owned(),
            Some(filename) => filename.clone(),
        };

        let line_num = init.lineno.unwrap_or(0);

        let col_num = init.colno.unwrap_or(0);

        let bubbles = if init.parent.bubbles { EventBubbles::Bubbles } else { EventBubbles::DoesNotBubble };

        let cancelable = if init.parent.cancelable {
            EventCancelable::Cancelable
        } else {
            EventCancelable::NotCancelable
        };

        let event = ErrorEvent::new(global, type_,
                                bubbles, cancelable,
                                msg, file_name,
                                line_num, col_num,
                                unsafe { HandleValue::from_marked_location(&init.error) });
        Ok(event)
    }

}

impl ErrorEventMethods for ErrorEvent {
    // https://html.spec.whatwg.org/multipage/#dom-errorevent-lineno
    fn Lineno(&self) -> u32 {
        self.lineno.get()
    }

    // https://html.spec.whatwg.org/multipage/#dom-errorevent-colno
    fn Colno(&self) -> u32 {
        self.colno.get()
    }

    // https://html.spec.whatwg.org/multipage/#dom-errorevent-message
    fn Message(&self) -> DOMString {
        self.message.get()
    }

    // https://html.spec.whatwg.org/multipage/#dom-errorevent-filename
    fn Filename(&self) -> DOMString {
        self.filename.get()
    }

    // https://html.spec.whatwg.org/multipage/#dom-errorevent-error
    fn Error(&self, _cx: *mut JSContext) -> JSVal {
        self.error.get()
    }

}
