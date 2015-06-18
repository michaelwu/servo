/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use syntax::ast;
use syntax::ast::{MetaItem, Expr};
use syntax::codemap::Span;
use syntax::ext::base::{Annotatable, ExtCtxt};
use syntax::ext::build::AstBuilder;
use syntax::ext::deriving::generic::{Struct, Substructure, TraitDef, ty};
use syntax::ext::deriving::generic::{combine_substructure, EnumMatching, FieldInfo, MethodDef};
use syntax::ptr::P;

/// Provides the hook to expand `#[derive(JSTraceable)]` into an implementation of `JSTraceable`
///
/// The expansion basically calls `trace()` on all of the fields of the struct/enum, erroring if they do not
/// implement the method.
pub fn expand_jstraceable(cx: &mut ExtCtxt, span: Span, mitem: &MetaItem, item: &Annotatable,
                          push: &mut FnMut(Annotatable)) {
    let trait_def = TraitDef {
        is_unsafe: false,
        span: span,
        attributes: Vec::new(),
        path: ty::Path::new(vec!("dom", "bindings", "trace", "JSTraceable")),
        additional_bounds: Vec::new(),
        generics: ty::LifetimeBounds::empty(),
        methods: vec![
            MethodDef {
                name: "trace",
                generics: ty::LifetimeBounds::empty(),
                explicit_self: ty::borrowed_explicit_self(),
                args: vec!(ty::Ptr(box ty::Literal(ty::Path::new(vec!("js", "jsapi", "JSTracer"))),
                                   ty::Raw(ast::MutMutable))),
                ret_ty: ty::nil_ty(),
                attributes: vec![quote_attr!(cx, #[inline])],
                is_unsafe: false,
                combine_substructure: combine_substructure(box jstraceable_substructure)
            }
        ],
        associated_types: vec![],
    };
    trait_def.expand(cx, mitem, item, push)
}

// Mostly copied from syntax::ext::deriving::hash
/// Defines how the implementation for `trace()` is to be generated
fn jstraceable_substructure(cx: &mut ExtCtxt, trait_span: Span, substr: &Substructure) -> P<Expr> {
    let state_expr = if substr.nonself_args.len() == 1 {
        &substr.nonself_args[0]
    } else {
        cx.span_bug(trait_span, "incorrect number of arguments in `jstraceable`")
    };
    let trace_ident = substr.method_ident;
    let call_trace = |span, thing_expr| {
        let expr = cx.expr_method_call(span, thing_expr, trace_ident, vec!(state_expr.clone()));
        cx.stmt_expr(expr)
    };
    let mut stmts = Vec::new();

    let fields = match *substr.fields {
        Struct(ref fs) | EnumMatching(_, _, ref fs) => fs,
        _ => cx.span_bug(trait_span, "impossible substructure in `jstraceable`")
    };

    for &FieldInfo { ref self_, span, .. } in fields {
        stmts.push(call_trace(span, self_.clone()));
    }

    cx.expr_block(cx.block(trait_span, stmts, None))
}
