/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::NodeBinding::NodeMethods;
use dom::bindings::codegen::Bindings::NodeListBinding;
use dom::bindings::codegen::Bindings::NodeListBinding::NodeListMethods;
use dom::bindings::global::{GlobalRef, global_object_for_dom_object};
use dom::bindings::js::{JS, Root, RootedReference, DOMVec};
use dom::bindings::magic::alloc_dom_object;
use dom::node::{ChildrenMutation, Node, NodeIter};
use dom::window::Window;
use std::cell::Cell;

#[must_root]
pub enum NodeListType {
    Simple(DOMVec<JS<Node>>),
    Children(Root<ChildrenList>),
}

// https://dom.spec.whatwg.org/#interface-nodelist
magic_dom_struct! {
    pub struct NodeList {
        list_type: NodeListType,
    }
}

impl NodeList {
    fn new_inherited(&mut self, list_type: NodeListType) {
        self.list_type.init(list_type);
    }

    pub fn new(window: &Window,
               list_type: NodeListType) -> Root<NodeList> {
        let mut obj = alloc_dom_object::<NodeList>(GlobalRef::Window(window));
        obj.new_inherited(list_type);
        obj.into_root()
    }

    pub fn new_simple_list<T>(window: &Window, iter: T)
                              -> Root<NodeList>
                              where T: Iterator<Item=Root<Node>> {
        NodeList::new(window, NodeListType::Simple(DOMVec::from_iter(GlobalRef::Window(window), iter.map(|r| JS::from_rooted(&r)))))
    }

    pub fn new_child_list(window: &Window, node: &Node) -> Root<NodeList> {
        let list = ChildrenList::new(node);
        NodeList::new(window, NodeListType::Children(list))
    }
}

impl NodeListMethods for NodeList {
    // https://dom.spec.whatwg.org/#dom-nodelist-length
    fn Length(&self) -> u32 {
        match self.list_type.get() {
            NodeListType::Simple(elems) => elems.len() as u32,
            NodeListType::Children(list) => list.len(),
        }
    }

    // https://dom.spec.whatwg.org/#dom-nodelist-item
    fn Item(&self, index: u32) -> Option<Root<Node>> {
        match self.list_type.get() {
            NodeListType::Simple(elems) => {
                elems.get(index).map(|node| node.root())
            },
            NodeListType::Children(list) => list.item(index),
        }
    }

    // https://dom.spec.whatwg.org/#dom-nodelist-item
    fn IndexedGetter(&self, index: u32, found: &mut bool) -> Option<Root<Node>> {
        let item = self.Item(index);
        *found = item.is_some();
        item
    }
}


impl NodeList {
    pub fn as_children_list(&self) -> Root<ChildrenList> {
        if let NodeListType::Children(list) = self.list_type.get() {
            list
        } else {
            panic!("called as_children_list() on a simple node list")
        }
    }
}

magic_dom_struct! {
    pub struct ChildrenList {
        node: JS<Node>,
        last_visited: Mut<Option<JS<Node>>>,
        last_index: Mut<u32>,
    }
}

anonymous_dom_object!(ChildrenList);

impl ChildrenList {
    fn new(node: &Node) -> Root<ChildrenList> {
        let last_visited = node.GetFirstChild();
        let global = global_object_for_dom_object(node);
        let mut obj = alloc_dom_object::<ChildrenList>(global.r());
        obj.node.init(JS::from_ref(node));
        obj.last_visited.init(last_visited.map(|last| JS::from_rooted(&last)));
        obj.last_index.init(0);
        obj.into_root()
    }

    pub fn len(&self) -> u32 {
        self.node.get().root().children_count()
    }

    pub fn item(&self, index: u32) -> Option<Root<Node>> {
        // This always start traversing the children from the closest element
        // among parent's first and last children and the last visited one.
        let len = self.len() as u32;
        if index >= len {
            return None;
        }
        if index == 0u32 {
            // Item is first child if any, not worth updating last visited.
            return self.node.get().root().GetFirstChild();
        }
        let last_index = self.last_index.get();
        if index == last_index {
            // Item is last visited child, no need to update last visited.
            return Some(self.last_visited.get().unwrap().root());
        }
        let last_visited = if index - 1u32 == last_index {
            // Item is last visited's next sibling.
            self.last_visited.get().unwrap().root().GetNextSibling().unwrap()
        } else if last_index > 0 && index == last_index - 1u32 {
            // Item is last visited's previous sibling.
            self.last_visited.get().unwrap().root().GetPreviousSibling().unwrap()
        } else if index > last_index {
            if index == len - 1u32 {
                // Item is parent's last child, not worth updating last visited.
                return Some(self.node.get().root().GetLastChild().unwrap());
            }
            if index <= last_index + (len - last_index) / 2u32 {
                // Item is closer to the last visited child and follows it.
                self.last_visited.get().unwrap().root()
                                 .inclusively_following_siblings()
                                 .nth((index - last_index) as usize).unwrap()
            } else {
                // Item is closer to parent's last child and obviously
                // precedes it.
                self.node.get().root().GetLastChild().unwrap()
                    .inclusively_preceding_siblings()
                    .nth((len - index - 1u32) as usize).unwrap()
            }
        } else if index >= last_index / 2u32 {
            // Item is closer to the last visited child and precedes it.
            self.last_visited.get().unwrap().root()
                             .inclusively_preceding_siblings()
                             .nth((last_index - index) as usize).unwrap()
        } else {
            // Item is closer to parent's first child and obviously follows it.
            debug_assert!(index < last_index / 2u32);
            self.node.get().root().GetFirstChild().unwrap()
                     .inclusively_following_siblings()
                     .nth(index as usize)
                     .unwrap()
        };
        self.last_visited.set(Some(JS::from_rooted(&last_visited)));
        self.last_index.set(index);
        Some(last_visited)
    }

