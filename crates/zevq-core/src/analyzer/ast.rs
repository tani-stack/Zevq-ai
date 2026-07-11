use anyhow::Result;
use syn::{visit::Visit, Expr, ExprMethodCall};

#[derive(Debug, Default, Clone)]
pub struct AnalysisReport {
    pub has_unsafe_unwrap: bool,
    pub has_unchecked_index: bool,
    pub has_expect: bool,
    pub cyclomatic_complexity: u32,
    pub null_checks: u32,
    pub safe_guards: u32,
}

struct Visitor<'a> {
    report: &'a mut AnalysisReport,
}

impl<'ast> Visit<'ast> for Visitor<'_> {
    fn visit_expr_method_call(&mut self, node: &'ast ExprMethodCall) {
        let method = node.method.to_string();
        if method == "unwrap" {
            self.report.has_unsafe_unwrap = true;
        }
        if method == "expect" {
            self.report.has_expect = true;
        }
        if method == "get" {
            self.report.safe_guards += 1;
        }
        syn::visit::visit_expr_method_call(self, node);
    }

    fn visit_expr(&mut self, node: &'ast Expr) {
        if matches!(node, Expr::Index(_)) {
            self.report.has_unchecked_index = true;
        }
        if matches!(node, Expr::If(_)) {
            self.report.cyclomatic_complexity += 1;
        }
        syn::visit::visit_expr(self, node);
    }
}

pub fn analyze_code(code: &str) -> Result<AnalysisReport> {
    let mut report = AnalysisReport {
        cyclomatic_complexity: 1,
        ..Default::default()
    };
    if let Ok(syntax) = syn::parse_str::<syn::File>(code) {
        let mut visitor = Visitor {
            report: &mut report,
        };
        visitor.visit_file(&syntax);
    } else {
        if code.contains(".unwrap()") {
            report.has_unsafe_unwrap = true;
        }
        if code.contains("[0]") || code.contains("[1]") {
            report.has_unchecked_index = true;
        }
        for kw in ["if", "for", "while", "match"] {
            report.cyclomatic_complexity += code.matches(kw).count() as u32;
        }
    }
    if code.contains("is_some")
        || code.contains("is_none")
        || code.contains("if let")
        || code.contains('?')
    {
        report.safe_guards += 1;
        report.null_checks += 1;
    }
    if code.contains("checked_") || code.contains("saturating_") {
        report.safe_guards += 2;
    }
    Ok(report)
}

pub fn analyze_sql_validity(sql: &str) -> Result<bool> {
    use sqlparser::{dialect::GenericDialect, parser::Parser};
    let dialect = GenericDialect {};
    Ok(Parser::parse_sql(&dialect, sql).is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_unwrap_via_syn() {
        let r = analyze_code("fn f(o: Option<i32>){ let _ = o.unwrap(); }").unwrap();
        assert!(r.has_unsafe_unwrap);
    }

    #[test]
    fn test_safe_code() {
        let r = analyze_code("fn f(o: Option<i32>){ if let Some(v)=o { let _=v; } }").unwrap();
        assert!(r.safe_guards > 0);
        assert!(!r.has_unsafe_unwrap);
    }
}
