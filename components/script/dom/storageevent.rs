/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::EventBinding::{EventMethods};
use dom::bindings::codegen::Bindings::StorageEventBinding;
use dom::bindings::codegen::Bindings::StorageEventBinding::{StorageEventMethods};
use dom::bindings::conversions::Castable;
use dom::bindings::error::Fallible;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::{JS, Root, RootedReference};
use dom::bindings::magic::alloc_dom_object;
use dom::event::{Event, EventBubbles, EventCancelable};
use dom::storage::Storage;
use util::str::DOMString;

magic_dom_struct! {
    pub struct StorageEvent {
        event: Base<Event>,
        key: Option<DOMString>,
        oldValue: Option<DOMString>,
        newValue: Option<DOMString>,
        url: DOMString,
        storageArea: Mut<Option<JS<Storage>>>
    }
}


impl StorageEvent {
    pub fn new_inherited(&mut self, key: Option<DOMString>,
                         oldValue: Option<DOMString>,
                         newValue: Option<DOMString>,
                         url: DOMString,
                         storageArea: Option<&Storage>) {
        self.event.new_inherited();
        self.key.init(key);
        self.oldValue.init(oldValue);
        self.newValue.init(newValue);
        self.url.init(url);
        self.storageArea.init(storageArea.map(JS::from_ref));
    }

    pub fn new(global: GlobalRef,
               type_: DOMString,
               bubbles: EventBubbles,
               cancelable: EventCancelable,
               key: Option<DOMString>,
               oldValue: Option<DOMString>,
               newValue: Option<DOMString>,
               url: DOMString,
               storageArea: Option<&Storage>) -> Root<StorageEvent> {
        let mut ev = alloc_dom_object::<StorageEvent>(global);
        ev.new_inherited(key, oldValue, newValue,
                                                                    url, storageArea);
        {
            let event = ev.upcast::<Event>();
            event.InitEvent(type_, bubbles == EventBubbles::Bubbles, cancelable == EventCancelable::Cancelable);
        }
        ev.into_root()
    }

    pub fn Constructor(global: GlobalRef,
                       type_: DOMString,
                       init: &StorageEventBinding::StorageEventInit) -> Fallible<Root<StorageEvent>> {
        let key = init.key.clone();
        let oldValue = init.oldValue.clone();
        let newValue = init.newValue.clone();
        let url = init.url.clone();
        let storageArea = init.storageArea.r();
        let bubbles = if init.parent.bubbles { EventBubbles::Bubbles } else { EventBubbles::DoesNotBubble };
        let cancelable = if init.parent.cancelable {
            EventCancelable::Cancelable
        } else {
            EventCancelable::NotCancelable
        };
        let event = StorageEvent::new(global, type_,
                                      bubbles, cancelable,
                                      key, oldValue, newValue,
                                      url, storageArea);
        Ok(event)
    }
}

impl StorageEventMethods for StorageEvent {
    // https://html.spec.whatwg.org/multipage/#dom-storageevent-key
    fn GetKey(&self) -> Option<DOMString> {
        self.key.get()
    }

    // https://html.spec.whatwg.org/multipage/#dom-storageevent-oldvalue
    fn GetOldValue(&self) -> Option<DOMString> {
        self.oldValue.get()
    }

    // https://html.spec.whatwg.org/multipage/#dom-storageevent-newvalue
    fn GetNewValue(&self) -> Option<DOMString> {
        self.newValue.get()
    }

    // https://html.spec.whatwg.org/multipage/#dom-storageevent-url
    fn Url(&self) -> DOMString {
        self.url.get()
    }

    // https://html.spec.whatwg.org/multipage/#dom-storageevent-storagearea
    fn GetStorageArea(&self) -> Option<Root<Storage>> {
        self.storageArea.get().map(Root::from_rooted)
    }
}
