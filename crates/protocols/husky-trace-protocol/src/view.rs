pub mod action;
pub mod data_ref;

use crate::*;
use husky_token_protocol::TokenClass;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceViewData {
    pub trace_kind: TraceKind,
    lines_data: Vec<TraceViewLineData>,
    have_subtraces: bool,
}

impl TraceViewData {
    pub fn assoc_trace_ids(&self) -> Vec<TraceId> {
        let mut assoc_trace_ids: Vec<TraceId> = vec![];
        for line_data in &self.lines_data {
            for token_data in &line_data.tokens_data {
                if let Some(assoc_trace_id) = token_data.assoc_trace_id {
                    assoc_trace_ids.push(assoc_trace_id)
                }
            }
        }
        assoc_trace_ids
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceViewLineData {
    tokens_data: Vec<TraceViewTokenData>,
}

impl TraceViewData {
    pub fn new(trace_kind: TraceKind, lines: Vec<TraceViewLineData>, have_subtraces: bool) -> Self {
        Self {
            trace_kind,
            lines_data: lines,
            have_subtraces,
        }
    }

    pub fn lines_data(&self) -> &[TraceViewLineData] {
        self.lines_data.as_ref()
    }

    pub fn have_subtraces(&self) -> bool {
        self.have_subtraces
    }
}

impl TraceViewLineData {
    pub fn new(tokens_data: Vec<TraceViewTokenData>) -> Self {
        Self { tokens_data }
    }

    pub fn tokens_data(&self) -> &[TraceViewTokenData] {
        self.tokens_data.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceViewTokenData {
    text: String,
    token_class: TokenClass,
    spaces_before: u32,
    assoc_trace_id: Option<TraceId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SeparationAfter {
    SameLine { spaces: u32 },
    NextLine { indent: u32 },
    Eof,
}

impl TraceViewTokenData {
    pub fn new(
        text: String,
        token_class: TokenClass,
        spaces_before: u32,
        assoc_trace: Option<TraceId>,
    ) -> Self {
        Self {
            text,
            token_class,
            spaces_before,
            assoc_trace_id: assoc_trace,
        }
    }

    pub fn text(&self) -> &str {
        self.text.as_ref()
    }

    pub fn spaces_before(&self) -> u32 {
        self.spaces_before
    }

    pub fn token_class(&self) -> TokenClass {
        self.token_class
    }

    pub fn assoc_trace_id(&self) -> Option<TraceId> {
        self.assoc_trace_id
    }
}
