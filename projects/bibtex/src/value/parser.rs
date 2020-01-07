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

pub fn parse_bibliography(input: ParseState) -> ParseResult<Bibliography> {
    let (state, entry_type) = parse_at_item(input)?;
    let (state, citation_key) = parse_curly(state)?;

    state.finish(Bibliography {
        entry_type: entry_type.to_string(),
        citation_key: citation_key.to_string(),
        tags: Default::default(),
    })
}

fn parse_at_item<'i>(input: ParseState<'i>) -> ParseResult<&'i str> {
    let (state, _) = input.match_char('@')?;
    let (state, letter) = state.match_str_if(|c| c.is_alphabetic(), "LETTER")?;
    state.finish(letter)
}

fn parse_curly<'i>(input: ParseState<'i>) -> ParseResult<&'i str> {
    let (state, _) = input.match_char('{')?;
    let (state, content) = state.match_str_if(|c| c != '}', "CONTENT")?;
    let (state, _) = state.match_char('}')?;
    state.finish(content)
}
