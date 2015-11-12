/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::CloseEventBinding;
use dom::bindings::codegen::Bindings::CloseEventBinding::CloseEventMethods;
use dom::bindings::codegen::Bindings::EventBinding::EventMethods;
use dom::bindings::conversions::Castable;
use dom::bindings::error::Fallible;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::Root;
use dom::bindings::magic::alloc_dom_object;
use dom::event::{Event, EventBubbles, EventCancelable};
use script_task::ScriptChan;
use util::str::DOMString;

magic_dom_struct! {
    pub struct CloseEvent {
        event: Base<Event>,
        wasClean: bool,
        code: u16,
        reason: DOMString,
    }
}

impl CloseEvent {
    pub fn new_inherited(&mut self, wasClean: bool, code: u16,
                         reason: DOMString) {
        self.event.new_inherited();
        self.wasClean.init(wasClean);
        self.code.init(code);
        self.reason.init(reason);
    }

    pub fn new(global: GlobalRef,
               type_: DOMString,
               bubbles: EventBubbles,
               cancelable: EventCancelable,
               wasClean: bool,
               code: u16,
               reason: DOMString) -> Root<CloseEvent> {
        let mut ev = alloc_dom_object::<CloseEvent>(global);
        ev.new_inherited(wasClean, code, reason);
        {
            let event = ev.upcast::<Event>();
            event.InitEvent(type_,
                            bubbles == EventBubbles::Bubbles,
                            cancelable == EventCancelable::Cancelable);
        }
        ev.into_root()
    }

    pub fn Constructor(global: GlobalRef,
                       type_: DOMString,
                       init: &CloseEventBinding::CloseEventInit)
                       -> Fallible<Root<CloseEvent>> {
        let bubbles = if init.parent.bubbles { EventBubbles::Bubbles } else { EventBubbles::DoesNotBubble };
        let cancelable = if init.parent.cancelable {
            EventCancelable::Cancelable
        } else {
            EventCancelable::NotCancelable
        };
        Ok(CloseEvent::new(global, type_, bubbles, cancelable, init.wasClean,
                           init.code, init.reason.clone()))
    }
}

impl CloseEventMethods for CloseEvent {
    // https://html.spec.whatwg.org/multipage/#dom-closeevent-wasclean
    fn WasClean(&self) -> bool {
        self.wasClean.get()
    }

    // https://html.spec.whatwg.org/multipage/#dom-closeevent-code
    fn Code(&self) -> u16 {
        self.code.get()
    }

    // https://html.spec.whatwg.org/multipage/#dom-closeevent-reason
    fn Reason(&self) -> DOMString {
        self.reason.get()
    }
}
