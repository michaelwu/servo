/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! Smart pointers for the JS-managed DOM objects.
//!
//! The DOM is made up of DOM objects whose lifetime is entirely controlled by
//! the whims of the SpiderMonkey garbage collector. The types in this module
//! are designed to ensure that any interactions with said Rust types only
//! occur on values that will remain alive the entire time.
//!
//! Here is a brief overview of the important types:
//!
//! - `Root<T>`: a stack-based reference to a rooted DOM object.
//! - `JS<T>`: a reference to a DOM object that can automatically be traced by
//!   the GC when encountered as a field of a Rust structure.
//!
//! `JS<T>` does not allow access to their inner value without explicitly
//! creating a stack-based root via the `root` method. This returns a `Root<T>`,
//! which causes the JS-owned value to be uncollectable for the duration of the
//! `Root` object's lifetime. A reference to the object can then be obtained
//! from the `Root` object. These references are not allowed to outlive their
//! originating `Root<T>`.
//!

use core::nonzero::NonZero;
use dom::bindings::conversions::{Castable, DerivedFrom};
use dom::bindings::global::GlobalRef;
use dom::bindings::trace::{JSTraceable, trace_unbarriered_object};
use dom::bindings::magic::MagicDOMClass;
use dom::bindings::utils::DOMJSClass;
use dom::node::Node;
use js::jsapi::{JSContext, JSObject, Heap, JSTracer, HandleObject, RootedValue};
use js::jsapi::{JSAutoRequest, JSAutoCompartment, ObjectOpResult, JS_GetReservedSlot};
use js::jsapi::{JS_NewArrayObject1, JS_GetArrayLength, JS_SetArrayLength, JS_GetElement, JS_SetElement};
use js::jsapi::{JS_GetUCProperty, JS_SetUCProperty, JS_HasUCProperty, JS_DeleteUCProperty, JS_NewObject, JS_GetClass};
use js::jsval::{JSVal, UndefinedValue, ObjectValue};
use js::glue::GetProxyExtra;
use layout_interface::TrustedNodeAddress;
use script_task::{STACK_ROOTS, THREAD_JSCTX};
use std::cell::{Cell, UnsafeCell};
use std::default::Default;
use std::marker::PhantomData;
use std::mem;
use std::ops::Deref;
use std::ptr;
use util::mem::HeapSizeOf;

/// Get the JSContext for this thread.
/// Intended for use with the fast conversion code.
pub fn get_tls_jsctx() -> *mut JSContext {
    THREAD_JSCTX.with(|ref r| r.get())
}

/// This trait is used to convert between rust types and JSVals.
/// Conversions here can make assumptions to go faster.
pub trait JSValConversion {
    /// Get this object in the form of a JSVal.
    fn get_jsval(&self) -> JSVal;
    /// Convert from a JSVal into this object.
    fn from_jsval(val: JSVal) -> Self;
}

/// This trait is used to convert between raw JSObjects and wrappers.
pub trait JSObjectConversion {
    /// Get the underlying JSObject of this object.
    fn get_jsobj(&self) -> *mut JSObject;
    /// Generate a `Self` from a raw JSObject.
    fn from_jsobj(obj: *mut JSObject) -> Self;
}

/// A traced reference to a DOM object
///
/// This type is critical to making garbage collection work with the DOM,
/// but it is very dangerous; if garbage collection happens with a `JS<T>`
/// on the stack, the `JS<T>` can point to freed memory.
///
/// This should only be used as a field in other DOM objects.
#[must_root]
pub struct JS<T> {
    ptr: NonZero<*mut JSObject>,
    phantom: PhantomData<T>,
}

// JS<T> is similar to Rc<T>, in that it's not always clear how to avoid double-counting.
// For now, we choose not to follow any such pointers.
impl<T> HeapSizeOf for JS<T> {
    fn heap_size_of_children(&self) -> usize {
        0
    }
}

