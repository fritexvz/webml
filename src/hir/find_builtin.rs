use crate::config::Config;
use crate::hir::util::Transform;
use crate::hir::*;
use crate::pass::Pass;

pub struct FindBuiltin;

impl Transform for FindBuiltin {
    fn transform_app(&mut self, ty: HTy, fun: Box<Expr>, arg: Box<Expr>) -> Expr {
        use crate::hir::Expr::*;
        match *fun {
            Expr::Sym { ref name, .. } if name.1 == 0 => {
                assert!(crate::BUILTIN_FUNCTIONS.contains(&name.0.as_str()));
                let fun = match name.0.as_str() {
                    "print" => BIF::Print,
                    name => unreachable!("unknown builtin function found: {}", name),
                };
                let arg = self.transform_expr(*arg);
                BuiltinCall {
                    ty,
                    fun,
                    args: vec![arg],
                }
            }
            _ => {
                let arg = self.transform_expr(*arg);
                let fun = self.transform_expr(*fun);

                App {
                    fun: Box::new(fun),
                    arg: Box::new(arg),
                    ty,
                }
            }
        }
    }
}

impl FindBuiltin {
    pub fn new() -> Self {
        FindBuiltin
    }
}

impl<E> Pass<HIR, E> for FindBuiltin {
    type Target = HIR;

    fn trans(&mut self, hir: HIR, _: &Config) -> ::std::result::Result<Self::Target, E> {
        Ok(self.transform_hir(hir))
    }
}
