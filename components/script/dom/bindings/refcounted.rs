/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! A generic, safe mechanism by which DOM objects can be pinned and transferred
//! between tasks (or intra-task for asynchronous events). Akin to Gecko's
//! nsMainThreadPtrHandle, this uses thread-safe reference counting and ensures
//! that the actual SpiderMonkey GC integration occurs on the script task via
//! message passing. Ownership of a `Trusted<T>` object means the DOM object of
//! type T to which it points remains alive. Any other behaviour is undefined.
//! To guarantee the lifetime of a DOM object when performing asynchronous operations,
//! obtain a `Trusted<T>` from that object and pass it along with each operation.
//! A usable pointer to the original DOM object can be obtained on the script task
//! from a `Trusted<T>` via the `to_temporary` method.
//!
//! The implementation of Trusted<T> is as follows:
//! A hashtable resides in the script task, keyed on the pointer to the Rust DOM object.
//! The values in this hashtable are atomic reference counts. When a Trusted<T> object is
//! created or cloned, this count is increased. When a Trusted<T> is dropped, the count
//! decreases. If the count hits zero, a message is dispatched to the script task to remove
//! the entry from the hashmap if the count is still zero. The JS reflector for the DOM object
//! is rooted when a hashmap entry is first created, and unrooted when the hashmap entry
//! is removed.

use core::nonzero::NonZero;
use dom::bindings::js::Root;
use dom::bindings::trace::trace_object;
use dom::bindings::magic::MagicDOMClass;
use js::jsapi::{JSContext, JSTracer, JSObject, Heap};
use libc;
use script_task::{CommonScriptMsg, ScriptChan};
use std::cell::RefCell;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};


#[allow(missing_docs)]  // FIXME
mod dummy {  // Attributes don’t apply through the macro.
    use std::cell::RefCell;
    use std::rc::Rc;
    use super::LiveDOMReferences;
    thread_local!(pub static LIVE_REFERENCES: Rc<RefCell<Option<LiveDOMReferences>>> =
            Rc::new(RefCell::new(None)));
}
pub use self::dummy::LIVE_REFERENCES;


/// A pointer to a Rust DOM object that needs to be destroyed.
pub struct TrustedReference(Arc<Heap<*mut JSObject>>);
unsafe impl Send for TrustedReference {}

/// A safe wrapper around a raw pointer to a DOM object that can be
/// shared among tasks for use in asynchronous operations. The underlying
/// DOM object is guaranteed to live at least as long as the last outstanding
/// `Trusted<T>` instance.
pub struct Trusted<T: MagicDOMClass> {
    objref: Arc<Heap<*mut JSObject>>,
    script_chan: Box<ScriptChan + Send>,
    owner_thread: *const libc::c_void,
    phantom: PhantomData<T>,
}

unsafe impl<T: MagicDOMClass> Send for Trusted<T> {}

impl<T: MagicDOMClass> Trusted<T> {
    /// Create a new `Trusted<T>` instance from an existing DOM pointer. The DOM object will
    /// be prevented from being GCed for the duration of the resulting `Trusted<T>` object's
    /// lifetime.
    pub fn new(_cx: *mut JSContext, ptr: &T, script_chan: Box<ScriptChan + Send>) -> Trusted<T> {
        LIVE_REFERENCES.with(|ref r| {
            let r = r.borrow();
            let live_references = r.as_ref().unwrap();
            let objref = live_references.addref(ptr.get_jsobj());
            Trusted {
                objref: objref,
                script_chan: script_chan.clone(),
                owner_thread: (&*live_references) as *const _ as *const libc::c_void,
                phantom: PhantomData,
            }
        })
    }

    /// Obtain a usable DOM pointer from a pinned `Trusted<T>` value. Fails if used on
    /// a different thread than the original value from which this `Trusted<T>` was
    /// obtained.
    pub fn root(&self) -> Root<T> {
        assert!(LIVE_REFERENCES.with(|ref r| {
            let r = r.borrow();
            let live_references = r.as_ref().unwrap();
            self.owner_thread == (&*live_references) as *const _ as *const libc::c_void
        }));
        unsafe {
            Root::new(NonZero::new(self.objref.get()))
        }
    }
}

// XXX cannot allow cloning on another thread.
impl<T: MagicDOMClass> Clone for Trusted<T> {
    fn clone(&self) -> Trusted<T> {
        Trusted {
            objref: self.objref.clone(),
            script_chan: self.script_chan.clone(),
            owner_thread: self.owner_thread,
            phantom: PhantomData,
        }
    }
}

impl<T: MagicDOMClass> Drop for Trusted<T> {
    fn drop(&mut self) {
        let refcount = Arc::strong_count(&self.objref);
        assert!(refcount > 0);
        if refcount <= 2 {
            // It's possible this send will fail if the script task
            // has already exited. There's not much we can do at this
            // point though.
            let msg = CommonScriptMsg::RefcountCleanup(TrustedReference(self.objref.clone()));
            let _ = self.script_chan.send(msg);
        }
    }
}

/// The set of live, pinned DOM objects that are currently prevented
/// from being garbage collected due to outstanding references.
pub struct LiveDOMReferences {
    // keyed on pointer to Rust DOM object
    table: RefCell<Vec<Arc<Heap<*mut JSObject>>>>
}

impl LiveDOMReferences {
    /// Set up the task-local data required for storing the outstanding DOM references.
    pub fn initialize() {
        LIVE_REFERENCES.with(|ref r| {
            *r.borrow_mut() = Some(LiveDOMReferences {
                table: RefCell::new(Vec::new()),
            })
        });
    }

    fn addref(&self, ptr: *mut JSObject) -> Arc<Heap<*mut JSObject>> {
        let mut table = self.table.borrow_mut();
        if let Some(entry) = table.iter().find(|entry| entry.get() == ptr) {
            return entry.clone();
        }

        let mut refcount: Arc<Heap<*mut JSObject>> = Arc::new(Default::default());
        refcount.set(ptr);
        table.push(refcount.clone());
        refcount
    }

    /// Unpin the given DOM object if its refcount is 1.
    pub fn cleanup(objref: TrustedReference) {
        let TrustedReference(objref) = objref;
        LIVE_REFERENCES.with(|ref r| {
            let r = r.borrow();
            let live_references = r.as_ref().unwrap();
            let mut table = live_references.table.borrow_mut();
            match table.iter().position(|entry| entry.get() == objref.get()) {
                Some(idx) => {
                    if Arc::strong_count(&table[idx]) <= 2 {
                        table.swap_remove(idx);
                    }
                }
                None => {
                    unreachable!("Attempted to remove a non-existant reference");
                }
            }
        })
    }
}

/// A JSTraceDataOp for tracing reflectors held in LIVE_REFERENCES
pub unsafe extern fn trace_refcounted_objects(tracer: *mut JSTracer, _data: *mut libc::c_void) {
    LIVE_REFERENCES.with(|ref r| {
        let r = r.borrow();
        let live_references = r.as_ref().unwrap();
        let table = live_references.table.borrow();
        for obj in &*table {
            trace_object(tracer, "LIVE_REFERENCES", &**obj);
        }
    });
}