impl<T: MagicDOMClass> JS<T> {
    /// Root this JS-owned value to prevent its collection as garbage.
    pub fn root(&self) -> Root<T> {
        Root::new(self.ptr)
    }
    /// Create a JS<T> from a Root<T>
    /// XXX Not a great API. Should be a call on Root<T> instead
    #[allow(unrooted_must_root)]
    pub fn from_rooted(root: &Root<T>) -> JS<T> {
        JS::from_jsobj((*root).get_jsobj())
    }
    /// Create a JS<T> from a &T
    #[allow(unrooted_must_root)]
    pub fn from_ref(obj: &T) -> JS<T> {
        JS::from_jsobj(obj.get_jsobj())
    }
    /// Store an rooted value in this field. This is safe under the
    /// assumption that JS<T> values are only used as fields in DOM types that
    /// are reachable in the GC graph, so this unrooted value becomes
    /// transitively rooted for the lifetime of its new owner.
    pub fn assign(&mut self, val: Root<T>) {
        self.ptr = unsafe { NonZero::new(**val.ptr) };
    }

    /// Returns `LayoutJS<T>` containing the same pointer.
    pub unsafe fn to_layout(self) -> LayoutJS<T> {
        LayoutJS {
            ptr: self.ptr.clone(),
            phantom: PhantomData
        }
    }

    /// Used for tracing only.
    pub unsafe fn traceable(&self) -> &*mut JSObject {
        &*self.ptr
    }
}

impl<T: MagicDOMClass> JSObjectConversion for JS<T> {
    fn get_jsobj(&self) -> *mut JSObject {
        *self.ptr
    }

    #[allow(unrooted_must_root)]
    fn from_jsobj(obj: *mut JSObject) -> JS<T> {
        JS {
            ptr: unsafe { NonZero::new(obj) },
            phantom: PhantomData
        }
    }
}

use dom::bindings::conversions::is_dom_proxy;
impl<T: MagicDOMClass> JSValConversion for JS<T> {
    fn get_jsval(&self) -> JSVal {
        let obj = *self.ptr;
        let clasp = unsafe { &*(JS_GetClass(obj) as *const DOMJSClass) };
        if let Some(slot) = clasp.dom_class.proxy_slot {
            unsafe { JS_GetReservedSlot(obj, slot as u32) }
        } else {
            unsafe { ObjectValue(&*obj) }
        }
    }

    #[allow(unrooted_must_root)]
    fn from_jsval(val: JSVal) -> JS<T> {
        let obj = val.to_object();
        if is_dom_proxy(obj) {
            JS {
                ptr: unsafe {
                    NonZero::new(GetProxyExtra(obj, 0).to_object())
                },
                phantom: PhantomData
            }
        } else {
            JS {
                ptr: unsafe { NonZero::new(obj) },
                phantom: PhantomData
            }
        }
    }
}

impl<T: MagicDOMClass> Copy for JS<T> {}

/// HeapJS is used to store pointers to DOM objects in
/// heap allocated objects.
/// This can be used like a Root.
#[must_root]
pub struct HeapJS<T: JSObjectConversion> {
    ptr: Heap<*mut JSObject>,
    phantom: PhantomData<T>,
}

impl<T: JSObjectConversion> HeapJS<T> {
    /// Sets the contents of this HeapJS.
    pub fn set(&self, obj: Option<T>) {
        match obj {
            Some(obj) => self.ptr.set(obj.get_jsobj()),
            None => self.ptr.set(ptr::null_mut()),
        }
    }

    /// Get a copy of the contents.
    pub fn get(&self) -> Option<T> {
        let obj = self.ptr.get();
        if obj.is_null() {
            None
        } else {
            Some(T::from_jsobj(obj))
        }
    }

    /// Gets a pointer to the `Heap<*mut JSObject>` which can be traced.
    pub fn traceable<'a>(&'a self) -> &'a Heap<*mut JSObject> {
        &self.ptr
    }
}

