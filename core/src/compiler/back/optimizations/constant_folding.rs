use crate::common::binary_op::BinaryOp;
use crate::common::expr::{BinaryExpr, Expr, UnaryExpr};
use crate::common::unary_op::UnaryOp;
use crate::common::value::Value;

use super::super::*;

pub fn constant_fold(rule: &mut Rule) {
  for lit in rule.body_literals_mut() {
    match lit {
      Literal::Assign(a) => match &a.right {
        AssignExpr::Binary(b) => match (&b.op1, &b.op2) {
          (Term::Constant(c1), Term::Constant(c2)) => {
            let expr = BinaryExpr {
              op: b.op.clone().into(),
              op1: Box::new(Expr::Constant(c1.clone())),
              op2: Box::new(Expr::Constant(c2.clone())),
            };
            let result = expr.eval(&().into()).as_value();
            *lit = Literal::Constraint(Constraint::Binary(BinaryConstraint {
              op: BinaryConstraintOp::Eq,
              op1: Term::Variable(a.left.clone()),
              op2: Term::Constant(result),
            }));
          }
          _ => {}
        },
        AssignExpr::Unary(u) => match &u.op1 {
          Term::Constant(c1) => {
            let expr = UnaryExpr {
              op: u.op.clone().into(),
              op1: Box::new(Expr::Constant(c1.clone())),
            };
            let result = expr.eval(&().into()).as_value();
            *lit = Literal::Constraint(Constraint::Binary(BinaryConstraint {
              op: BinaryConstraintOp::Eq,
              op1: Term::Variable(a.left.clone()),
              op2: Term::Constant(result),
            }));
          }
          _ => {}
        },
        AssignExpr::IfThenElse(i) => match &i.cond {
          Term::Constant(Value::Bool(true)) => {
            *lit = Literal::Constraint(Constraint::Binary(BinaryConstraint {
              op: BinaryConstraintOp::Eq,
              op1: Term::Variable(a.left.clone()),
              op2: i.then_br.clone(),
            }))
          }
          Term::Constant(Value::Bool(false)) => {
            *lit = Literal::Constraint(Constraint::Binary(BinaryConstraint {
              op: BinaryConstraintOp::Eq,
              op1: Term::Variable(a.left.clone()),
              op2: i.else_br.clone(),
            }))
          }
          _ => {}
        },
      },
      Literal::Constraint(c) => match c {
        Constraint::Binary(b) => match (&b.op, &b.op1, &b.op2) {
          (op, Term::Constant(c1), Term::Constant(c2)) => {
            let expr = BinaryExpr {
              op: BinaryOp::from(op),
              op1: Box::new(Expr::Constant(c1.clone())),
              op2: Box::new(Expr::Constant(c2.clone())),
            };
            let result = expr.eval(&().into()).as_bool();
            if result {
              *lit = Literal::True;
            } else {
              *lit = Literal::False;
            }
          }
          (BinaryConstraintOp::Neq, Term::Variable(v1), Term::Variable(v2)) if v1 == v2 => {
            *lit = Literal::False;
          }
          (BinaryConstraintOp::Eq, Term::Variable(v1), Term::Variable(v2)) if v1 == v2 => {
            *lit = Literal::True;
          }
          _ => {}
        },
        Constraint::Unary(u) => match &u.op1 {
          Term::Constant(c1) => {
            let expr = UnaryExpr {
              op: UnaryOp::from(&u.op),
              op1: Box::new(Expr::Constant(c1.clone())),
            };
            let result = expr.eval(&().into()).as_bool();
            if result {
              *lit = Literal::True;
            } else {
              *lit = Literal::False;
            }
          }
          _ => {}
        },
      },
      _ => {}
    }
  }
}
