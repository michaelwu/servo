/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::cell::DOMRefCell;
use dom::bindings::js::{JS, HeapJS, Root};
use dom::bindings::trace::JSTraceable;
use dom::document::Document;
use dom::window::Window;
use msg::constellation_msg::PipelineId;
use std::cell::Cell;
use std::default::Default;
use std::rc::Rc;
use js::jsapi::JSTracer;

/// Encapsulates a handle to a frame in a frame tree.
#[derive(JSTraceable, HeapSizeOf)]
#[allow(unrooted_must_root)] // FIXME(#6687) this is wrong
pub struct Page {
    /// Pipeline id associated with this page.
    id: PipelineId,

    /// The outermost frame containing the document and window.
    frame: DOMRefCell<Frame>,

    /// Indicates if reflow is required when reloading.
    needs_reflow: Cell<bool>,

    // Child Pages.
    pub children: DOMRefCell<Vec<Rc<Page>>>,
}

pub struct PageIterator {
    stack: Vec<Rc<Page>>,
}

pub trait IterablePage {
    fn iter(&self) -> PageIterator;
    fn find(&self, id: PipelineId) -> Option<Rc<Page>>;
}

impl IterablePage for Rc<Page> {
    fn iter(&self) -> PageIterator {
        PageIterator {
            stack: vec!(self.clone()),
        }
    }
    fn find(&self, id: PipelineId) -> Option<Rc<Page>> {
        if self.id == id { return Some(self.clone()); }
        for page in &*self.children.borrow() {
            let found = page.find(id);
            if found.is_some() { return found; }
        }
        None
    }

}

impl Page {
    pub fn new(id: PipelineId) -> Page {
        Page {
            id: id,
            frame: DOMRefCell::new(Default::default()),
            needs_reflow: Cell::new(true),
            children: DOMRefCell::new(vec!()),
        }
    }

    pub fn pipeline(&self) -> PipelineId {
        self.id
    }

    pub fn window(&self) -> Root<Window> {
        self.frame.borrow().window.get().unwrap().root()
    }

    pub fn document(&self) -> Root<Document> {
        self.frame.borrow().document.get().unwrap().root()
    }

    // must handle root case separately
    pub fn remove(&self, id: PipelineId) -> Option<Rc<Page>> {
        let remove_idx = {
            self.children
                .borrow_mut()
                .iter_mut()
                .position(|page_tree| page_tree.id == id)
        };
        match remove_idx {
            Some(idx) => Some(self.children.borrow_mut().remove(idx)),
            None => {
                self.children
                    .borrow_mut()
                    .iter_mut()
                    .filter_map(|page_tree| page_tree.remove(id))
                    .next()
            }
        }
    }

    #[allow(unsafe_code, unrooted_must_root)]
    pub fn trace_parser(&self, tr: *mut JSTracer) {
        if let Some(doc) = self.frame.borrow().document.get() {
            let doc = doc.root();
            if let Some(parser) = doc.get_current_parser() {
                unsafe {
                    parser.tokenizer().borrow_for_gc_trace().trace(tr);
                }
            }
        }
        for child in &*self.children.borrow() {
            child.trace_parser(tr);
        }
    }
}

impl Iterator for PageIterator {
    type Item = Rc<Page>;

    fn next(&mut self) -> Option<Rc<Page>> {
        match self.stack.pop() {
            Some(next) => {
                for child in &*next.children.borrow() {
                    self.stack.push(child.clone());
                }
                Some(next)
            },
            None => None,
        }
    }
}

impl Page {
    pub fn set_reflow_status(&self, status: bool) -> bool {
        let old = self.needs_reflow.get();
        self.needs_reflow.set(status);
        old
    }

    pub fn set_frame(&self, frame: Option<(&Document, &Window)>) {
        let frame_ref = self.frame.borrow_mut();
        match frame {
            Some((doc, win)) => {
                frame_ref.document.set(Some(JS::from_ref(doc)));
                frame_ref.window.set(Some(JS::from_ref(win)));
            },
            None => {
                frame_ref.document.set(None);
                frame_ref.window.set(None);
            }
        }
    }
}

/// Information for one frame in the browsing context.
#[derive(JSTraceable, Default, HeapSizeOf)]
#[must_root]
pub struct Frame {
    /// The document for this frame.
    pub document: HeapJS<JS<Document>>,
    /// The window object for this frame.
    pub window: HeapJS<JS<Window>>,
}
