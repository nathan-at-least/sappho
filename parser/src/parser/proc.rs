use crate::delimited::delimited;
use crate::parser::common::common_expr;
use crate::parser::procfx::proc_effect;
use crate::parser::recursive::recursive_expr;
use crate::parser::universal::universal_expr;
use crate::space::ws;
use crate::Error;
use chumsky::recursive::Recursive;
use chumsky::Parser;
use saplang_ast::ProcExpr;

pub(super) fn proc_expr_def(
    pexpr: Recursive<'_, char, ProcExpr, Error>,
) -> impl Parser<char, ProcExpr, Error = Error> + '_ {
    non_application(pexpr)
        .then_ignore(ws().or_not())
        .repeated()
        .at_least(1)
        .map(|exprs| {
            exprs
                .into_iter()
                .reduce(ProcExpr::application)
                .expect(".at_least(1) postcondition failed.")
        })
}

fn non_application(
    pexpr: Recursive<'_, char, ProcExpr, Error>,
) -> impl Parser<char, ProcExpr, Error = Error> + '_ {
    parens_expr(pexpr.clone())
        .or(proc_effect(pexpr.clone()).map(ProcExpr::Effect))
        .or(universal_expr().map(ProcExpr::Universal))
        .or(common_expr(pexpr.clone()).map(ProcExpr::Common))
        .or(recursive_expr(pexpr).map(ProcExpr::Recursive))
}

fn parens_expr(
    pexpr: Recursive<'_, char, ProcExpr, Error>,
) -> impl Parser<char, ProcExpr, Error = Error> + '_ {
    delimited('(', pexpr, ')')
}