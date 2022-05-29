use crate::delimited::delimited;
use crate::error::BareError;
use crate::error::Span;
use crate::expr::pattern::pattern;
use crate::expr::{pure_expr, query_expr};
use crate::keyword::Keyword;
use crate::space::ws;
use chumsky::primitive::just;
use chumsky::recursive::Recursive;
use chumsky::Parser;
use sappho_ast::{CommonExpr, FuncDef, ObjectDef, ProcExpr, QueryDef};

pub(crate) fn common_expr(
    expr: Recursive<'_, char, ProcExpr, BareError>,
) -> impl Parser<char, CommonExpr, Error = BareError> + '_ {
    use CommonExpr::*;

    object_def(expr.clone())
        .map(Object)
        .or(func_def(expr.clone()).map(Func))
        .or(query_def(expr).map(Query))
}

fn func_def(
    expr: Recursive<'_, char, ProcExpr, BareError>,
) -> impl Parser<char, FuncDef, Error = BareError> + '_ {
    Keyword::Fn
        .parser()
        .ignore_then(pattern())
        .then_ignore(just("->").delimited_by(ws(), ws()))
        .then(pure_expr(expr))
        .map(|(binding, body)| FuncDef {
            binding,
            body: Box::new(body),
        })
        .labelled("fn definition")
}

fn query_def(
    expr: Recursive<'_, char, ProcExpr, BareError>,
) -> impl Parser<char, QueryDef, Error = BareError> + '_ {
    Keyword::Query
        .parser()
        .ignore_then(query_expr(expr))
        .map(|body| QueryDef {
            body: Box::new(body),
        })
        .labelled("query definition")
}

fn object_def(
    expr: Recursive<'_, char, ProcExpr, BareError>,
) -> impl Parser<char, ObjectDef, Error = BareError> + '_ {
    let innards = object_clause(expr)
        .separated_by(just(';').then(ws().or_not()))
        .try_map(construct_object);

    delimited('{', innards, '}').labelled("object definition")
}

enum ObjectClause {
    Func(FuncDef),
    Query(QueryDef),
}

fn object_clause(
    expr: Recursive<'_, char, ProcExpr, BareError>,
) -> impl Parser<char, ObjectClause, Error = BareError> + '_ {
    use ObjectClause::*;

    func_def(expr.clone())
        .map(Func)
        .or(query_def(expr).map(Query))
}

fn construct_object(clauses: Vec<ObjectClause>, span: Span) -> Result<ObjectDef, BareError> {
    let mut query = None;
    let mut func = None;

    for clause in clauses.into_iter() {
        use ObjectClause::*;

        let clspan = span.clone();
        match clause {
            Query(x) => set_clause(&mut query, x, "query", clspan)?,
            Func(x) => set_clause(&mut func, x, "fn", clspan)?,
        }
    }

    Ok(ObjectDef { query, func })
}

fn set_clause<T>(
    slot: &mut Option<T>,
    clause: T,
    label: &str,
    span: Span,
) -> Result<(), BareError> {
    if slot.replace(clause).is_none() {
        Ok(())
    } else {
        Err(BareError::custom(
            span,
            format!("Object may not contain multiple {} clauses", label),
        ))
    }
}