    pub fn children_changed(&self, mutation: &ChildrenMutation) {
        fn prepend(list: &ChildrenList, mut added: NodeIter, next: &Node) {
            let len = added.len() as u32;
            if len == 0u32 {
                return;
            }
            let index = list.last_index.get();
            if index < len {
                list.last_visited.set(added.next_back().map(JS::from_ref));
            } else if index / 2u32 >= len {
                // If last index is twice as large as the number of added nodes,
                // updating only it means that less nodes will be traversed if
                // caller is traversing the node list linearly.
                list.last_index.set(len + index);
            } else {
                // If last index is not twice as large but still larger,
                // it's better to update it to the number of added nodes.
                list.last_visited.set(Some(JS::from_ref(next)));
                list.last_index.set(len);
            }
        }

        fn replace(list: &ChildrenList,
                   prev: Option<&Node>,
                   removed: &Node,
                   mut added: Option<NodeIter>,
                   next: Option<&Node>) {
            let added_len = match added {
                Some(ref added) => added.len(),
                None => 0,
            };
            let index = list.last_index.get();
            if JS::from_ref(removed) == list.last_visited.get().unwrap() {
                let visited = match (prev, added, next) {
                    (None, _, None) => {
                        // Such cases where parent had only one child should
                        // have been changed into ChildrenMutation::ReplaceAll
                        // by ChildrenMutation::replace().
                        unreachable!()
                    },
                    (_, Some(ref mut added), _) => added.next().unwrap(),
                    (_, None, Some(next)) => next,
                    (Some(prev), None, None) => {
                        list.last_index.set(index - 1u32);
                        prev
                    },
                };
                list.last_visited.set(Some(JS::from_ref(visited)));
            } else if added_len != 1 {
                // The replaced child isn't the last visited one, and there are
                // 0 or more than 1 nodes to replace it. Special care must be
                // given to update the state of that ChildrenList.
                match (prev, next) {
                    (Some(_), None) => {},
                    (None, Some(next)) => {
                        list.last_index.set(index - 1);
                        if let Some(added) = added {
                            prepend(list, added, next);
                        }
                    },
                    (Some(_), Some(_)) => {
                        list.reset();
                    },
                    (None, None) => unreachable!(),
                }
            }
        }

        match *mutation {
            ChildrenMutation::Append { .. } => {},
            ChildrenMutation::Insert { .. } => {
                self.reset();
            },
            ChildrenMutation::Prepend { ref added, next } => {
                if let &Some(ref added) = added {
                    prepend(self, added.clone(), next);
                }
            },
            ChildrenMutation::Replace { prev, removed, ref added, next } => {
                if let &Some(ref added) = added {
                    replace(self, prev, removed, Some(added.clone()), next);
                } else {
                    replace(self, prev, removed, None, next);
                }
            },
            ChildrenMutation::ReplaceAll { ref added, .. } => {
                match *added {
                    Some(ref added) => {
                        let mut added = added.clone();
                        let len = added.len() as u32;
                        let index = self.last_index.get();
                        if index < len {
                            self.last_visited.set(added.next_back()
                                                       .map(JS::from_ref));
                        } else {
                            // Setting last visited to parent's last child serves no purpose,
                            // so the middle is arbitrarily chosen here in case the caller
                            // wants random access.
                            let middle = len / 2;
                            self.last_visited.set(added.nth(middle as usize)
                                                       .map(JS::from_ref));
                            self.last_index.set(middle);
                        }
                    },
                    None => {
                        self.last_visited.set(None);
                        self.last_index.set(0u32);
                    },
                };
            },
        }
    }

    fn reset(&self) {
        self.last_visited.set(self.node.get().root().GetFirstChild().as_ref().map(JS::from_rooted));
        self.last_index.set(0u32);
    }
}
