//! Migration code for the `expr_fragment_specifier_2024`
//! rule.
use rustc_ast::token::Token;
use rustc_ast::token::TokenKind;
use rustc_ast::tokenstream::TokenStream;
use rustc_ast::tokenstream::TokenTree;
use rustc_session::declare_lint;
use rustc_session::declare_lint_pass;
use rustc_session::lint::FutureIncompatibilityReason;
use rustc_span::edition::Edition;
use rustc_span::sym;

use crate::lints::MacroExprFragment2023;
use crate::EarlyLintPass;

declare_lint! {
    pub EDITION_2024_EXPR_FRAGMENT_SPECIFIER,
    Allow,
    "The `expr` fragment specifier will accept more expressions in the 2024 edition. \
    To keep the existing before, use the `expr_2021` fragment specifier.",
    @future_incompatible = FutureIncompatibleInfo {
        reason: FutureIncompatibilityReason::EditionSemanticsChange(Edition::Edition2024),
        reference: "issue #123742 <https://github.com/rust-lang/rust/issues/123742>",
    };
}

declare_lint_pass!(Expr2024 => [EDITION_2024_EXPR_FRAGMENT_SPECIFIER,]);

impl Expr2024 {
    fn check_tokens(&mut self, cx: &crate::EarlyContext<'_>, tokens: &TokenStream) {
        // Check if the preceding token is `$`, because we want to allow `$async`, etc.
        let mut prev_dollar = false;
        for tt in tokens.trees() {
            match tt {
                // Only report non-raw idents.
                TokenTree::Token(token, _) => {
                    if token.kind == TokenKind::Dollar {
                        prev_dollar = true;
                        continue;
                    } else {
                        if !prev_dollar {
                            self.check_ident_token(cx, token);
                        }
                    }
                }
                TokenTree::Delimited(.., tts) => self.check_tokens(cx, tts),
            }
            prev_dollar = false;
        }
    }

    fn check_ident_token(&mut self, cx: &crate::EarlyContext<'_>, token: &Token) {
        debug!("check_ident_token: {:?}", token);
        let (sym, edition) = match token.kind {
            TokenKind::Ident(sym, _) => (sym, Edition::Edition2024),
            _ => return,
        };

        // Don't lint `r#foo`.
        debug!("token.span.edition(): {:?}", token.span.edition());
        if token.span.edition() >= edition {
            return;
        }

        if sym != sym::expr {
            return;
        }

        debug!("emitting lint");
        cx.builder.emit_span_lint(
            &EDITION_2024_EXPR_FRAGMENT_SPECIFIER,
            token.span.into(),
            MacroExprFragment2023 { suggestion: token.span },
        );
    }
}

impl EarlyLintPass for Expr2024 {
    fn check_mac_def(&mut self, cx: &crate::EarlyContext<'_>, mc: &rustc_ast::MacroDef) {
        self.check_tokens(cx, &mc.body.tokens);
    }
}