impl<T: JSObjectConversion> Default for HeapJS<T> {
    #[allow(unrooted_must_root)]
    fn default() -> HeapJS<T> {
        HeapJS {
            ptr: Default::default(),
            phantom: PhantomData,
        }
    }
}

impl<T: MagicDOMClass> Deref for HeapJS<JS<T>> {
    type Target = T;
    fn deref<'a>(&'a self) -> &'a T {
        unsafe { mem::transmute(&*self.ptr.handle().ptr) }
    }
}

impl<T: JSObjectConversion> HeapSizeOf for HeapJS<T> {
    fn heap_size_of_children(&self) -> usize {
        0
    }
}

/// An unrooted reference to a DOM object for use in layout. `Layout*Helpers`
/// traits must be implemented on this.
pub struct LayoutJS<T: MagicDOMClass> {
    ptr: NonZero<*mut JSObject>,
    phantom: PhantomData<T>,
}

impl<T: Castable> LayoutJS<T> {
    /// Cast a DOM object root upwards to one of the interfaces it derives from.
    pub fn upcast<U>(&self) -> LayoutJS<U> where U: Castable, T: DerivedFrom<U> {
        unsafe { mem::transmute_copy(self) }
    }

    /// Cast a DOM object downwards to one of the interfaces it might implement.
    pub fn downcast<U>(&self) -> Option<LayoutJS<U>> where U: DerivedFrom<T> {
        unsafe {
            if (*self.unsafe_get()).is::<U>() {
                Some(mem::transmute_copy(self))
            } else {
                None
            }
        }
    }
}

impl<T: MagicDOMClass> LayoutJS<T> {
    /// Get the underlying JSObject.
    pub unsafe fn get_jsobj(&self) -> *mut JSObject {
        *self.ptr
    }
}

impl<T: MagicDOMClass> Copy for LayoutJS<T> {}

impl<T> PartialEq for JS<T> {
    fn eq(&self, other: &JS<T>) -> bool {
        self.ptr == other.ptr
    }
}

impl<T: MagicDOMClass> PartialEq for LayoutJS<T> {
    fn eq(&self, other: &LayoutJS<T>) -> bool {
        self.ptr == other.ptr
    }
}

impl <T: MagicDOMClass> Clone for JS<T> {
    #[inline]
    #[allow(unrooted_must_root)]
    fn clone(&self) -> JS<T> {
        JS {
            ptr: self.ptr.clone(),
            phantom: PhantomData
        }
    }
}

impl <T: MagicDOMClass> Clone for LayoutJS<T> {
    #[inline]
    fn clone(&self) -> LayoutJS<T> {
        LayoutJS {
            ptr: self.ptr.clone(),
            phantom: PhantomData
        }
    }
}

impl LayoutJS<Node> {
    /// Create a new JS-owned value wrapped from an address known to be a
    /// `Node` pointer.
    pub unsafe fn from_trusted_node_address(inner: TrustedNodeAddress)
                                            -> LayoutJS<Node> {
        let TrustedNodeAddress(addr) = inner;
        LayoutJS {
            ptr: NonZero::new(addr),
            phantom: PhantomData
        }
    }
}

impl<T: MagicDOMClass> LayoutJS<T> {
    /// Returns an unsafe pointer to the interior of this JS object. This is
    /// the only method that be safely accessed from layout. (The fact that
    /// this is unsafe is what necessitates the layout wrappers.)
    pub unsafe fn unsafe_get(&self) -> *const T {
        &*self.ptr as *const *mut JSObject as *const T
    }
}

/// Get an `Option<JSRef<T>>` out of an `Option<Root<T>>`
pub trait RootedReference<T> {
    /// Obtain a safe optional reference to the wrapped JS owned-value that
    /// cannot outlive the lifetime of this root.
    fn r(&self) -> Option<&T>;
}

impl<T: MagicDOMClass> RootedReference<T> for Option<Root<T>> {
    fn r(&self) -> Option<&T> {
        self.as_ref().map(|root| root.r())
    }
}

