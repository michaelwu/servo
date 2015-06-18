/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use syntax::ext::base::{ExtCtxt, DummyResult, MacEager, MacResult};
use syntax::codemap::Span;
use syntax::ptr::P;
use syntax::ast;
use syntax::util::small_vector::SmallVector;

enum MagicFieldType {
    BaseField,
    ConstField,
    MutField,
    LayoutField,
}

fn get_field_info(field: &ast::StructField) -> (MagicFieldType, P<ast::Ty>) {
    let path = match field.node.ty.node {
        ast::TyPath(_, ref path) => path,
        _ => return (MagicFieldType::ConstField, field.node.ty.clone()),
    };

    if path.segments.len() != 1 {
        return (MagicFieldType::ConstField, field.node.ty.clone());
    }

    let segment = &path.segments[0];
    let inner_type = match segment.parameters {
        ast::AngleBracketedParameters(ref params) => {
            if params.lifetimes.len() > 0 || params.types.len() != 1 {
                return (MagicFieldType::ConstField, field.node.ty.clone());
            }
            params.types[0].clone()
        },
        _ => return (MagicFieldType::ConstField, field.node.ty.clone()),
    };

    let fieldtype = match &*segment.identifier.name.as_str() {
        "Base" => MagicFieldType::BaseField,
        "Mut" => MagicFieldType::MutField,
        "Layout" => MagicFieldType::LayoutField,
        _ => MagicFieldType::ConstField,
    };

    match fieldtype {
        MagicFieldType::ConstField => (MagicFieldType::ConstField, field.node.ty.clone()),
        _ => (fieldtype, inner_type),
    }
}

fn field_size_expr<'cx>(cx: &'cx mut ExtCtxt,
                        last_idx_type: P<ast::Ty>, last_inner_type: P<ast::Ty>)
                        -> P<ast::Expr> {
    quote_expr!(cx,
        <$last_idx_type as ::dom::bindings::magic::SlotIndex>::IDX +
        <$last_inner_type as ::dom::bindings::magic::MagicCastable>::SLOT_SIZE)
}

