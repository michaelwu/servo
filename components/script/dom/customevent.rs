/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::CustomEventBinding;
use dom::bindings::codegen::Bindings::CustomEventBinding::CustomEventMethods;
use dom::bindings::codegen::Bindings::EventBinding::EventMethods;
use dom::bindings::codegen::InheritTypes::{CustomEventDerived, EventCast, EventTypeId};
use dom::bindings::error::Fallible;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::{Root};
use dom::bindings::magic::alloc_dom_object;
use dom::bindings::utils::TopDOMClass;
use dom::event::Event;
use js::jsapi::{HandleValue, JSContext};
use js::jsval::{JSVal, UndefinedValue};
use util::str::DOMString;

// https://dom.spec.whatwg.org/#interface-customevent
magic_dom_struct! {
    pub struct CustomEvent {
        event: Base<Event>,
        #[ignore_heap_size_of = "Defined in rust-mozjs"]
        detail: Mut<JSVal>,
    }
}

impl CustomEventDerived for Event {
    fn is_customevent(&self) -> bool {
        *self.type_id() == EventTypeId::CustomEvent
    }
}

impl CustomEvent {
    fn new_inherited(&mut self) {
        self.event.new_inherited();
        self.detail.init(UndefinedValue());
    }

    pub fn new_uninitialized(global: GlobalRef) -> Root<CustomEvent> {
        let mut obj = alloc_dom_object::<CustomEvent>(global);
        obj.new_inherited();
        obj.into_root()
    }
    pub fn new(global: GlobalRef,
               type_: DOMString,
               bubbles: bool,
               cancelable: bool,
               detail: HandleValue) -> Root<CustomEvent> {
        let ev = CustomEvent::new_uninitialized(global);
        ev.r().InitCustomEvent(global.get_cx(), type_, bubbles, cancelable, detail);
        ev
    }
    #[allow(unsafe_code)]
    pub fn Constructor(global: GlobalRef,
                       type_: DOMString,
                       init: &CustomEventBinding::CustomEventInit) -> Fallible<Root<CustomEvent>>{
        Ok(CustomEvent::new(global,
                            type_,
                            init.parent.bubbles,
                            init.parent.cancelable,
                            unsafe { HandleValue::from_marked_location(&init.detail) }))
    }
}

impl CustomEventMethods for CustomEvent {
    // https://dom.spec.whatwg.org/#dom-customevent-detail
    fn Detail(&self, _cx: *mut JSContext) -> JSVal {
        self.detail.get()
    }

    // https://dom.spec.whatwg.org/#dom-customevent-initcustomevent
    fn InitCustomEvent(&self,
                       _cx: *mut JSContext,
                       type_: DOMString,
                       can_bubble: bool,
                       cancelable: bool,
                       detail: HandleValue) {
        let event = EventCast::from_ref(self);
        if event.dispatching() {
            return;
        }

        self.detail.set(detail.get());
        event.InitEvent(type_, can_bubble, cancelable);
    }
}
