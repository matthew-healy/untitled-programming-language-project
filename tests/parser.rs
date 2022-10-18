use rand::Rng;
use untitled_programming_language_project::{ast, parser};

#[test]
fn number_literal() {
    let mut rng = rand::thread_rng();

    let mut ns = [0; 128];
    rng.fill(&mut ns);

    let parser = parser::ExprParser::new();

    for n in ns {
        let input = format!("{}", n);
        let expr = parser.parse_successfully(input.as_str());
        assert_eq!(ast::Expr::Number(n), *expr)
    }
}

#[test]
fn binary_ops() {
    use ast::*;

    let parser = parser::ExprParser::new();

    for (name, input, expected) in [
        ("addition", "1 + 1", mk_op(1, Opcode::Add, 1)),
        ("subtraction", "99 - 4", mk_op(99, Opcode::Sub, 4)),
        ("multiplication", "-3 * -914", mk_op(-3, Opcode::Mul, -914)),
        ("division", "4444 / 1111", mk_op(4444, Opcode::Div, 1111)),
    ] {
        let actual = parser.parse_successfully(input);
        assert_eq!(expected, *actual, "{}", name)
    }
}

#[test]
fn operator_precedence() {
    use ast::{self, Opcode::*};

    let parser = parser::ExprParser::new();

    for (name, input, expected) in [
        ("+ then +", "1 + 2 + 3", mk_op(mk_op(1, Add, 2), Add, 3)),
        ("+ then -", "3 + 5 - 4", mk_op(mk_op(3, Add, 5), Sub, 4)),
        ("+ then *", "1 + 2 * 3", mk_op(1, Add, mk_op(2, Mul, 3))),
        ("+ then /", "2 + 2 / 2", mk_op(2, Add, mk_op(2, Div, 2))),
        ("- then +", "6 - 2 + 1", mk_op(mk_op(6, Sub, 2), Add, 1)),
        ("- then -", "9 - 6 - 8", mk_op(mk_op(9, Sub, 6), Sub, 8)),
        ("- then *", "3 - 2 * 1", mk_op(3, Sub, mk_op(2, Mul, 1))),
        ("- then /", "2 - 2 / 2", mk_op(2, Sub, mk_op(2, Div, 2))),
        ("* then +", "1 * 2 + 3", mk_op(mk_op(1, Mul, 2), Add, 3)),
        ("* then -", "3 * 5 - 4", mk_op(mk_op(3, Mul, 5), Sub, 4)),
        ("* then *", "1 * 2 * 3", mk_op(mk_op(1, Mul, 2), Mul, 3)),
        ("* then /", "2 * 2 / 2", mk_op(mk_op(2, Mul, 2), Div, 2)),
        ("/ then +", "6 / 2 + 1", mk_op(mk_op(6, Div, 2), Add, 1)),
        ("/ then -", "9 / 6 - 8", mk_op(mk_op(9, Div, 6), Sub, 8)),
        ("/ then *", "3 / 2 * 1", mk_op(mk_op(3, Div, 2), Mul, 1)),
        ("/ then /", "2 / 2 / 2", mk_op(mk_op(2, Div, 2), Div, 2)),
        (
            "* then + then *",
            "2 * 2 + 3 * 3",
            mk_op(mk_op(2, Mul, 2), Add, mk_op(3, Mul, 3)),
        ),
        (
            "+ then * then +",
            "2 + 2 * 3 + 3",
            mk_op(mk_op(2, Add, mk_op(2, Mul, 3)), Add, 3),
        ),
        (
            "/ then * then /",
            "2 / 4 * 1 / 3",
            mk_op(mk_op(mk_op(2, Div, 4), Mul, 1), Div, 3),
        ),
        (
            "parens can force precedence",
            "2 / 4 * (1 / 3)",
            mk_op(mk_op(2, Div, 4), Mul, mk_op(1, Div, 3)),
        ),
    ] {
        let expr = parser.parse_successfully(input);
        assert_eq!(expected, *expr, "{}", name)
    }
}

/// make a binary op from two expressions.
fn mk_op<L, R>(l: L, op: ast::Opcode, r: R) -> ast::Expr
where
    L: Into<ast::Expr>,
    R: Into<ast::Expr>,
{
    ast::Expr::Op(Box::new(l.into()), op, Box::new(r.into()))
}

/// wrap the lalrpop generated `ExprParser` so we don't need to
/// handle the `Err` case every time.
trait ParseSuccessfully {
    fn parse_successfully<'input>(&self, input: &'input str) -> Box<ast::Expr>;
}

impl ParseSuccessfully for parser::ExprParser {
    fn parse_successfully<'input>(&self, input: &'input str) -> Box<ast::Expr> {
        match self.parse(input) {
            Ok(expr) => expr,
            Err(e) => panic!("unexpected parse failure.\ninput: {}\nerror: {}", input, e),
        }
    }
}
