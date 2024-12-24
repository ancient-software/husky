use super::*;
use llm_prelude::transformation::LlmStringTransformationInstruction;

pub(super) fn simplification_transformations() -> Vec<AllLlmsStringTransformation> {
    vec![
        AllLlmsStringTransformation {
            model: AllLlmModel::GEMINI_1_5_FLASH,
            instruction: LlmStringTransformationInstruction::MainInputSide {
                main: "Please simplify the following mathematical proof:".to_string(),
                side: Some(format!(
                    r#"
    
    There should only be one solution. No need to include alternatives. If the problem is entirely trivial, just need to say that it's trivial that XXX holds.

The solution should move forward logically, building upon previous steps to reach the conclusion. Avoid working backwards from what we want to prove.
    "#
                )),
            },
            examples: vec![],
        },
        AllLlmsStringTransformation {
            model: AllLlmModel::GEMINI_1_5_FLASH,
            instruction: LlmStringTransformationInstruction::MainInputSide {
                main: r#"Make sure the solution doesn't contain any \begin{{document}}, \end{{document}}, \begin{{proof}}, or \end{{proof}}. It is intended to be latex code contained in the document body, not a full document. Make sure to make it valid under latex text mode."#.to_string(),
                side: None,
            },
            examples: vec![],
        },
        AllLlmsStringTransformation {
            model: AllLlmModel::GEMINI_1_5_FLASH,
            instruction: LlmStringTransformationInstruction::MainInputSide {
                main:
                    "Please remove all labels in align environments from the following latex code:"
                        .to_string(),
                side: None,
            },
            examples: vec![],
        },
    ]
}

pub(super) fn visored_preprocessing_transformations() -> Vec<AllLlmsStringTransformation> {
    vec![AllLlmsStringTransformation {
        model: AllLlmModel::GEMINI_1_5_FLASH,
        instruction: LlmStringTransformationInstruction::MainInputSide {
            main: "Remove discussion of equality holds. These are not needed.".to_string(),
            side: None,
        },
        examples: vec![],
    }]
}