/// Get an `Option<Option<&T>>` out of an `Option<Option<Root<T>>>`
pub trait OptionalRootedReference<T> {
    /// Obtain a safe optional optional reference to the wrapped JS owned-value
    /// that cannot outlive the lifetime of this root.
    fn r(&self) -> Option<Option<&T>>;
}

impl<T: MagicDOMClass> OptionalRootedReference<T> for Option<Option<Root<T>>> {
    fn r(&self) -> Option<Option<&T>> {
        self.as_ref().map(|inner| inner.r())
    }
}

/// A rooting mechanism for DOM objects on the stack.
///
/// See also [*Exact Stack Rooting - Storing a GCPointer on the CStack*]
/// (https://developer.mozilla.org/en-US/docs/Mozilla/Projects/SpiderMonkey/Internals/GC/Exact_Stack_Rooting).
#[no_move]
pub struct RootCollection {
    roots: UnsafeCell<Vec<*mut JSObject>>,
    next_empty_idx: Cell<usize>,
}

/// A pointer to a RootCollection, for use in global variables.
pub struct RootCollectionPtr(pub *const RootCollection);

impl Copy for RootCollectionPtr {}
impl Clone for RootCollectionPtr {
    fn clone(&self) -> RootCollectionPtr { *self }
}

impl RootCollection {
    /// Create an empty collection of roots
    pub fn new() -> RootCollection {
        RootCollection {
            roots: UnsafeCell::new(Vec::with_capacity(4096)),
            next_empty_idx: Cell::new(0),
        }
    }

    /// Start tracking a stack-based root
    fn root(&self, obj: NonZero<*mut JSObject>) -> (*const *mut JSObject, usize) {
        let mut roots = unsafe { &mut *self.roots.get() };
        let len = roots.len();
        let mut next_empty_idx = self.next_empty_idx.get();
        while next_empty_idx < len && !roots[next_empty_idx].is_null() {
            next_empty_idx += 1;
        }
        if next_empty_idx < len {
            roots[next_empty_idx] = *obj;
        } else {
            roots.push(*obj);
        }
        self.next_empty_idx.set(next_empty_idx);
        (&roots[next_empty_idx], next_empty_idx)
    }

    /// Stop tracking a stack-based root, asserting if the obj isn't found
    fn unroot(&self, idx: usize) {
        let mut roots = unsafe { &mut *self.roots.get() };
        let len = roots.len();
        assert!(!roots[idx].is_null());

        roots[idx] = ptr::null_mut();

        let next_empty_idx = self.next_empty_idx.get();
        if (len - 1) != idx {
            if idx < next_empty_idx {
                self.next_empty_idx.set(idx);
            }
            return;
        }

        let mut idx = idx;
        for entry in roots[..idx].iter().rev() {
            if !entry.is_null() {
                break;
            }
            idx -= 1;
        }
        self.next_empty_idx.set(idx);
        roots.truncate(idx);
    }
}

/// SM Callback that traces the rooted reflectors
pub unsafe fn trace_roots(tracer: *mut JSTracer) {
    STACK_ROOTS.with(|ref collection| {
        let RootCollectionPtr(collection) = collection.get().unwrap();
        let collection = &*(*collection).roots.get();
        for root in collection {
            if !root.is_null() {
                trace_unbarriered_object(tracer, "DOM object root collection", root);
            }
        }
    });
}

/// A rooted reference to a DOM object.
///
/// The JS value is pinned for the duration of this object's lifetime; roots
/// are additive, so this object's destruction will not invalidate other roots
/// for the same JS value. `Root`s cannot outlive the associated
/// `RootCollection` object.
pub struct Root<T: MagicDOMClass> {
    /// Reference to rooted value that must not outlive this container
    ptr: NonZero<*const *mut JSObject>,
    /// Index of the the rooted value inside the associated `RootCollection`
    idx: usize,
    /// List that ensures correct dynamic root ordering
    root_list: *const RootCollection,
    /// ?
    phantom: PhantomData<T>,
}