pub fn expand_magic_dom_struct<'cx>(cx: &'cx mut ExtCtxt,
                                    sp: Span, tts: &[ast::TokenTree])
                                    -> Box<MacResult + 'cx> {
    let mut items = vec!();
    let mut item = match cx.new_parser_from_tts(tts).parse_item() {
        Some(item) => (*item).clone(),
        None => return DummyResult::expr(sp),
    };
    let mut slot_count = None;
    let mut traces = vec!();
    let mut size_updates = vec!();
    let mut finalizers = vec!();
    let mut js_accessors = vec!();
    let name: ast::Ident = item.ident;

    item.attrs.push(quote_attr!(cx, #[privatize]));

    item.node = if let ast::ItemStruct(ref def, ref gen) = item.node {
        let mut new_fields = vec!();
        // Put every type in a MagicField
        let mut last_idx_type = None;
        let mut last_inner_type = quote_ty!(cx, FakeType);
        let mut last_field_type = None;
        let mut need_finalize_expr = quote_expr!(cx, false);
        let mut heap_type_expr = quote_expr!(cx, false);
        for field in &def.fields {
            let (field_type, inner_type) = get_field_info(field);
            let mut field = field.clone();
            let field_name = match field.node.kind {
                ast::NamedField(ident, _) => (&*ident.name.as_str()).to_owned(),
                ast::UnnamedField(_) => {
                    cx.span_err(field.span,
                                "Unexpected unnamed field in struct");
                    "_unnamed_".to_owned()
                }
            };
            let idx_type_name =
                format!("_{}_{}", item.ident.name.as_str(), field_name);
            let idx_type_id = cx.ident_of(&idx_type_name);
            let idx_type = quote_ty!(cx, $idx_type_id);
            let field_ty = match field_type {
                MagicFieldType::BaseField => inner_type.clone(),
                MagicFieldType::ConstField => quote_ty!(cx, ::dom::bindings::magic::ConstMagicField<$inner_type, $idx_type>),
                MagicFieldType::MutField => quote_ty!(cx, ::dom::bindings::magic::MutMagicField<$inner_type, $idx_type>),
                MagicFieldType::LayoutField => quote_ty!(cx, ::dom::bindings::magic::LayoutMagicField<$inner_type, $idx_type>),
            };
            field.node.ty = field_ty.clone();
            new_fields.push(field);

            match field_type {
                MagicFieldType::BaseField => {
                    last_field_type = Some(field_type);
                    last_inner_type = inner_type;
                    continue;
                },
                _ => (),
            }

            items.push(quote_item!(cx, #[allow(non_camel_case_types)] struct $idx_type_id;).unwrap());
            if last_idx_type.is_none() {
                let expr = match last_field_type {
                    Some(MagicFieldType::BaseField) =>
                        quote_expr!(cx, <$last_inner_type as ::dom::bindings::magic::SlotCount>::SLOT_COUNT),
                    _ => quote_expr!(cx, 0),
                };
                items.push(quote_item!(cx,
                    impl ::dom::bindings::magic::SlotIndex for $idx_type_id {
                        const IDX: u8 = $expr;
                    }).unwrap());
            } else {
                let expr = field_size_expr(cx, last_idx_type.clone().unwrap(), last_inner_type.clone());
                items.push(quote_item!(cx,
                    impl ::dom::bindings::magic::SlotIndex for $idx_type {
                        const IDX: u8 = $expr;
                    }).unwrap());
            }

            if let MagicFieldType::BaseField = field_type {
                let field_id = cx.ident_of(&field_name);
                traces.push(quote_expr!(cx, ::dom::bindings::trace::JSTraceable::trace(&self.$field_id, trc)));
                size_updates.push(quote_expr!(cx, size += ::util::mem::HeapSizeOf::heap_size_of_children(&self.$field_id)));
                finalizers.push(quote_expr!(cx,
                    if <$inner_type as ::dom::bindings::magic::SlotCount>::NEED_FINALIZE {
                        ::dom::bindings::magic::SlotCount::finalize(&self.$field_id)
                    }));
            } else {
                traces.push(quote_expr!(cx,
                    if <$inner_type as ::dom::bindings::magic::MagicCastable>::HEAP_TYPE {
                        <$inner_type as ::dom::bindings::magic::MagicCastable>::trace(real, <$idx_type_id as ::dom::bindings::magic::SlotIndex>::IDX, trc);
                    }));
                size_updates.push(quote_expr!(cx,
                    if <$inner_type as ::dom::bindings::magic::MagicCastable>::HEAP_TYPE {
                        size += <$inner_type as ::dom::bindings::magic::MagicCastable>::heap_size_of(real, <$idx_type_id as ::dom::bindings::magic::SlotIndex>::IDX);
                    }));
                finalizers.push(quote_expr!(cx,
                    if <$inner_type as ::dom::bindings::magic::MagicCastable>::NEED_FINALIZE {
                        <$inner_type as ::dom::bindings::magic::MagicCastable>::finalize_slots(real, <$idx_type_id as ::dom::bindings::magic::SlotIndex>::IDX);
                    }));
                js_accessors.push(quote_expr!(cx,
                    if true {
                        buf.push_str(&(<$field_ty>::slot_access_code($field_name)));
                    }
                ));
            }

            need_finalize_expr = if let Some(MagicFieldType::BaseField) = last_field_type {
                quote_expr!(cx, <$last_inner_type as ::dom::bindings::magic::SlotCount>::NEED_FINALIZE)
            } else if let Some(_) = last_idx_type {
                quote_expr!(cx, $need_finalize_expr || <$inner_type as ::dom::bindings::magic::MagicCastable>::NEED_FINALIZE)
            } else {
                quote_expr!(cx, <$inner_type as ::dom::bindings::magic::MagicCastable>::NEED_FINALIZE)
            };

            heap_type_expr = if let Some(MagicFieldType::BaseField) = last_field_type {
                quote_expr!(cx, <$last_inner_type as ::dom::bindings::magic::SlotCount>::HEAP_TYPE)
            } else if let Some(_) = last_idx_type {
                quote_expr!(cx, $heap_type_expr || <$inner_type as ::dom::bindings::magic::MagicCastable>::HEAP_TYPE)
            } else {
                quote_expr!(cx, <$inner_type as ::dom::bindings::magic::MagicCastable>::HEAP_TYPE)
            };

            last_idx_type = Some(idx_type);
            last_inner_type = inner_type;
            last_field_type = Some(field_type);
        }
        let expr = match last_field_type {
            Some(MagicFieldType::BaseField) =>
                quote_expr!(cx, <$last_inner_type as ::dom::bindings::magic::SlotCount>::SLOT_COUNT),
            None => quote_expr!(cx, 0),
            _ => field_size_expr(cx, last_idx_type.unwrap(), last_inner_type.clone()),
        };
        slot_count = quote_item!(cx,
            impl ::dom::bindings::magic::SlotCount for $name {
                const SLOT_COUNT: u8 = $expr;
                const NEED_FINALIZE: bool = $need_finalize_expr;
                const HEAP_TYPE: bool = $heap_type_expr;
                #[allow(unused_variables)]
                #[allow(unsafe_code)]
                unsafe fn finalize(&self) {
                    debug_assert!(<$name as ::dom::bindings::magic::SlotCount>::NEED_FINALIZE);
                    let real = self as *const _ as *const ::dom::bindings::magic::RealFields;
                    let real = &*real;
                    { $finalizers }
                }
                fn get_access_code() -> String {
                    let mut buf = format!("class {} {{\n", stringify!($name));
                    buf.push_str("    obj: Object;\n");
                    buf.push_str("    constructor(obj: Object) { this.obj = obj; }\n");
                    { $js_accessors };
                    buf.push_str("}\n");
                    buf
                }
            });
        ast::ItemStruct(P(ast::StructDef {fields: new_fields, ctor_id: None}), gen.clone())
    } else {
        cx.span_err(sp, "#[dom_struct] applied to something other than a struct");
        item.node
    };
    items.push(P(item));

    items.push(quote_item!(cx,
        impl ::dom::bindings::trace::JSTraceable for $name {
            #[allow(unused_variables)]
            #[allow(unsafe_code)]
            fn trace(&self, trc: *mut ::js::jsapi::JSTracer) {
                let real = self as *const _ as *const ::dom::bindings::magic::RealFields;
                let real = unsafe { &*real };
                { $traces }
            }
        }).unwrap());

    items.push(quote_item!(cx,
        impl ::util::mem::HeapSizeOf for $name {
            #[allow(unused_variables)]
            #[allow(unused_mut)]
            #[allow(unsafe_code)]
            fn heap_size_of_children(&self) -> usize {
                let real = self as *const _ as *const ::dom::bindings::magic::RealFields;
                let real = unsafe { &*real };
                let mut size = 0;
                { $size_updates }
                size
            }
        }).unwrap());

    match slot_count {
        Some(slot_count) => items.push(slot_count),
        None => (),
    }
    MacEager::items(SmallVector::many(items))
}
