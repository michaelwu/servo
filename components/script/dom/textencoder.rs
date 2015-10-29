/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::TextEncoderBinding;
use dom::bindings::codegen::Bindings::TextEncoderBinding::TextEncoderMethods;
use dom::bindings::error::Error::Range;
use dom::bindings::error::Fallible;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::Root;
use dom::bindings::str::USVString;
use dom::bindings::magic::alloc_dom_object;
use encoding::label::encoding_from_whatwg_label;
use encoding::types::EncodingRef;
use encoding::{EncoderTrap, Encoding};
use js::jsapi::{JSContext, JSObject};
use js::jsapi::{JS_GetUint8ArrayData, JS_NewUint8Array};
use libc::uint8_t;
use std::borrow::ToOwned;
use std::ptr;
use util::str::DOMString;

magic_dom_struct! {
    pub struct TextEncoder {
        encoding: DOMString,
        #[ignore_heap_size_of = "Defined in rust-encoding"]
        encoder: EncodingRef,
    }
}

impl TextEncoder {
    fn new_inherited(&mut self, encoding: DOMString, encoder: EncodingRef) {
        self.encoding.init(encoding);
        self.encoder.init(encoder);
    }

    pub fn new(global: GlobalRef, encoding: DOMString, encoder: EncodingRef) -> Root<TextEncoder> {
        let mut obj = alloc_dom_object::<TextEncoder>(global);
        obj.new_inherited(encoding, encoder);
        obj.into_root()
    }

    // https://encoding.spec.whatwg.org/#dom-textencoder
    pub fn Constructor(global: GlobalRef,
                       label: DOMString) -> Fallible<Root<TextEncoder>> {
        let encoding = match encoding_from_whatwg_label(&label) {
            Some(enc) => enc,
            None => {
                debug!("Encoding Label Not Supported");
                return Err(Range("The given encoding is not supported.".to_owned()))
            }
        };

        match encoding.name() {
            "utf-8" | "utf-16be" | "utf-16le" => {
                Ok(TextEncoder::new(global, encoding.name().to_owned(), encoding))
            }
            _ => {
                debug!("Encoding Not UTF");
                Err(Range("The encoding must be utf-8, utf-16le, or utf-16be.".to_owned()))
            }
        }
    }
}

impl TextEncoderMethods for TextEncoder {
    // https://encoding.spec.whatwg.org/#dom-textencoder-encoding
    fn Encoding(&self) -> DOMString {
        self.encoding.get()
    }

    #[allow(unsafe_code)]
    // https://encoding.spec.whatwg.org/#dom-textencoder-encode
    fn Encode(&self, cx: *mut JSContext, input: USVString) -> *mut JSObject {
        unsafe {
            let encoded = self.encoder.get().encode(&input.0, EncoderTrap::Strict).unwrap();
            let length = encoded.len() as u32;
            let js_object: *mut JSObject = JS_NewUint8Array(cx, length);

            let js_object_data: *mut uint8_t = JS_GetUint8ArrayData(js_object, ptr::null());
            ptr::copy_nonoverlapping(encoded.as_ptr(), js_object_data, length as usize);
            js_object
        }
    }
}