impl<T: Castable> Root<T> {
    /// Cast a DOM object root upwards to one of the interfaces it derives from.
    pub fn upcast<U>(root: Root<T>) -> Root<U> where U: Castable, T: DerivedFrom<U> {
        unsafe { mem::transmute(root) }
    }

    /// Cast a DOM object root downwards to one of the interfaces it might implement.
    pub fn downcast<U>(root: Root<T>) -> Option<Root<U>> where U: DerivedFrom<T> {
        if root.is::<U>() {
            Some(unsafe { mem::transmute(root) })
        } else {
            None
        }
    }
}

impl<T: MagicDOMClass> Root<T> {
    /// Create a new stack-bounded root for the provided JS-owned value.
    /// It cannot not outlive its associated `RootCollection`, and it gives
    /// out references which cannot outlive this new `Root`.
    pub fn new(unrooted: NonZero<*mut JSObject>) -> Root<T> {
        STACK_ROOTS.with(|ref collection| {
            let RootCollectionPtr(collection) = collection.get().unwrap();
            let (ptr, idx) = unsafe { (*collection).root(unrooted) };
            Root {
                ptr: unsafe { NonZero::new(ptr) },
                idx: idx,
                root_list: collection,
                phantom: PhantomData,
            }
        })
    }

    /// Generate a new root from a reference
    pub fn from_ref(unrooted: &T) -> Root<T> {
        unsafe {
            Root::new(NonZero::new(unrooted.get_jsobj()))
        }
    }

    /// Obtain a safe reference to the wrapped JS owned-value that cannot
    /// outlive the lifetime of this root.
    pub fn r(&self) -> &T {
        &self
    }

    /// Don't use this. Don't make me find you.
    pub fn get_unsound_ref_forever<'a, 'b>(&'a self) -> &'b T {
        unsafe { mem::transmute(&**self.ptr) }
    }

    /// Generate a new root from a JS<T> reference
    #[allow(unrooted_must_root)]
    pub fn from_rooted(js: JS<T>) -> Root<T> {
        js.root()
    }

    /// Get a handle to the underlying JSObj
    pub fn handle(&self) -> HandleObject {
        unsafe { HandleObject::from_marked_location(*self.ptr) }
    }
}

impl<T: MagicDOMClass> Deref for Root<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { mem::transmute(&**self.ptr.deref()) }
    }
}

impl<T: MagicDOMClass> PartialEq for Root<T> {
    fn eq(&self, other: &Root<T>) -> bool {
        self.get_jsobj() == other.get_jsobj()
    }
}

impl<T: MagicDOMClass> Drop for Root<T> {
    fn drop(&mut self) {
        unsafe { (*self.root_list).unroot(self.idx); }
    }
}

/// DOMVec is a vector that stores entries in a JS array.
#[allow_unrooted_interior]
pub struct DOMVec<T: JSValConversion> {
    obj: *const *mut JSObject,
    idx: usize,
    cx: *mut JSContext,
    root_list: *const RootCollection,
    phantom: PhantomData<T>,
}

impl<T: JSValConversion> DOMVec<T> {
    /// Generates a new DOMVec with a given minimum capacity.
    pub fn new(global: GlobalRef, len: u32) -> DOMVec<T> {
        let cx = global.get_cx();
        let _ar = JSAutoRequest::new(cx);
        let _ac = JSAutoCompartment::new(cx, global.handle().get());
        let obj = unsafe { JS_NewArrayObject1(cx, len as ::libc::size_t) };
        assert!(!obj.is_null());
        DOMVec::from_jsobject(cx, obj)
    }

    /// Generates a DOMVec from a raw JSObject array.
    pub fn from_jsobject(cx: *mut JSContext, obj: *mut JSObject) -> DOMVec<T> {
        STACK_ROOTS.with(|ref collection| {
            let RootCollectionPtr(collection) = collection.get().unwrap();
            let (ptr, idx) = unsafe { (*collection).root(NonZero::new(obj)) };
            DOMVec {
                obj: ptr,
                idx: idx,
                cx: cx,
                root_list: collection,
                phantom: PhantomData,
            }
        })
    }

