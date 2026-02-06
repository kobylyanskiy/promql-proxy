use promql_parser::label::MatchOp;
use promql_parser::label::Matcher;
use promql_parser::parser;
use promql_parser::parser::Expr;

fn is_not_env_label(m: &Matcher, env_out: &mut String) -> bool {
    // TODO use target_label from config
    if m.name == "env" && m.op == MatchOp::Equal {
        *env_out = m.value.clone();
        false
    } else {
        true
    }
}

pub fn parse_promql(query: &str) -> (String, String) {
    match parser::parse(query) {
        Ok(mut expr) => {
            let env = extract_env(&mut expr);
            (env, expr.prettify())
        }
        Err(info) => {
            tracing::warn!("PromQL parse error: {info:?}");
            (String::new(), query.to_string())
        }
    }
}

fn extract_env(expr: &mut Expr) -> String {
    tracing::info!("Prettify: {}", expr.prettify());
    let env = walk_expr(expr);
    if !env.is_empty() {
        tracing::info!("Found environment: {}", env);
    } else {
        tracing::info!("No environment specified, using default receiver.");
    }
    tracing::info!("MODIFIED AST: {expr:?}");
    env
}

fn walk_expr(expr: &mut Expr) -> String {
    let mut env_value = String::new();
    match expr {
        Expr::VectorSelector(vs) => {
            vs.matchers
                .matchers
                .retain(|m| is_not_env_label(m, &mut env_value));
            env_value
        }

        Expr::MatrixSelector(ms) => {
            ms.vs
                .matchers
                .matchers
                .retain(|m| is_not_env_label(m, &mut env_value));
            env_value
        }
        Expr::Aggregate(a) => walk_expr(&mut a.expr),
        //
        // TODO a bit complicated logic to merge both sides of query
        // correctly
        //
        // (env=1) / (env=1) => 1
        // (env=1) / (env=2) => ""
        // (env="") / (env=1) => ""
        // (env=1) / (env="") => ""
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
        _ => env_value,
    }
}
