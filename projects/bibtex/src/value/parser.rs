// @book{texbook,
//     author = {Donald E. Knuth},
//     year = {1986},
//     title = {The {\TeX} Book},
//     publisher = {Addison-Wesley Professional}
// }

use super::*;

use pex::{
    helpers::{make_from_str, whitespace},
    ParseResult, ParseState, StopBecause,
};

impl FromStr for Bibliography {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, parse_bibliography)
    }
}

#[test]
fn test_tex_book() {
    let texbook = r#"
    @book{texbook,
    author = {Donald E. Knuth},
    year = {1986},
    title = {The {\TeX} Book},
    publisher = {Addison-Wesley Professional}
}
"#;
    let bib = Bibliography::from_str(texbook).unwrap();
    println!("{}", bib);
}

/// ```text
/// bibliography = `@entry_tag { citation_key tags }`
/// tags         = , pair (, pair)+ (,)?
/// ```
pub fn parse_bibliography(input: ParseState) -> ParseResult<Bibliography> {
    let (state, _) = input.match_char('@')?;
    let (state, entry_type) = state.match_str_if(|c| c.is_alphabetic(), "ENTRY_TYPE").map_inner(|s| s.to_string())?;
    let (state, _) = state.skip(whitespace).match_char('{')?;
    let (state, citation_key) =
        state.skip(whitespace).match_str_if(|c| c != ',', "CITATION_KEY").map_inner(|s| s.trim_end().to_string())?;
    let (state, tags) = state.match_repeats(parse_comma_pair)?;
    let (state, _) = state.match_optional(|state| state.skip(whitespace).match_char(','))?;
    let (state, _) = state.skip(whitespace).match_char('}')?;
    state.finish(Bibliography { entry_type, citation_key, tags: tags.into_iter().collect() })
}

fn parse_comma_pair(input: ParseState) -> ParseResult<(String, String)> {
    input.skip(whitespace).match_char(',')?.0.match_fn(parse_pair)
}

// , pair (, pair)? ,? }
fn parse_pair(input: ParseState) -> ParseResult<(String, String)> {
    let (state, key) = input.match_str_if(|c| c.is_alphabetic(), "KEY")?;
    let (state, _) = state.skip(whitespace).match_char('=')?;
    let (state, value) = state.skip(whitespace).match_fn(parse_curly)?;
    state.finish((key.to_string(), value.to_string()))
}

fn parse_curly<'i>(input: ParseState<'i>) -> ParseResult<&'i str> {
    if !input.rest_text.starts_with('{') {
        StopBecause::custom_error("missing start mark `{`", input.start_offset)?;
    }
    let mut nested = 0;
    let mut offset = 0;
    for char in input.rest_text.chars() {
        match char {
            '{' => {
                nested += 1;
                offset += 1
            }
            '}' => {
                nested -= 1;
                offset += 1;
                if nested == 0 {
                    break;
                }
            }
            _ => offset += char.len_utf8(),
        }
    }
    if nested != 0 {
        StopBecause::custom_error("missing end mark `}`", input.start_offset + offset)?;
    }
    let (state, curly) = input.advance_view(offset)?;
    state.finish(&curly[1..curly.len() - 1])
}
