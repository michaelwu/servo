/^    pub struct \([a-zA-Z0-9]*\) [{]/{
	N
	s/ [{]\n *[}]/;/
}

/^    pub fn new[(]/{
:top
	p
	n
	s/new_uninitialized/new_uninitialized/
	t end
	s/new_initialized/new_initialized/
	t end

	s/^        \([a-z]*\)$/        \1.into_root()/
	t end

	s/^    [}]$/    }/
	t end
	b top
:end
}

s/: RootedVec[<]JS[<][a-zA-Z0-9]*[>][>]//
s/RootedVec[<]JS[<]\([a-zA-Z0-9]*\)[>][>]/RootedVec<\1>/
s/r[(][)][.]reflector[(][)][.]get_jsobject[(][)]/handle()/
s/reflector[(][)][.]get_jsobject[(][)][.]get[(][)]/get_jsobj()/
s/reflector[(][)][.]get_jsobject[(][)]/handle()/
s/borrow_for_layout[(][)][.]clone[(][)]/layout_get()/
s/get_rooted[(][)]/get().map(Root::from_rooted)/

p
