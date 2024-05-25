use rustc_span::edition::Edition;
use rustc_ast::token::Token;


declare_lint! {
    pub EXPR_2024,
    Allow,
    "TODO: add a comment here",
    @future_incompatible = FutureIncompatibleInfo {
        reason: FutureIncompatibilityReason::EditionError(Edition::Edition2024),
        reference: "issue #123742 <https://github.com/rust-lang/rust/issues/123742>",
    };
}

declare_lint_pass!(
    Expr2024 => [ EXPR_2024 ]
);

impl Expr2024 {
    fn check_tokens(&mut self, cx: &EarlyContext<'_>, tokens: &TokenStream) {
        let mut prev_dollar = false;
        for tt in tokens.trees() {
            if let Some((ident, token::IdentIsRaw::No)) = token.ident() {
                if !prev_dollar {
                    self.check_ident_token(cx, ident);
                } else if token.kind == TokenKind::Dollar {
                    prev_dollar = true;
                    continue;
                }
                prev_dollar = false;
            }
        }
    }

    fn check_ident_token(
        &mut self,
        cx: &EarlyContext<'_>,
        UnderMacro(under_macro): UnderMacro,
        token: Token,
    ) {
        let (lint, edition) = match ident.kind() {
            sym::Expr2024 => (EXPR_2024, Edition::Edition2021),
            _ => return,
        };

        if token.span().edition() >= edition {
            return;
        }

        cx.emit_span_lint(
            lint,
            ident.span,
            ()
        );
    }
}

impl EarlyLintPass for Expr2024 {
    fn check_mac_def(&mut self, cx: &EarlyContext<'_>, mac_def: &ast::MacroDef) {
        self.check_tokens(cx, &mac_def.body.tokens);
    }
}

