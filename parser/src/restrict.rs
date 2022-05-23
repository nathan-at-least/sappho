use crate::Error;
use saplang_ast::{
    Application, GenExpr, LetExpr, ProcEffects, PureEffects, QueryEffects, QueryExpr, RecursiveExpr,
};

pub(crate) type Span = <Error as chumsky::error::Error<char>>::Span;

pub(crate) trait Restrict<S>: Sized {
    fn restrict(src: S, span: Span) -> Result<Self, Error>;
}

impl Restrict<ProcEffects> for PureEffects {
    fn restrict(src: ProcEffects, span: Span) -> Result<Self, Error> {
        Err(Error::custom(
            span,
            format!(
                "Pure expressions cannot contain inquire/evoke effects: {:?}",
                src
            ),
        ))
    }
}

impl Restrict<ProcEffects> for QueryEffects {
    fn restrict(src: ProcEffects, span: Span) -> Result<Self, Error> {
        match src {
            ProcEffects::Inquire(x) => {
                Box::<QueryExpr>::restrict(x, span).map(QueryEffects::Inquire)
            }
            ProcEffects::Evoke(_) => Err(Error::custom(
                span,
                format!("Query expressions cannot contain evoke effects: {:?}", src),
            )),
        }
    }
}

impl<FXS, FXD> Restrict<GenExpr<FXS>> for GenExpr<FXD>
where
    FXD: Restrict<FXS>,
{
    fn restrict(src: GenExpr<FXS>, span: Span) -> Result<Self, Error> {
        use GenExpr::*;

        match src {
            Universal(x) => Ok(Universal(x)),
            Common(x) => Ok(Common(x)),
            Recursive(x) => RecursiveExpr::restrict(x, span).map(Recursive),
            Effect(x) => FXD::restrict(x, span).map(Effect),
        }
    }
}

impl<FXS, FXD> Restrict<RecursiveExpr<FXS>> for RecursiveExpr<FXD>
where
    FXD: Restrict<FXS>,
{
    fn restrict(src: RecursiveExpr<FXS>, span: Span) -> Result<Self, Error> {
        use RecursiveExpr::*;

        match src {
            List(x) => {
                let mut v = vec![];
                for subx in x.into_iter() {
                    let suby = GenExpr::<FXD>::restrict(subx, span.clone())?;
                    v.push(suby);
                }
                Ok(List(v))
            }
            Let(x) => LetExpr::restrict(x, span).map(Let),
            Apply(x) => Application::restrict(x, span).map(Apply),
        }
    }
}

impl<FXS, FXD> Restrict<LetExpr<FXS>> for LetExpr<FXD>
where
    FXD: Restrict<FXS>,
{
    fn restrict(src: LetExpr<FXS>, span: Span) -> Result<Self, Error> {
        Ok(LetExpr {
            binding: src.binding,
            bindexpr: Box::new(GenExpr::<FXD>::restrict(*src.bindexpr, span.clone())?),
            tail: Box::new(GenExpr::<FXD>::restrict(*src.tail, span)?),
        })
    }
}

impl<FXS, FXD> Restrict<Application<FXS>> for Application<FXD>
where
    FXD: Restrict<FXS>,
{
    fn restrict(src: Application<FXS>, span: Span) -> Result<Self, Error> {
        Ok(Application {
            target: Box::new(GenExpr::<FXD>::restrict(*src.target, span.clone())?),
            argument: Box::new(GenExpr::<FXD>::restrict(*src.argument, span)?),
        })
    }
}

impl<S, D> Restrict<Box<S>> for Box<D>
where
    D: Restrict<S>,
{
    fn restrict(src: Box<S>, span: Span) -> Result<Self, Error> {
        let d = D::restrict(*src, span)?;
        Ok(Box::new(d))
    }
}