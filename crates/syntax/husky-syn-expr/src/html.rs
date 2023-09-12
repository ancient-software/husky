use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SynHtmlArgumentExpr {
    Expanded {
        property_ident: RegionalIdentToken,
        eq: RegionalEqToken,
        lcurl: RegionalLCurlToken,
        expr: SynExprIdx,
        rcurl: RegionalRCurlToken,
    },
    Shortened {
        lcurl: RegionalLCurlToken,
        property_ident: RegionalIdentToken,
        rcurl: RegionalRCurlToken,
    },
}

impl vec_like::AsVecMapEntry for SynHtmlArgumentExpr {
    type K = Ident;

    fn key(&self) -> Self::K
    where
        Self::K: Copy,
    {
        match self {
            SynHtmlArgumentExpr::Expanded { property_ident, .. }
            | SynHtmlArgumentExpr::Shortened { property_ident, .. } => property_ident.ident(),
        }
    }

    fn key_ref(&self) -> &Self::K {
        match self {
            SynHtmlArgumentExpr::Expanded {
                property_ident: argument_ident,
                ..
            }
            | SynHtmlArgumentExpr::Shortened {
                property_ident: argument_ident,
                ..
            } => argument_ident.ident_ref(),
        }
    }
}
