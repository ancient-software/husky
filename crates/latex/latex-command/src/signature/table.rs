use super::*;
use crate::path::{
    menu::{command_path_menu, LxCommandPathMenu},
    LxCommandName,
};
use latex_prelude::mode::LxMode;
use parameter::{LxCommandParameter, LxCommandParameterMode};
use rustc_hash::FxHashMap;

#[salsa::derive_debug_with_db]
#[derive(Debug)]
pub struct LxCommandSignatureTable {
    pub signatures: FxHashMap<LxCommandName, LxCommandSignature>,
}

impl std::ops::Index<LxCommandName> for LxCommandSignatureTable {
    type Output = LxCommandSignature;

    fn index(&self, index: LxCommandName) -> &Self::Output {
        &self.signatures[&index]
    }
}

impl LxCommandSignatureTable {
    fn new(value: &[(LxCommandPath, &[LxCommandParameterMode])]) -> Self {
        Self {
            signatures: value
                .into_iter()
                .copied()
                .map(|(path, parameter_modes)| {
                    (
                        path.name(),
                        LxCommandSignature {
                            path,
                            parameters: parameter_modes
                                .into_iter()
                                .copied()
                                .map(LxCommandParameter::new)
                                .collect(),
                        },
                    )
                })
                .collect(),
        }
    }
}

impl std::ops::Deref for LxCommandSignatureTable {
    type Target = FxHashMap<LxCommandName, LxCommandSignature>;

    fn deref(&self) -> &Self::Target {
        &self.signatures
    }
}

impl LxCommandSignatureTable {
    pub fn new_default(db: &salsa::Db) -> Self {
        use LxCommandParameterMode::*;

        let LxCommandPathMenu {
            alpha_path,
            beta_path,
            gamma_path,
            pi_path,
            sqrt_path,
            sin_path,
            cos_path,
            frac_path,
            text_path,
        } = *command_path_menu(db);
        Self::new(&[
            (alpha_path, &[]),
            (beta_path, &[]),
            (gamma_path, &[]),
            (pi_path, &[]),
            (sqrt_path, &[Math]),
            (sin_path, &[]),
            (cos_path, &[]),
            (frac_path, &[Math, Math]),
            (text_path, &[Rose]),
        ])
    }
}
