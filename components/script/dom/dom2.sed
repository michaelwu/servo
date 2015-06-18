
/magic_dom_struct[!] [{]/{
:pre_top
	n
	s/pub struct [a-zA-Z0-9]*;//
	t end
	s/pub struct //
	t start
	b pre_top
:start
	n
:top
	s/: Base[<]//
	t skip
	s/: Mut[<]//
	t skip
	s/: Box[<]//
	t skip
	s/: Layout[<]/: Layout</
	t layout_mode
	s/: [*]mut JSObject//
	t skip
	s/: JSVal//
	t skip
	s/        \/\///
	t skip
	s/ *\/\/.*//

	h

	s/ *\([a-zA-Z_0-9-]*\): [a-zA-Z0-9<>() ,_-]*,\?$/s|self[.]\1[.]init[(]|self.\1.init(|\nt skip/
	p
	g
	s/ *\([a-zA-Z_0-9-]*\): [a-zA-Z0-9<>() ,_-]*,\?$/s|self[.]\1[.]clone[(][)]|self.\1.get()|\nt skip/
	p
	g
	s/ *\([a-zA-Z_0-9-]*\): [a-zA-Z0-9<>() ,_-]*,\?$/s|self[.]\1[(][)]|self.\1()|\nt skip/
	p
	g
	s/ *\([a-zA-Z_0-9-]*\): [a-zA-Z0-9<>() ,_-]*,\?$/s|self[.]\1|self.\1.get()|\nt skip/
	p
	g
	s/ *\([a-zA-Z_0-9-]*\): [a-zA-Z0-9<>() ,_-]*,\?$/s|self[.]unsafe_get[(][)][)][.]\1|self.unsafe_get()).\1.get()|\nt skip/
	p

	b skip
:layout_mode

	h

	s/ *\([a-zA-Z_0-9-]*\): .*/s|[*]\\([a-z]*\\)[.]\1[.]borrow_mut[(][)] = \\(.*\\)\\([,;]\\)|\\1.\1.set(\\2)\\3|\nt skip/
	p
	g
	s/ *\([a-zA-Z_0-9-]*\): .*/s|[*]\\([a-z]*\\)[.]\1[.]borrow_for_script_deallocation[(][)] = \\(.*\\)\\([,;]\\)|\\1.\1.set(\\2)\\3|\nt skip/
	p
	g
	s/ *\([a-zA-Z_0-9-]*\): .*/s|\\([a-z]*\\)[.]\1[.]borrow[(][)][.]clone[(][)]|\\1.\1.get()|\nt skip/
	p
	g
	s/ *\([a-zA-Z_0-9-]*\): .*/s|\\([a-z]*\\)[.]\1[.]borrow_for_layout[(][)]|\\1.\1.layout_get()|\nt skip/
	p
	g
	s/ *\([a-zA-Z_0-9-]*\): .*/s|\\([a-z]*\\)[.]\1[.]borrow[(][)]|\\1.\1.get()|\nt skip/
	p

:skip
	n

	s/^    \}$/:skip/
	t end
	b top
:end
	p
}
