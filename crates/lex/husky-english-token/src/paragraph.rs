use crate::*;

pub(crate) fn produce_paragraph_starts(tokens: &[EnglishToken]) -> Vec<usize> {
    let line_starts = produce_line_starts(tokens);
    let mut i = 0;
    let mut paragraph_starts = vec![];
    while i < line_starts.len() {
        let line_start0 = line_starts[i];
        let line_indent0 = tokens[line_start0].range().start.col.0;
        paragraph_starts.push(line_start0);
        i = {
            let mut j = i + 1;
            while j < line_starts.len() {
                let line_start1 = line_starts[j];
                let line_start_token = &tokens[line_start1];
                let line_indent1 = line_start_token.range().start.col.0;
                if line_indent1 > line_indent0 {
                    // detect an indentation
                    j += 1;
                } else {
                    if line_indent1 == line_indent0 {
                        match line_start_token.variant() {
                            EnglishTokenVariant::Special(EnglishSpecialToken::RightCurly)
                            | EnglishTokenVariant::Special(EnglishSpecialToken::RightBox) => j += 1,
                            _ => break,
                        }
                    }
                }
            }
            j
        }
    }
    paragraph_starts
}

fn produce_line_starts(tokens: &[EnglishToken]) -> Vec<usize> {
    (0..tokens.len())
        .filter_map(|line_start| {
            if line_start == 0 {
                Some(0)
            } else if tokens[line_start - 1].range().end.line
                < tokens[line_start].range().start.line
            {
                Some(line_start)
            } else {
                None
            }
        })
        .collect()
}
