pub mod data;
pub mod error;

use crate::p1_syntax as syntax;
use crate::p2_items as items;
pub use {data::*, error::*};

pub fn check_bodies(items: items::Program) -> Result<Program> {
    let mut program = Program {
        items,
        func_bodies: Vec::new(),
    };

    for item in &program.items.funcs {
        if let (name, items::FuncItem::Local(func)) = item {
            let signature = FuncItemRef::new(name.clone(), &program.items.funcs)
                .expect("func should exist at this point.");
            let body = func
                .body
                .iter()
                .map(|expr| visit_expression(expr, &program.items))
                .collect::<Result<_>>()?;
            program.func_bodies.push(FuncBody { signature, body });
        }
    }

    Ok(program)
}

fn visit_expression(expr: &syntax::Expression, items: &items::Program) -> Result<Expression> {
    Ok(match expr {
        syntax::Expression::Integer(num) => Expression::Integer(*num),
        syntax::Expression::Call {
            leading,
            arguments,
            name,
        } => {
            let name = FuncItemRef::new(name.clone(), &items.funcs)
                .ok_or_else(|| BodyError::UnknownCall { name: name.clone() })?;

            let arguments = arguments
                .iter()
                .map(|expr| visit_expression(expr, items))
                .collect::<Result<_>>()?;

            let leading = if let Some(expr) = leading {
                Some(Box::new(visit_expression(expr, items)?))
            } else {
                None
            };

            Expression::Call {
                name,
                arguments,
                leading,
            }
        }
    })
}
