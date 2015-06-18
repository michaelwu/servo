/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use syntax::ext::base::{ExtCtxt, DummyResult, MacEager, MacResult};
use syntax::codemap::Span;
use syntax::ptr::P;
use syntax::ast;
use syntax::util::small_vector::SmallVector;

fn simple_segment(cx: &mut ExtCtxt, name: &str) -> ast::PathSegment {
    ast::PathSegment {
        identifier: cx.ident_of(name),
        parameters: ast::PathParameters::none(),
    }
}

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

fn slot_info_expr<'cx>(cx: &'cx mut ExtCtxt, sp: Span,
                       last_inner_type: P<ast::Ty>,
                       trait_name: &[(&'static str, Option<ast::PathParameters>)],
                       field_name: &str)
                       -> P<ast::Expr> {
    let mut segments: Vec<_> =
        trait_name.iter().map(|&(name, ref params)| {
            match *params {
                None => simple_segment(cx, name),
                Some(ref params) => ast::PathSegment {
                    identifier: cx.ident_of(name),
                    parameters: params.clone(),
                },
            }
        }).collect();
    segments.push(simple_segment(cx, field_name));
    P(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        node: ast::ExprPath(
            Some(ast::QSelf {
                ty: last_inner_type,
                position: trait_name.len(),
            }),
            ast::Path {
                span: sp,
                global: true,
                segments: segments,
            }),
        span: sp,
    })
}

fn field_size_expr<'cx>(cx: &'cx mut ExtCtxt, sp: Span,
                        last_idx_type: P<ast::Ty>, last_inner_type: P<ast::Ty>)
                        -> P<ast::Expr> {
    let idx_expr =
        slot_info_expr(cx, sp, last_idx_type.clone(),
                       &[("dom", None),
                         ("bindings", None),
                         ("magic", None),
                         ("SlotIndex", None)],
                       "IDX");

    let slotsize_expr =
        slot_info_expr(cx, sp, last_inner_type.clone(),
                       &[("dom", None),
                         ("bindings", None),
                         ("magic", None),
                         ("MagicCastable", None)],
                       "SLOT_SIZE");

    quote_expr!(cx, $idx_expr + $slotsize_expr)
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

    item.attrs.push(quote_attr!(cx, #[privatize]));

    item.node = if let ast::ItemStruct(ref def, ref gen) = item.node {
        let mut new_fields = vec!();
        // Put every type in a MagicField
        let mut last_idx_type = None;
        let mut last_inner_type = quote_ty!(cx, FakeType);
        let mut last_field_type = None;
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
            let idx_type = P(ast::Ty {
                id: ast::DUMMY_NODE_ID,
                node: ast::TyPath(None,
                    ast::Path {
                        span: inner_type.span,
                        global: false,
                        segments: vec!(simple_segment(cx, &idx_type_name)),
                    }),
                span: inner_type.span,
            });
            field.node.ty = match field_type {
                MagicFieldType::BaseField => inner_type.clone(),
                MagicFieldType::ConstField => quote_ty!(cx, ::dom::bindings::magic::ConstMagicField<$inner_type, $idx_type>),
                MagicFieldType::MutField => quote_ty!(cx, ::dom::bindings::magic::MutMagicField<$inner_type, $idx_type>),
                MagicFieldType::LayoutField => quote_ty!(cx, ::dom::bindings::magic::LayoutMagicField<$inner_type, $idx_type>),
            };
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
                        slot_info_expr(cx, inner_type.span, last_inner_type.clone(),
                                       &[("dom", None),
                                         ("bindings", None),
                                         ("magic", None),
                                         ("SlotCount", None)],
                                       "SLOT_COUNT"),
                    _ => quote_expr!(cx, 0),
                };
                items.push(quote_item!(cx, impl ::dom::bindings::magic::SlotIndex for $idx_type_id { const IDX: u8 = $expr; }).unwrap());
            } else {
                let expr = field_size_expr(cx, inner_type.span, last_idx_type.unwrap(), last_inner_type.clone());
                items.push(quote_item!(cx, impl ::dom::bindings::magic::SlotIndex for $idx_type { const IDX: u8 = $expr; }).unwrap());
            }

            last_idx_type = Some(idx_type);
            last_inner_type = inner_type;
            last_field_type = Some(field_type);
        }
        let name: ast::Ident = item.ident;
        let expr = match last_field_type {
            Some(MagicFieldType::BaseField) =>
                slot_info_expr(cx, item.span, last_inner_type.clone(),
                               &[("dom", None),
                                 ("bindings", None),
                                 ("magic", None),
                                 ("SlotCount", None)],
                               "SLOT_COUNT"),
            None => quote_expr!(cx, 0),
            _ => field_size_expr(cx, item.span, last_idx_type.unwrap(), last_inner_type.clone()),
        };
        slot_count = quote_item!(cx, impl ::dom::bindings::magic::SlotCount for $name { const SLOT_COUNT: u8 = $expr; });
        ast::ItemStruct(P(ast::StructDef {fields: new_fields, ctor_id: None}), gen.clone())
    } else {
        cx.span_err(sp, "#[dom_struct] applied to something other than a struct");
        item.node
    };
    items.push(P(item));

    match slot_count {
        Some(slot_count) => items.push(slot_count),
        None => (),
    }
    MacEager::items(SmallVector::many(items))
}
