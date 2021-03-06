use crate::error::BareError;
use crate::expr::pattern::pattern;
use crate::keyword::Keyword;
use crate::space::ws;
use chumsky::primitive::just;
use chumsky::recursive::Recursive;
use chumsky::Parser;
use sappho_ast::{Expr, LetClause, LetExpr, ListExpr, MatchClause, MatchExpr};

pub(crate) fn recursive_expr<'a, FX: 'a>(
    expr: Recursive<'a, char, Expr<FX>, BareError>,
) -> impl Parser<char, Expr<FX>, Error = BareError> + 'a {
    use Expr::List;

    list_expr(expr.clone())
        .map(List)
        .or(let_expr(expr.clone()).map(Expr::from))
        .or(match_expr(expr).map(Expr::from))
}

fn list_expr<'a, FX: 'a>(
    expr: Recursive<'a, char, Expr<FX>, BareError>,
) -> impl Parser<char, ListExpr<FX>, Error = BareError> + 'a {
    use crate::listform::list_form;

    list_form(expr.clone(), expr.map(Box::new)).labelled("list-expression")
}

fn let_expr<'a, FX: 'a>(
    expr: Recursive<'a, char, Expr<FX>, BareError>,
) -> impl Parser<char, LetExpr<FX>, Error = BareError> + 'a {
    let_clause(expr.clone())
        .then_ignore(ws())
        .repeated()
        .at_least(1)
        .then(expr)
        .map(|(clauses, tail)| LetExpr {
            clauses,
            tail: Box::new(tail),
        })
        .labelled("let-expression")
}

fn let_clause<'a, FX: 'a>(
    expr: Recursive<'a, char, Expr<FX>, BareError>,
) -> impl Parser<char, LetClause<FX>, Error = BareError> + 'a {
    Keyword::Let
        .parser()
        .ignore_then(pattern())
        .then_ignore(just('=').delimited_by(ws(), ws()))
        .then(expr.clone())
        .then_ignore(just(';'))
        .map(|(binding, bindexpr)| LetClause {
            binding,
            bindexpr: Box::new(bindexpr),
        })
}

fn match_expr<'a, FX: 'a>(
    expr: Recursive<'a, char, Expr<FX>, BareError>,
) -> impl Parser<char, MatchExpr<FX>, Error = BareError> + 'a {
    use crate::delimited::delimited;

    Keyword::Match
        .parser()
        .ignore_then(expr.clone())
        .then_ignore(ws())
        .then(delimited(
            '{',
            match_clause(expr)
                .separated_by(just(',').then(ws()))
                .allow_trailing(),
            '}',
        ))
        .map(|(target, clauses)| MatchExpr {
            target: Box::new(target),
            clauses,
        })
        .labelled("match-expression")
}

fn match_clause<'a, FX: 'a>(
    expr: Recursive<'a, char, Expr<FX>, BareError>,
) -> impl Parser<char, MatchClause<FX>, Error = BareError> + 'a {
    pattern()
        .then_ignore(just("->").delimited_by(ws(), ws()))
        .then(expr)
        .map(|(pattern, body)| MatchClause {
            pattern,
            body: Box::new(body),
        })
        .labelled("match-clause")
}
