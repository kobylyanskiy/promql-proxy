use promql_parser::label::MatchOp;
use promql_parser::label::Matcher;
use promql_parser::parser;
use promql_parser::parser::Expr;

pub fn parse_promql(label_name: String, query: &str) -> (String, String) {
    match parser::parse(query) {
        Ok(mut expr) => {
            let env = walk_expr(&label_name, &mut expr);
            tracing::debug!("MODIFIED AST: {expr:?}");
            if !env.is_empty() {
                tracing::debug!("Found environment: {}", env);
                (env, expr.prettify())
            } else {
                tracing::debug!("No environment specified, using default receiver.");
                (env, query.to_string())
            }
        }
        Err(info) => {
            tracing::warn!("PromQL parse error: {info:?}");
            (String::new(), query.to_string())
        }
    }
}

fn walk_expr(label_name: &String, expr: &mut Expr) -> String {
    match expr {
        Expr::VectorSelector(vs) => {
            extract_label_from_matchers(label_name, &mut vs.matchers.matchers)
        }

        Expr::MatrixSelector(ms) => {
            extract_label_from_matchers(label_name, &mut ms.vs.matchers.matchers)
        }

        Expr::Aggregate(a) => walk_expr(label_name, &mut a.expr),
        Expr::Unary(u) => walk_expr(label_name, &mut u.expr),
        Expr::Subquery(s) => walk_expr(label_name, &mut s.expr),

        Expr::Binary(b) => {
            let left = walk_expr(label_name, &mut b.lhs);
            let right = walk_expr(label_name, &mut b.rhs);

            if !left.is_empty() && left == right {
                left
            } else {
                String::new()
            }
        }

        // TODO check one more time
        Expr::Call(c) => {
            let mut found = String::new();
            for arg in &mut c.args.args {
                let res = walk_expr(label_name, arg);
                if !res.is_empty() {
                    if !found.is_empty() && found != res {
                        // Ой! В одном вызове разные окружения
                        return String::new();
                    }
                    found = res;
                }
            }
            found
        }
        _ => String::new(),
    }
}

fn extract_label_from_matchers(label_name: &String, matchers: &mut Vec<Matcher>) -> String {
    let mut label_value = String::new();

    // retain() walks the vector and keeps elements only if the closure returns true
    matchers.retain(|m| {
        if m.name == *label_name && m.op == MatchOp::Equal {
            // 1. Capture the value
            label_value = m.value.clone();
            // 2. Return false to remove this matcher from the vector
            false
        } else {
            // 3. Return true to keep everything else
            true
        }
    });

    label_value
}
