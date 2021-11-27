use crate::{Diagnostic, DiagnosticsContext, Severity};

// Diagnostic: malformed-derive
//
// This diagnostic is shown when the derive attribute has invalid input.
pub(crate) fn malformed_derive(
    ctx: &DiagnosticsContext<'_>,
    d: &semantics::MalformedDerive,
) -> Diagnostic {
    let display_range = ctx.sema.diagnostics_display_range(d.node.clone()).range;

    Diagnostic::new(
        "malformed-derive",
        "malformed derive input, derive attributes are of the form `#[derive(Derive1, Derive2, ...)]`",
        display_range,
    )
    .severity(Severity::Error)
}

#[cfg(test)]
mod tests {
    use crate::tests::check_diagnostics;

    #[test]
    fn invalid_input() {
        check_diagnostics(
            r#"
//- minicore:derive
mod __ {
    #[derive = "aaaa"]
  //^^^^^^^^^^^^^^^^^^ error: malformed derive input, derive attributes are of the form `#[derive(Derive1, Derive2, ...)]`
    struct Foo;
}
            "#,
        );
    }
}
