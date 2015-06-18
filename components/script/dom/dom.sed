# keep (mostly/partially) on heap:
# Attr
# CanvasGradient
# CanvasRenderingContext2D
# DedicatedWorkerGlobalScope?
# WorkerGlobalScope
# Window
# HTMLCollection
# Document
# URLSearchParams

# special handling
# EventTarget::handlers
# FormData
# HTMLInputElement
# HTMLTextAreaElement::textinput
# Trusted

# turn into Arc<>
# Trusted<T>/TrustedWorkerAddress

s/use dom::bindings::utils::[{]namespace_from_domstring, Reflect\(or\|able\), reflect_dom_object[}];/use dom::bindings::utils::namespace_from_domstring;\nuse dom::bindings::magic::alloc_dom_object;/
s/use dom::bindings::utils::[{]Reflector, \?reflect_dom_object[}];/use dom::bindings::magic::alloc_dom_object;/
s/use dom::bindings::utils::[{]Reflectable, \?reflect_dom_object[}];/use dom::bindings::magic::alloc_dom_object;/
s/use dom::bindings::utils::[{]Reflectable, namespace_from_domstring, reflect_dom_object[}];/use dom::bindings::magic::alloc_dom_object;\nuse dom::bindings::utils::namespace_from_domstring;/
s/use dom::bindings::utils::[{]namespace_from_domstring, reflect_dom_object[}];/use dom::bindings::magic::alloc_dom_object;\nuse dom::bindings::utils::namespace_from_domstring;/
s/use dom::bindings::utils::[{]Reflector, namespace_from_domstring, reflect_dom_object[}];/use dom::bindings::magic::alloc_dom_object;\nuse dom::bindings::utils::namespace_from_domstring;/
s/use dom::bindings::utils::[{]\?reflect_dom_object[}]\?;/use dom::bindings::magic::alloc_dom_object;/
s/use dom::bindings::utils::[{]Reflectable, Reflector, reflect_dom_object[}];/use dom::bindings::magic::alloc_dom_object;/
s/use dom::bindings::utils::[{]reflect_dom_object, Reflectable[}];/use dom::bindings::magic::alloc_dom_object;/
s/use dom::bindings::utils::[{]Reflectable, TopDOMClass, reflect_dom_object[}];/use dom::bindings::magic::alloc_dom_object;\nuse dom::bindings::utils::TopDOMClass;/
s/use dom::bindings::utils::[{]TopDOMClass, reflect_dom_object[}];/use dom::bindings::magic::alloc_dom_object;\nuse dom::bindings::utils::TopDOMClass;/
/use dom::bindings::utils::[{]\?Reflector[}]\?;/D
/use dom::bindings::utils::[{]\?Reflectable[}]\?;/D
/use dom::bindings::utils::[{]Reflectable, Reflector[}];/D
s/, MutNullableHeap//
s/, MutHeapJSVal//
s/MutHeapJSVal, //
s/, MutHeap//
s/Reflectable, //
s/, Reflectable//
s/global_object_for_reflector/global_object_for_dom_object/
s/[+] Reflectable/+ MagicDOMClass/
s/T[:] Reflectable/T: MagicDOMClass/

