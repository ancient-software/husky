use husky_completion::HuskyCompletionQuery;
use husky_diagnostics::{Diagnostic, HuskyDiagnosticQuery};
use husky_hover::HuskyHoverContentsQuery;

use crate::*;

impl HuskyDiagnosticQuery for HuskyComptime {}

impl HuskyHoverContentsQuery for HuskyComptime {}

impl HuskyCompletionQuery for HuskyComptime {}