    /// Fills a new DOMVec using an iterator.
    pub fn from_iter<J>(global: GlobalRef, iter: J) -> DOMVec<T>
        where J: Iterator<Item=T> {
        let (min_len, _) = iter.size_hint();
        let cx = global.get_cx();
        let _ar = JSAutoRequest::new(cx);
        let _ac = JSAutoCompartment::new(cx, global.handle().get());
        let vec = DOMVec::new(global, min_len as u32);
        let mut idx = 0;
        let mut root = RootedValue::new(cx, UndefinedValue());
        for obj in iter {
            root.ptr = obj.get_jsval();
            unsafe { JS_SetElement(cx, vec.handle(), idx, root.handle()); }
            idx += 1;
        }
        vec
    }

    /// Returns the underlying JSObject.
    pub fn get_jsobj(&self) -> *mut JSObject {
        unsafe { *self.obj }
    }

    /// Returns a HandleObject to the underlying JSObject.
    pub fn handle(&self) -> HandleObject {
        unsafe { HandleObject::from_marked_location(self.obj) }
    }

    /// Returns the length of this DOMVec.
    pub fn len(&self) -> u32 {
        let mut len = 0;
        let _ar = JSAutoRequest::new(self.cx);
        let _ac = JSAutoCompartment::new(self.cx, self.get_jsobj());
        unsafe {
            // XXX check return
            JS_GetArrayLength(self.cx, self.handle(), &mut len);
        }
        len
    }

    /// Returns whether the length is zero.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get an iterator for this DOMVec.
    pub fn iter(&self) -> DOMVecIter<T> {
        DOMVecIter {
            vec: self,
            idx: 0,
        }
    }

    /// Gets the entry at a given index, if it exists.
    pub fn get(&self, idx: u32) -> Option<T> {
        let mut val = RootedValue::new(self.cx, UndefinedValue());
        unsafe {
            let _ar = JSAutoRequest::new(self.cx);
            let _ac = JSAutoCompartment::new(self.cx, self.get_jsobj());
            // XXX check return
            JS_GetElement(self.cx, self.handle(), idx, val.handle_mut());
        }
        if !val.ptr.is_object() {
            None
        } else {
            Some(T::from_jsval(val.ptr))
        }
    }

    /// Sets the entry at a given index.
    pub fn set(&self, idx: u32, obj: T) {
        let val = RootedValue::new(self.cx, obj.get_jsval());
        unsafe {
            let _ar = JSAutoRequest::new(self.cx);
            let _ac = JSAutoCompartment::new(self.cx, self.get_jsobj());
            JS_SetElement(self.cx, self.handle(), idx, val.handle());
        }
    }

    /// Remove the entry at a given index.
    pub fn remove(&self, idx: u32) {
        let len = self.len();
        if len <= 1 {
            self.clear();
            return;
        }

        let _ar = JSAutoRequest::new(self.cx);
        let _ac = JSAutoCompartment::new(self.cx, self.get_jsobj());
        if (idx + 1) == len {
            unsafe { JS_SetArrayLength(self.cx, self.handle(), len - 1); }
            return;
        }

        let mut val = RootedValue::new(self.cx, UndefinedValue());
        // XXX make this shift rather than move in the last element
        unsafe {
            JS_GetElement(self.cx, self.handle(), len - 1, val.handle_mut());
            JS_SetArrayLength(self.cx, self.handle(), len - 1);
            JS_SetElement(self.cx, self.handle(), idx, val.handle());
        }
    }

    /// Puts the given object at the end of the array.
    pub fn push(&self, obj: T) {
        self.set(self.len(), obj);
    }

    /// Inserts an entry at a given index.
    /// XXX make this splice into the array correctly
    pub fn insert(&self, idx: u32, obj: T) {
        self.push(obj);
    }

