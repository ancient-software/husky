mod transformations;

use crate::*;
use all_llms::transformation::AllLlmsStringTransformationRecord;
use all_llms::{model::AllLlmModel, AllLlmsClient};
use eterned::db::EternerDb;
use input::VdPipelineInput;
use std::sync::Arc;
use transformations::{elaboration_transformations, simplification_transformations};

pub struct VdPipelineExecutor<'a, 'db> {
    input: &'a VdPipelineInput,
    config: &'a VdPipelineConfig,
    llm_client: AllLlmsClient<'db>,
    raw_solution: Option<String>,
    simplified_solution: Option<(Vec<AllLlmsStringTransformationRecord>, String)>,
    elaborated_solution: Option<(Vec<AllLlmsStringTransformationRecord>, String)>,
}

impl<'a, 'db> VdPipelineExecutor<'a, 'db> {
    pub fn new(
        db: &'db EternerDb,
        input: &'a VdPipelineInput,
        config: &'a VdPipelineConfig,
    ) -> Self {
        let base = input.file_path.parent().unwrap();
        let cache_dir = config.data.cache_dir.to_logical_path(base).join(format!(
            "{}/example-{}",
            input.file_path.file_stem().unwrap().to_str().unwrap(),
            input.index
        ));
        std::fs::create_dir_all(&cache_dir).unwrap();
        Self {
            input,
            config,
            llm_client: AllLlmsClient::new(db, cache_dir).unwrap(),
            raw_solution: None,
            simplified_solution: None,
            elaborated_solution: None,
        }
    }
}

impl<'a, 'db> VdPipelineExecutor<'a, 'db> {
    pub(crate) fn execute_all(&mut self) {
        self.query_raw_solution();
    }

    fn query_raw_solution(&mut self) {
        let prompt = format!(
            r#"Please provide the raw solution to the following problem. The solution should be a complete mathematical proof written in LaTeX, using forward reasoning - meaning each step should build upon previous steps to reach the conclusion, rather than working backwards from what we want to prove.

```latex
{}
```

Provide only the LaTeX code for the solution, without any surrounding text. Do not include \begin{{document}}, \end{{document}}, \begin{{proof}}, or \end{{proof}}. The solution should:
- Start from given information and progress logically forward to the conclusion
- Show each step's reasoning clearly
- Build upon previous steps in a natural progression
- Use appropriate mathematical notation and LaTeX environments
- Avoid unnecessary labels or references"#,
            self.input.content
        );
        // TODO: use config
        let model = AllLlmModel::GEMINI_1_5_FLASH;
        self.raw_solution = Some(self.llm_client.generate_text(model, prompt).unwrap());
        self.simplified_solution = Some(
            self.llm_client
                .apply_transformations_sequentially(
                    &simplification_transformations(),
                    format!(
                        r#"```latex
{}
```"#,
                        self.input.content
                    ),
                )
                .unwrap(),
        );
        self.elaborated_solution = Some(
            self.llm_client
                .apply_transformations_sequentially(
                    &elaboration_transformations(),
                    self.raw_solution.as_ref().unwrap().clone(),
                )
                .unwrap(),
        );
        // Some(extract_latex(
        //     &self.llm_client.generate_text(model, prompt).unwrap(),
        // ));
    }

    pub(crate) fn finish(
        self,
    ) -> (
        String,
        (Vec<AllLlmsStringTransformationRecord>, String),
        (Vec<AllLlmsStringTransformationRecord>, String),
    ) {
        (
            self.raw_solution.unwrap(),
            self.simplified_solution.unwrap(),
            self.elaborated_solution.unwrap(),
        )
    }
}

fn extract_latex(s: &str) -> String {
    if let (Some(start), Some(end)) = (s.find("```latex"), s.rfind("```")) {
        let content = &s[start + 8..end];
        content.trim().to_string()
    } else {
        s.to_string()
    }
}

#[test]
fn extract_latex_works() {
    #[track_caller]
    fn t(input: &str, expected: &str) {
        assert_eq!(extract_latex(input), expected);
    }
    t(
        r"
Here's the solution:

```latex
\begin{align*}
x^2 + 2x + 1 = 0
\end{align*}
```
",
        r"\begin{align*}
x^2 + 2x + 1 = 0
\end{align*}",
    );
}
