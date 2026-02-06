use promql_parser::label::MatchOp;
use promql_parser::parser;
use promql_parser::parser::Expr;

pub fn parse_promql(query: &str) -> Vec<String> {
    match parser::parse(query) {
        Ok(mut expr) => extract_labels(&mut expr),
        Err(info) => {
            tracing::warn!("PromQL parse error: {info:?}");
            vec![]
        }
    }

    // vec!["dev".to_string(), "production".to_string()]
}

fn extract_labels(expr: &mut Expr) -> Vec<String> {
    tracing::info!("Prettify: {}", expr.prettify());
    let mut result = Vec::new();
    walk_expr(expr, &mut result);
    tracing::info!("AST: {expr:?}");
    tracing::info!("result: {result:?}");
    result
}

// TODO right now supported only vector expressions
// and retaining only one label
fn walk_expr(expr: &mut Expr, out: &mut Vec<String>) {
    match expr {
        Expr::VectorSelector(vs) => {
            vs.matchers
                .matchers
                .retain(|m| !(m.name == "env" && m.op == MatchOp::Equal));
            // TODO add MatchOp::Re
            // currently we can send the data to the default receiver if Regex

            for m in &vs.matchers.matchers {
                out.push(m.value.clone());
            }
        }

        // Expr::Aggregate(a) => walk_expr(&a.expr, out),
        //
        // Expr::Binary(b) => {
        //     walk_expr(&b.lhs, out);
        //     walk_expr(&b.rhs, out);
        // }
        //
        // Expr::Unary(u) => walk_expr(&u.expr, out),
        //
        // Expr::Call(c) => {
        //     for arg in &c.args.args {
        //         walk_expr(arg, out);
        //     }
        // }
        //
        // Expr::Subquery(s) => walk_expr(&s.expr, out),
        //
        // Expr::MatrixSelector(ms) => walk_expr(&Expr::VectorSelector(ms.vs.clone()), out),
        _ => {}
    }
}
