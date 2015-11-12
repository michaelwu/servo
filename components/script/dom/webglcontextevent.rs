/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::EventBinding::EventMethods;
use dom::bindings::codegen::Bindings::WebGLContextEventBinding;
use dom::bindings::codegen::Bindings::WebGLContextEventBinding::WebGLContextEventInit;
use dom::bindings::codegen::Bindings::WebGLContextEventBinding::WebGLContextEventMethods;
use dom::bindings::conversions::Castable;
use dom::bindings::error::Fallible;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::Root;
use dom::bindings::magic::alloc_dom_object;
use dom::event::{Event, EventBubbles, EventCancelable};
use util::str::DOMString;

magic_dom_struct! {
    pub struct WebGLContextEvent {
        event: Base<Event>,
        status_message: DOMString,
    }
}

impl WebGLContextEventMethods for WebGLContextEvent {
    // https://www.khronos.org/registry/webgl/specs/latest/1.0/#5.15
    fn StatusMessage(&self) -> DOMString {
        self.status_message.clone()
    }
}

impl WebGLContextEvent {
    pub fn new_inherited(&mut self, status_message: DOMString) {
        self.event.new_inherited();
        self.status_message.init(status_message);
    }

    pub fn new(global: GlobalRef,
               type_: DOMString,
               bubbles: EventBubbles,
               cancelable: EventCancelable,
               status_message: DOMString) -> Root<WebGLContextEvent> {
        let mut event = alloc_dom_object::<WebGLContextEvent>(global);
        event.new_inherited(status_message);

        {
            let parent = event.upcast::<Event>();
            parent.InitEvent(type_, bubbles == EventBubbles::Bubbles, cancelable == EventCancelable::Cancelable);
        }

        event.into_root()
    }

    pub fn Constructor(global: GlobalRef,
                       type_: DOMString,
                       init: &WebGLContextEventInit) -> Fallible<Root<WebGLContextEvent>> {
        let status_message = match init.statusMessage.as_ref() {
            Some(message) => message.clone(),
            None => "".to_owned(),
        };

        let bubbles = if init.parent.bubbles {
            EventBubbles::Bubbles
        } else {
            EventBubbles::DoesNotBubble
        };

        let cancelable = if init.parent.cancelable {
            EventCancelable::Cancelable
        } else {
            EventCancelable::NotCancelable
        };

        Ok(WebGLContextEvent::new(global, type_,
                                  bubbles,
                                  cancelable,
                                  status_message))
    }
}