    /// Truncates the array length to zero.
    pub fn clear(&self) {
        let _ar = JSAutoRequest::new(self.cx);
        let _ac = JSAutoCompartment::new(self.cx, self.get_jsobj());
        unsafe {
            JS_SetArrayLength(self.cx, self.handle(), 0);
        }
    }
}

/// An iterator for `DOMVec`s.
pub struct DOMVecIter<'a, T: JSValConversion + 'a> {
    vec: &'a DOMVec<T>,
    idx: u32,
}

impl<'a, T: JSValConversion + 'a> Iterator for DOMVecIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let ret = self.vec.get(self.idx);
        self.idx += 1;
        ret
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.vec.len() as usize;
        (len, Some(len))
    }
}

impl<T: JSValConversion> JSObjectConversion for DOMVec<T> {
    fn get_jsobj(&self) -> *mut JSObject {
        unsafe { *self.obj }
    }

    fn from_jsobj(obj: *mut JSObject) -> DOMVec<T> {
        DOMVec::from_jsobject(get_tls_jsctx(), obj)
    }
}

impl<T: JSValConversion> Drop for DOMVec<T> {
    fn drop(&mut self) {
        unsafe { (*self.root_list).unroot(self.idx) };
    }
}

struct ObjectGroup;
struct Shape;
type HeapSlot = JSVal;

struct NativeObject {
    group: *const ObjectGroup,
    shape: *const Shape,
    slots: *const HeapSlot,
    elements: *const HeapSlot,
}

struct ObjectElements {
    flags: u32,
    initialized_length: u32,
    capacity: u32,
    length: u32,
}

impl ObjectElements {
    unsafe fn from_elements(elems: *const HeapSlot) -> *const ObjectElements {
        (elems as *const ObjectElements).offset(-1)
    }
}

/// This is like a DOMVec, except it is read only and can be used on the
/// layout thread if appropriate precautions are made to ensure GC doesn't run.
pub struct ReadOnlyDOMVec<T: JSValConversion> {
    obj: *mut NativeObject,
    phantom: PhantomData<T>,
}

impl<'a, T: JSValConversion> ReadOnlyDOMVec<T> {
    /// Create a ReadOnlyDOMVec from a JSObject
    pub fn from_jsobject(obj: *mut JSObject) -> ReadOnlyDOMVec<T> {
        ReadOnlyDOMVec {
            obj: obj as *mut NativeObject,
            phantom: PhantomData,
        }
    }

    /// Get an iterator for this ReadOnlyDOMVec
    pub fn iter(&'a self) -> ReadOnlyDOMVecIter<'a, T> {
        let elements = unsafe { (*self.obj).elements };
        let len = unsafe {
            (*ObjectElements::from_elements(elements)).initialized_length
        };
        ReadOnlyDOMVecIter {
            elements: elements,
            idx: 0,
            len: len,
            phantom: PhantomData,
        }
    }
}

/// An iterator for ReadOnlyDOMVec
pub struct ReadOnlyDOMVecIter<'a, T: JSValConversion + 'a> {
    elements: *const HeapSlot,
    idx: u32,
    len: u32,
    phantom: PhantomData<&'a T>,
}

impl<'a, T: JSValConversion + 'a> Iterator for ReadOnlyDOMVecIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.idx < self.len {
            let ret = unsafe { self.elements.offset(self.idx as isize) };
            self.idx += 1;
            unsafe { Some(T::from_jsval(*ret)) }
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = (self.len - self.idx) as usize;
        (len, Some(len))
    }
}

/// DOMMap is a hashmap that uses JSObjects for storage.
/// The keys are always strings. The entries can be any
/// JSObject based type.
#[allow_unrooted_interior]
pub struct DOMMap<T: JSObjectConversion> {
    obj: *const *mut JSObject,
    idx: usize,
    cx: *mut JSContext,
    root_list: *const RootCollection,
    phantom: PhantomData<T>,
}