/^\#\[dom_struct\]/{
:real_top
	n
	s/[#].derive.HeapSizeOf..//
	t real_top
	s/\(.*\)/magic_dom_struct\! \{\n    \1/
	p
	n
:top
	s/    reflector_: Reflector,//
	t skip_line
	s/    reflector_: Reflector//
	t skip_line

	# things that need to be annotated.. maybe
	s/event: Event/event: Base<Event>/
	t top
	s/element: Element/element: Base<Element>/
	t top
	s/htmlelement: HTMLElement/htmlelement: Base<HTMLElement>/
	t top
	s/eventtarget: EventTarget/eventtarget: Base<EventTarget>/
	t top
	s/uievent: UIEvent/uievent: Base<UIEvent>/
	t top
	s/htmlmediaelement: HTMLMediaElement/htmlmediaelement: Base<HTMLMediaElement>/
	t top
	s/characterdata: CharacterData/characterdata: Base<CharacterData>/
	t top
	s/eventtarget: XMLHttpRequestEventTarget/eventtarget: Base<XMLHttpRequestEventTarget>/
	t top
	s/blob: Blob/blob: Base<Blob>/
	t top
	s/htmltablecellelement: HTMLTableCellElement/htmltablecellelement: Base<HTMLTableCellElement>/
	t top
	s/webgl_object: WebGLObject/webgl_object: Base<WebGLObject>/
	t top
	s/workerglobalscope: WorkerGlobalScope/workerglobalscope: Base<WorkerGlobalScope>/
	t top
	s/node: Node/node: Base<Node>/
	t top
	s/point: DOMPointReadOnly/point: Base<DOMPointReadOnly>/
	t top
	s/rect: DOMRectReadOnly/rect: Base<DOMRectReadOnly>/
	t top

	# things we can handle
	s/\([a-zA-Z0-9_]*\): Cell[<]\([a-zA-Z0-9\<\>, \(\)+]*\)[>]/\1: Mut<\2>/
	s/\([a-zA-Z0-9_]*\): UnsafeCell[<]\([a-zA-Z0-9\<\>]*\)[>]/\1: Mut<\2>/
	s/\([a-zA-Z0-9_]*\): DOMRefCell[<]\([a-zA-Z0-9\<\>, \(\)+]*\)[>]/\1: Layout<\2>/
	s/\([a-zA-Z0-9_]*\): RefCell[<]\([a-zA-Z0-9\<\>, \(\)]*\)[>]/\1: Layout<\2>/
	s/\([a-zA-Z0-9_]*\): MutNullableHeap[<]JS[<]\([a-zA-Z0-9]*\)[>][>]/\1: Mut<Option\<JS\<\2\>\>>/
	s/\([a-zA-Z0-9_]*\): MutHeap[<]JS[<]\([a-zA-Z0-9]*\)[>][>]/\1: Mut<JS\<\2\>>/
	s/Heap[<]JSVal[>]/JSVal/
	s/MutHeapJSVal/Mut<JSVal>/
	s/Heap[<][*]mut JSObject[>]/\*mut JSObject/
	s/: MutNullableHeap[<]CanvasContext[>]/: Mut<Option<CanvasContext>>/

	# Vec -> JSVec (JS Array)
	s/Vec[<]JS[<]\([a-zA-Z0-9]*\)[>][>]/DOMVec\<JS\<\1\>\>/
	# Vec -> JSVec (Typed array)
	# s/Vec[<]\([ui]8\)[>]/JSVec\<\1\>/
	# s/Vec[<]\([ui]16\)[>]/JSVec\<\1\>/
	# s/Vec[<]\(i32\)[>]/JSVec\<\1\>/
	# s/Vec[<]\(f32\)[>]/JSVec\<\1\>/
	# s/Vec[<]\(f64\)[>]/JSVec\<\1\>/

	s/    /        /
	p
:skip_line
	n
	s/^\}$/    \}\n\}/
	t end
	b top
:end
}

