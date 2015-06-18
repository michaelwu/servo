/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! JS object wrappers
//!
//! This function provides wrappers for common JS types.

use js::jsapi::{JSContext, JSObject};
use js::jsapi::{JS_NewInt8Array, JS_NewUint8Array};
use js::jsapi::{JS_NewInt16Array, JS_NewUint16Array};
use js::jsapi::{JS_NewInt32Array, JS_NewUint32Array};
use js::jsapi::{GetUint8ArrayLengthAndData, GetInt8ArrayLengthAndData};
use js::jsapi::{GetUint16ArrayLengthAndData, GetInt16ArrayLengthAndData};
use js::jsapi::{GetUint32ArrayLengthAndData, GetInt32ArrayLengthAndData};

use dom::bindings::js::{RootCollection, RootCollectionPtr, JS};
use script_task::STACK_ROOTS;
use std::marker::PhantomData;
use std::mem;
use std::slice;
use std::ops::Deref;
use core::nonzero::NonZero;

pub struct JSVec<T> {
    obj: *const *mut JSObject,
    idx: usize,
    root_list: *const RootCollection,
    phantom: PhantomData<T>,
}

trait TypedArrayInt {
    fn alloc(cx: *mut JSContext, len: u32) -> *mut JSObject;
    fn get_data(obj: *mut JSObject) -> (*mut u8, u32);
}

impl TypedArrayInt for u8 {
    fn alloc(cx: *mut JSContext, len: u32) -> *mut JSObject {
        unsafe { JS_NewUint8Array(cx, len) }
    }
    fn get_data(obj: *mut JSObject) -> (*mut u8, u32) {
        unsafe {
            let mut data = mem::zeroed();
            let mut len = 0;
            GetUint8ArrayLengthAndData(obj, &mut len, &mut data);
            (data as *mut u8, len)
        }
    }
}

impl TypedArrayInt for i8 {
    fn alloc(cx: *mut JSContext, len: u32) -> *mut JSObject {
        unsafe { JS_NewInt8Array(cx, len) }
    }
    fn get_data(obj: *mut JSObject) -> (*mut u8, u32) {
        unsafe {
            let mut data = mem::zeroed();
            let mut len = 0;
            GetInt8ArrayLengthAndData(obj, &mut len, &mut data);
            (data as *mut u8, len)
        }
    }
}

impl TypedArrayInt for u16 {
    fn alloc(cx: *mut JSContext, len: u32) -> *mut JSObject {
        unsafe { JS_NewUint16Array(cx, len) }
    }
    fn get_data(obj: *mut JSObject) -> (*mut u8, u32) {
        unsafe {
            let mut data = mem::zeroed();
            let mut len = 0;
            GetUint16ArrayLengthAndData(obj, &mut len, &mut data);
            (data as *mut u8, len)
        }
    }
}

impl TypedArrayInt for i16 {
    fn alloc(cx: *mut JSContext, len: u32) -> *mut JSObject {
        unsafe { JS_NewInt16Array(cx, len) }
    }
    fn get_data(obj: *mut JSObject) -> (*mut u8, u32) {
        unsafe {
            let mut data = mem::zeroed();
            let mut len = 0;
            GetInt16ArrayLengthAndData(obj, &mut len, &mut data);
            (data as *mut u8, len)
        }
    }
}

impl TypedArrayInt for u32 {
    fn alloc(cx: *mut JSContext, len: u32) -> *mut JSObject {
        unsafe { JS_NewUint32Array(cx, len) }
    }
    fn get_data(obj: *mut JSObject) -> (*mut u8, u32) {
        unsafe {
            let mut data = mem::zeroed();
            let mut len = 0;
            GetUint32ArrayLengthAndData(obj, &mut len, &mut data);
            (data as *mut u8, len)
        }
    }
}

impl TypedArrayInt for i32 {
    fn alloc(cx: *mut JSContext, len: u32) -> *mut JSObject {
        unsafe { JS_NewInt32Array(cx, len) }
    }
    fn get_data(obj: *mut JSObject) -> (*mut u8, u32) {
        unsafe {
            let mut data = mem::zeroed();
            let mut len = 0;
            GetInt32ArrayLengthAndData(obj, &mut len, &mut data);
            (data as *mut u8, len)
        }
    }
}

impl<T: TypedArrayInt> JSVec<T> {
/*
    pub fn new(cx: *mut JSContext, len: u32) -> JSVec<T> {
        let obj = T::alloc(cx, len);
        assert!(!obj.is_null());

        STACK_ROOTS.with(|ref collection| {
            let RootCollectionPtr(collection) = collection.get().unwrap();
            let (ptr, idx) = unsafe { (*collection).root(NonZero::new(obj)) };
            JSVec {
                obj: ptr,
                idx: idx,
                root_list: collection,
                phantom: PhantomData,
            }
        })
    }
*/
}

impl<T: TypedArrayInt> Deref for JSVec<T> {
    type Target = [T];

    fn deref<'a>(&'a self) -> &'a [T] {
        unsafe {
            let (data, len) = T::get_data(*self.obj);
            slice::from_raw_parts(data as *mut T as *const T, len as usize)
        }
    }
}
