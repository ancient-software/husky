use std::path::Path;

use super::*;
use crate::path::{menu::lx_command_path_menu, LxCommandName};
use base_coword::BaseCoword;
use eterned::db::EternerDb;
use latex_prelude::mode::LxMode;
use lisp_csv::{
    expr::LpCsvExprData,
    file::{LpCsvFile, LpCsvFileData},
    row::LpCsvRow,
};
use parameter::{LxCommandParameter, LxCommandParameterMode};
use path::menu::LxCommandPathMenu;
use rustc_hash::FxHashMap;

#[derive(Debug)]
pub struct LxCommandSignatureTable {
    pub signatures: FxHashMap<LxCommandName, LxCommandSignature>,
}

impl LxCommandSignatureTable {
    // TODO: return a closest match if the command is not found
    pub fn signature(&self, index: LxCommandName) -> Option<&LxCommandSignature> {
        self.signatures.get(&index)
    }
}

impl LxCommandSignatureTable {
    fn new(
        begin: LxCommandPath,
        end: LxCommandPath,
        left: LxCommandPath,
        right: LxCommandPath,
        letter_style_commands: &[(LxCommandPath, LxMathLetterStyle)],
        complete_commands: impl IntoIterator<
            Item = (
                LxCommandPath,
                impl AsRef<[LxMode]>,
                impl AsRef<[LxCommandParameterMode]>,
            ),
        >,
    ) -> Self {
        Self {
            signatures: [
                (begin.name(), LxCommandSignature::Begin),
                (end.name(), LxCommandSignature::End),
                (left.name(), LxCommandSignature::Left),
                (right.name(), LxCommandSignature::Right),
            ]
            .into_iter()
            .chain(
                letter_style_commands
                    .iter()
                    .copied()
                    .map(|(path, style)| (path.name(), LxCommandSignature::MathLetterStyle(style))),
            )
            .chain(
                complete_commands
                    .into_iter()
                    .map(|(path, allowed_modes, parameter_modes)| {
                        (
                            path.name(),
                            LxCommandSignature::Complete(LxCompleteCommandSignature {
                                path,
                                allowed_modes: allowed_modes.as_ref().iter().copied().collect(),
                                options: (),
                                parameters: parameter_modes
                                    .as_ref()
                                    .iter()
                                    .copied()
                                    .map(LxCommandParameter::new)
                                    .collect(),
                            }),
                        )
                    }),
            )
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
    fn default_complete_commands(
        db: &EternerDb,
    ) -> Vec<(
        LxCommandPath,
        &'static [LxMode],
        &'static [LxCommandParameterMode],
    )> {
        let LxCommandPathMenu {
            // - root
            usepackage,
            documentclass,
            newtheorem,
            // - divisions
            part,
            chapter,
            section,
            subsection,
            subsubsection,
            // - maths
            // ## letter style
            mathbb,
            mathbf,
            mathcal,
            mathit,
            mathrm,
            mathsf,
            mathscr,
            // - operators
            // -- relations
            eq,
            ne,
            neq,
            le,
            leq,
            ge,
            geq,
            r#in,
            subset,
            supset,
            subseteq,
            supseteq,
            subseteqq,
            supseteqq,
            subsetneq,
            supsetneq,
            // -- arithmetic
            cdot,
            int,
            sum,
            prod,
            times,
            otimes,
            // -- extended letters
            alpha,
            beta,
            gamma,
            pi,
            // -- functions
            sin,
            cos,
            // -- layouts
            sqrt,
            frac,
            text,
            ..
        } = *lx_command_path_menu(db);
        vec![
            // - root
            (usepackage, &[LxMode::Root], &[LxCommandParameterMode::Name]),
            (
                documentclass,
                &[LxMode::Root],
                &[LxCommandParameterMode::Name],
            ),
            (
                newtheorem,
                &[LxMode::Root],
                &[LxCommandParameterMode::Name, LxCommandParameterMode::Name],
            ),
            // - divisions
            (part, &[LxMode::Root], &[LxCommandParameterMode::Rose]),
            (chapter, &[LxMode::Root], &[LxCommandParameterMode::Rose]),
            (section, &[LxMode::Root], &[LxCommandParameterMode::Rose]),
            (subsection, &[LxMode::Root], &[LxCommandParameterMode::Rose]),
            (
                subsubsection,
                &[LxMode::Root],
                &[LxCommandParameterMode::Rose],
            ),
            // - operators
            // -- relations
            (eq, &[LxMode::Math], &[]),
            (ne, &[LxMode::Math], &[]),
            (neq, &[LxMode::Math], &[]),
            (le, &[LxMode::Math], &[]),
            (leq, &[LxMode::Math], &[]),
            (ge, &[LxMode::Math], &[]),
            (geq, &[LxMode::Math], &[]),
            (r#in, &[LxMode::Math], &[]),
            (subset, &[LxMode::Math], &[]),
            (supset, &[LxMode::Math], &[]),
            (subseteq, &[LxMode::Math], &[]),
            (supseteq, &[LxMode::Math], &[]),
            (subseteqq, &[LxMode::Math], &[]),
            (supseteqq, &[LxMode::Math], &[]),
            (subsetneq, &[LxMode::Math], &[]),
            (supsetneq, &[LxMode::Math], &[]),
            // -- arithmetic
            (cdot, &[LxMode::Math], &[]),
            (int, &[LxMode::Math], &[]),
            (sum, &[LxMode::Math], &[]),
            (prod, &[LxMode::Math], &[]),
            (times, &[LxMode::Math], &[]),
            (otimes, &[LxMode::Math], &[]),
            // -- extended letters
            (alpha, &[LxMode::Math], &[]),
            (beta, &[LxMode::Math], &[]),
            (gamma, &[LxMode::Math], &[]),
            (pi, &[LxMode::Math], &[]),
            // -- functions
            (sqrt, &[LxMode::Math], &[LxCommandParameterMode::Math]),
            (sin, &[LxMode::Math], &[]),
            (cos, &[LxMode::Math], &[]),
            // -- layouts
            (
                frac,
                &[LxMode::Math],
                &[LxCommandParameterMode::Math, LxCommandParameterMode::Math],
            ),
            (text, &[LxMode::Math], &[LxCommandParameterMode::Rose]),
        ]
    }

    pub fn new_from_lp_csv_file_paths(complete_commands_path: &Path, db: &EternerDb) -> Self {
        use lisp_csv::parse_lp_csv_filepath;

        Self::new_from_csv_files(
            &parse_lp_csv_filepath(complete_commands_path).expect("todo"),
            db,
        )
    }

    pub fn new_from_csv_files(complete_commands_file: &LpCsvFile, db: &EternerDb) -> Self {
        let LxCommandPathMenu {
            // - root
            begin,
            end,
            usepackage,
            documentclass,
            newtheorem,
            // - divisions
            part,
            chapter,
            section,
            subsection,
            subsubsection,
            // - maths
            // ## letter style
            mathbb,
            mathbf,
            mathcal,
            mathit,
            mathrm,
            mathsf,
            mathscr,
            left,
            right,
            ..
        } = *lx_command_path_menu(db);
        Self::new(
            begin,
            end,
            left,
            right,
            &[
                (mathbb, LxMathLetterStyle::MATHBB),
                (mathbf, LxMathLetterStyle::MATHFRAK),
                (mathcal, LxMathLetterStyle::MATHCAL),
                (mathit, LxMathLetterStyle::MATHIT),
                (mathrm, LxMathLetterStyle::MATHRM),
                (mathsf, LxMathLetterStyle::MATHSF),
                (mathscr, LxMathLetterStyle::MATHSCR),
            ],
            Self::complete_commands_from_csv_file(complete_commands_file, db),
        )
    }

    fn complete_commands_from_csv_file<'a>(
        complete_commands_file: &'a LpCsvFile,
        db: &'a EternerDb,
    ) -> impl Iterator<Item = (LxCommandPath, Vec<LxMode>, Vec<LxCommandParameterMode>)> + 'a {
        let LpCsvFileData::Rows(rows) = complete_commands_file.data();
        Self::complete_commands_from_csv_rows(rows, db)
    }

    fn complete_commands_from_csv_rows<'a>(
        rows: &'a [LpCsvRow],
        db: &'a EternerDb,
    ) -> impl Iterator<Item = (LxCommandPath, Vec<LxMode>, Vec<LxCommandParameterMode>)> + 'a {
        rows.iter()
            .map(|row| Self::complete_command_from_csv_row(row, db))
    }

    fn complete_command_from_csv_row(
        row: &LpCsvRow,
        db: &EternerDb,
    ) -> (LxCommandPath, Vec<LxMode>, Vec<LxCommandParameterMode>) {
        let LpCsvRow::SeparatedExprs(exprs) = row else {
            todo!("row = {:?}", row)
        };
        let [command_ident, allowed_modes, parameter_modes] = exprs.as_slice() else {
            todo!()
        };
        let LpCsvExprData::Ident(ref command_ident) = command_ident.data else {
            todo!()
        };
        // TODO: ad hoc
        let command_path = LxCommandPath::new_prelude(BaseCoword::from_ref(command_ident, db), db);
        let LpCsvExprData::List(ref allowed_modes) = allowed_modes.data else {
            todo!()
        };
        let allowed_modes: Vec<LxMode> = allowed_modes
            .iter()
            .map(|s| {
                let LpCsvExprData::Ident(ref ident) = s.data else {
                    todo!()
                };
                match ident.as_ref() {
                    "root" => LxMode::Root,
                    "math" => LxMode::Math,
                    "rose" => LxMode::Rose,
                    "name" => LxMode::Name,
                    _ => todo!(),
                }
            })
            .collect();
        let LpCsvExprData::List(ref parameter_modes) = parameter_modes.data else {
            todo!()
        };
        let parameter_modes: Vec<LxCommandParameterMode> = parameter_modes
            .iter()
            .map(|s| {
                let LpCsvExprData::Ident(ref ident) = s.data else {
                    todo!()
                };
                match ident.as_ref() {
                    "name" => LxCommandParameterMode::Name,
                    "math" => LxCommandParameterMode::Math,
                    "rose" => LxCommandParameterMode::Rose,
                    "single_letter" => LxCommandParameterMode::SingleLetter,
                    _ => todo!(),
                }
            })
            .collect();
        (command_path, allowed_modes, parameter_modes)
    }
}

#[test]
fn lx_command_signature_table_works() {
    use husky_path_utils::HuskyLangDevPaths;

    let db = &EternerDb::default();
    let dev_paths = HuskyLangDevPaths::new();
    let complete_commands_path = &dev_paths.specs_dir().join("latex/complete-commands.lpcsv");
    let table = LxCommandSignatureTable::new_from_lp_csv_file_paths(complete_commands_path, db);
    for (path, allowed_modes, parameter_modes) in
        LxCommandSignatureTable::default_complete_commands(db)
    {
        let Some(signature) = table.signature(path.name()) else {
            todo!()
        };
        let LxCommandSignature::Complete(ref complete_signature) = signature else {
            todo!()
        };
        assert_eq!(
            complete_signature.allowed_modes,
            allowed_modes.iter().copied().collect()
        );
        assert_eq!(
            complete_signature.parameters.as_slice(),
            parameter_modes
                .as_ref()
                .iter()
                .copied()
                .map(LxCommandParameter::new)
                .collect::<Vec<_>>()
        );
    }
}