impl<T: JSObjectConversion> DOMMap<T> {
    /// Allocate a new DOMMap
    pub fn new(global: GlobalRef) -> DOMMap<T> {
        let cx = global.get_cx();
        let _ar = JSAutoRequest::new(cx);
        let _ac = JSAutoCompartment::new(cx, global.handle().get());
        let obj = unsafe { JS_NewObject(cx, ptr::null()) };
        assert!(!obj.is_null());
        DOMMap::from_jsobject(cx, obj)
    }

    /// Generate a DOMMap from a raw JSObject
    pub fn from_jsobject(cx: *mut JSContext, obj: *mut JSObject) -> DOMMap<T> {
        STACK_ROOTS.with(|ref collection| {
            let RootCollectionPtr(collection) = collection.get().unwrap();
            let (ptr, idx) = unsafe { (*collection).root(NonZero::new(obj)) };
            DOMMap {
                obj: ptr,
                idx: idx,
                cx: cx,
                root_list: collection,
                phantom: PhantomData,
            }
        })
    }

    /// Get a HandleObject for the underlying JSObject
    pub fn handle(&self) -> HandleObject {
        unsafe { HandleObject::from_marked_location(self.obj) }
    }

    /// Get the entry corresponding to the key, if one exists.
    pub fn get(&self, key: &str) -> Option<T> {
        let string_utf16: Vec<u16> = key.utf16_units().collect();
        let mut val = RootedValue::new(self.cx, UndefinedValue());
        let _ar = JSAutoRequest::new(self.cx);
        let _ac = JSAutoCompartment::new(self.cx, self.get_jsobj());
        unsafe {
            JS_GetUCProperty(self.cx, self.handle(), string_utf16.as_ptr(), string_utf16.len() as ::libc::size_t, val.handle_mut());
        }
        if !val.ptr.is_object() {
            None
        } else {
            Some(T::from_jsobj(val.ptr.to_object()))
        }
    }

    /// Remove the entry with a given key
    pub fn remove(&self, key: &str) {
        let string_utf16: Vec<u16> = key.utf16_units().collect();
        let mut result = ObjectOpResult { code_: 0 };
        let _ar = JSAutoRequest::new(self.cx);
        let _ac = JSAutoCompartment::new(self.cx, self.get_jsobj());
        unsafe {
            JS_DeleteUCProperty(self.cx, self.handle(), string_utf16.as_ptr(), string_utf16.len() as ::libc::size_t, &mut result);
        }
    }

    /// Set
    pub fn set(&self, key: &str, val: &T) {
        let string_utf16: Vec<u16> = key.utf16_units().collect();
        let _ar = JSAutoRequest::new(self.cx);
        let _ac = JSAutoCompartment::new(self.cx, self.get_jsobj());
        unsafe {
            let val = RootedValue::new(self.cx, ObjectValue(&*val.get_jsobj()));
            JS_SetUCProperty(self.cx, self.handle(), string_utf16.as_ptr(), string_utf16.len() as ::libc::size_t, val.handle());
        }
    }

    /// Check if there is an entry for a given key.
    pub fn has(&self, key: &str) -> bool {
        let string_utf16: Vec<u16> = key.utf16_units().collect();
        let mut result = false;
        let _ar = JSAutoRequest::new(self.cx);
        let _ac = JSAutoCompartment::new(self.cx, self.get_jsobj());
        unsafe {
            JS_HasUCProperty(self.cx, self.handle(), string_utf16.as_ptr(), string_utf16.len() as ::libc::size_t, &mut result);
        }
        result
    }
}

impl<T: JSObjectConversion> JSObjectConversion for DOMMap<T> {
    fn get_jsobj(&self) -> *mut JSObject {
        unsafe { *self.obj }
    }

    fn from_jsobj(obj: *mut JSObject) -> DOMMap<T> {
        DOMMap::from_jsobject(get_tls_jsctx(), obj)
    }
}

impl<T: JSObjectConversion> Drop for DOMMap<T> {
    fn drop(&mut self) {
        unsafe { (*self.root_list).unroot(self.idx) };
    }
}