/fn new_\(inherited\|(\)/{
	s/fn new_inherited[<]T[>][(]/fn new_inherited<T>\(\&mut self, /
	s/fn new_inherited[(]/fn new_inherited\(\&mut self, /
	s/fn new_inherited_with_state[(]/fn new_inherited_with_state\(\&mut self, /
	s/fn new_[(]/fn new_\(\&mut self, /
	s/, [)]/\)/
	s/[-][>] [a-zA-Z0-9]\+ [{]/\{/
	p
	n
:ni_top

	s/ [-][>] [a-zA-Z0-9]\+//

	/^        [a-zA-Z0-9]\+ [{]$/{
		n
	:ni_block_top
		s/Reflector[:][:]new//
		t ni_block_skip

		s/^            extra: box /            self.extra.init\(box /
		t ni_box_fields
		b ni_normal_fields

	:ni_box_fields
                s/    //
		p
		n
		s/^            },\?$/        \}\)\;/
		t ni_block_done

		b ni_box_fields

	:ni_normal_fields
		/html[a-z]*:$/N
		s/^            \([a-zA-Z0-9_]*\):[ \n]*\([a-zA-Z]*\)::new_inherited[(]/        self.\1.new_inherited\(/
		t ni_block_done
		s/^            \([a-zA-Z0-9_]*\):[ \n]*\([a-zA-Z]*\)::new_inherited_with_state[(]/        self.\1.new_inherited_with_state\(/
		t ni_block_done
		s/^            \([a-zA-Z0-9_]*\): \([]a-zA-Z0-9_:()[".& !|=-]*\),\?$/        self.\1.init\(\2\)\;/
		s/^            \([a-zA-Z0-9_]*\): \([]a-zA-Z0-9_:()[".,& !|=-]*[)]\),$/        self.\1.init\(\2\)\;/
		s/^            \([\/]\{2\}.*\)/        \1/

	:ni_block_done
		s/[)],$/\)\;/
		s/MutNullableHeap::new[(]\(.*\)[)]/\1/
		s/MutHeap::new[(]\(.*\)[)]/\1/
		s/DOMRefCell::new[(]\(.*\)[)]/\1/
		s/RefCell::new[(]\(.*\)[)]/\1/
		s/Cell::new[(]\(.*\)[)]/\1/
		s/MutHeapJSVal::new[(][)]/UndefinedValue()/
		p

	:ni_block_skip
		n
		s/^\(        }\)$/\1/
		t ni_skip
		b ni_block_top
	:ni_block_end
	}

	p

:ni_skip
	n
	s/^\(    }\)$/\1/
	t ni_end
	b ni_top
:ni_end
}

/ reflect_dom_object[(]/{
:rdo_top
	s/Wrap[)]/Wrap\)/
	t rdo_finish
	N
	b rdo_top
:rdo_finish

	s/^        reflect_dom_object\([(]\).*box \([a-zA-Z0-9]*\)::new_inherited\([(][a-zA-Z0-9_.,:()*& \n-]*[)]\),[\n ]*\([a-zA-Z0-9().&*:_-]*\),.*/        let mut obj = alloc_dom_object::\<\2\>\1\4\);\n        obj.new_inherited\3;\n        obj.into_root\(\)/
	t rdo_end
	s/let \([a-zA-Z0-9_-]*\) = reflect_dom_object\([(]\).*box \([a-zA-Z0-9]*\)::new_inherited\([(][a-zA-Z0-9_.,:*& \n-]*[)]\),[\n ]*\([a-zA-Z0-9()&:_-]*\),.*/let mut \1 = alloc_dom_object::\<\3\>\2\5\);\n        \1.new_inherited\4;/

:rdo_end
}

/ Some(reflect_dom_object[(]/{
:srdo_top
	s/Wrap[)]/Wrap\)/
	t srdo_finish
	N
	b srdo_top
:srdo_finish
	s/^        Some[(]reflect_dom_object\([(]\).*box \([a-zA-Z0-9]*\)::new_inherited\([(][a-zA-Z0-9_.,:()*& \n-]*[)]\),[\n ]*\([a-zA-Z0-9().&*:_-]*\),.*/        let mut obj = alloc_dom_object::\<\2\>\1\4\);\n        obj.new_inherited\3;\n        Some\(obj.into_root\(\)\)/

:srdo_end
}

/let element = [a-zA-Z]*::new_inherited[(]/{
	N
	s/let element = \([a-zA-Z]*\)::new_inherited[(]\([a-zA-Z0-9:, _-]*\)[)];\n\( *\)Node::reflect_node[(]box element, document, [a-zA-Z]*::Wrap[)]/let mut obj = Node::alloc_node::<\1>\(document\);\n\3obj.new_inherited\(\2\);\n\3obj/
}

p
