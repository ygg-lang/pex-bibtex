use super::*;

use pex::{
    helpers::{make_from_str, whitespace},
    ParseResult, ParseState, StopBecause,
};

impl FromStr for Bibliography {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, StopBecause> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, parse_bibliography)
    }
}

/// ```text
/// bibliography = @entry_tag { citation_key tags }
/// tags         = , pair (, pair)+ (,)?
/// ```
pub fn parse_bibliography(input: ParseState) -> ParseResult<Bibliography> {
    let (state, entry_type) = parse_at_mark(input)?;
    let (state, _) = state.skip(whitespace).match_char('{')?;
    let (state, citation_key) = state
        .skip(whitespace)
        .match_str_if(|c| c != ',' && c != '}', "CITATION_KEY")
        .map_inner(|s| s.trim_end().to_string())?;
    let (state, tags) = state.match_repeats(parse_comma_pair)?;
    let (state, _) = state.match_optional(|state| state.skip(whitespace).match_char(','))?;
    let (state, _) = state.skip(whitespace).match_char('}')?;
    state.finish(Bibliography { entry_type, citation_key, tags: tags.into_iter().collect() })
}

fn parse_at_mark(input: ParseState) -> ParseResult<String> {
    let (state, _) = input.match_char('@')?;
    let (state, entry_type) = state.match_str_if(|c| c.is_alphabetic(), "ENTRY_TYPE")?;
    state.finish(entry_type.to_string())
}

#[test]
fn test_at() {
    let input = ParseState::new("@book {");
    let entry_type = parse_at_mark(input).as_result().unwrap().1;
    assert_eq!(entry_type, "book");
}

fn parse_comma_pair(input: ParseState) -> ParseResult<(String, String)> {
    let (state, _) = input.skip(whitespace).match_char(',')?;
    let (state, pair) = state.skip(whitespace).match_fn(parse_pair)?;
    state.finish(pair)
}

// , pair (, pair)? ,? }
fn parse_pair(input: ParseState) -> ParseResult<(String, String)> {
    let (state, key) = input.match_str_if(|c| c.is_alphabetic(), "KEY")?;
    let (state, _) = state.skip(whitespace).match_char('=')?;
    let (state, value) = state.skip(whitespace).match_fn(parse_curly)?;
    state.finish((key.to_string(), value.to_string()))
}

#[test]
fn test_pair() {
    let input = ParseState::new("key = {a{b}c}");
    let pair1 = parse_pair(input).as_result().unwrap().1;
    assert_eq!(pair1, ("key".to_string(), "a{b}c".to_string()));
    let input = ParseState::new(",\nkey = {a{b}c}");
    let pair2 = parse_comma_pair(input).as_result().unwrap().1;
    assert_eq!(pair2, ("key".to_string(), "a{b}c".to_string()));
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

#[test]
fn test_curly() {
    let input = ParseState::new("{a{b}c}d");
    let curly = parse_curly(input).as_result().unwrap().1;
    assert_eq!(curly, "a{b}c");
}
