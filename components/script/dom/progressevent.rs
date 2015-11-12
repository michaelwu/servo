/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::EventBinding::EventMethods;
use dom::bindings::codegen::Bindings::ProgressEventBinding;
use dom::bindings::codegen::Bindings::ProgressEventBinding::ProgressEventMethods;
use dom::bindings::conversions::Castable;
use dom::bindings::error::Fallible;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::Root;
use dom::bindings::magic::alloc_dom_object;
use dom::event::{Event, EventBubbles, EventCancelable};
use util::str::DOMString;

magic_dom_struct! {
    pub struct ProgressEvent {
        event: Base<Event>,
        length_computable: bool,
        loaded: u64,
        total: u64
    }
}

impl ProgressEvent {
    fn new_inherited(&mut self, length_computable: bool, loaded: u64, total: u64) {
        self.event.new_inherited();
        self.length_computable.init(length_computable);
        self.loaded.init(loaded);
        self.total.init(total);
    }
    pub fn new(global: GlobalRef, type_: DOMString,
               can_bubble: EventBubbles, cancelable: EventCancelable,
               length_computable: bool, loaded: u64, total: u64) -> Root<ProgressEvent> {
        let mut ev = alloc_dom_object::<ProgressEvent>(global);
        ev.new_inherited(length_computable, loaded, total);
        {
            let event = ev.upcast::<Event>();
            event.InitEvent(type_, can_bubble == EventBubbles::Bubbles, cancelable == EventCancelable::Cancelable);
        }
        ev.into_root()
    }
    pub fn Constructor(global: GlobalRef,
                       type_: DOMString,
                       init: &ProgressEventBinding::ProgressEventInit)
                       -> Fallible<Root<ProgressEvent>> {
        let bubbles = if init.parent.bubbles { EventBubbles::Bubbles } else { EventBubbles::DoesNotBubble };
        let cancelable = if init.parent.cancelable { EventCancelable::Cancelable }
                         else { EventCancelable::NotCancelable };
        let ev = ProgressEvent::new(global, type_, bubbles, cancelable,
                                    init.lengthComputable, init.loaded, init.total);
        Ok(ev)
    }
}

impl ProgressEventMethods for ProgressEvent {
    // https://xhr.spec.whatwg.org/#dom-progressevent-lengthcomputable
    fn LengthComputable(&self) -> bool {
        self.length_computable
    }

    // https://xhr.spec.whatwg.org/#dom-progressevent-loaded
    fn Loaded(&self) -> u64 {
        self.loaded
    }

    // https://xhr.spec.whatwg.org/#dom-progressevent-total
    fn Total(&self) -> u64 {
        self.total
    }
}
