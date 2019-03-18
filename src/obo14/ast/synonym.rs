use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use iri_string::Url;
use pest::iterators::Pair;

use super::super::parser::FromPair;
use super::super::parser::Parser;
use super::super::parser::Rule;
use super::QuotedString;
use super::SynonymTypeId;
use super::Xref;
use crate::error::Error;
use crate::error::Result;

/// A synonym scope specifier.
#[derive(Debug, Eq, Hash, PartialEq)]
pub enum SynonymScope {
    Exact,
    Broad,
    Narrow,
    Related,
}

impl Display for SynonymScope {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::SynonymScope::*;
        match self {
            Exact => f.write_str("EXACT"),
            Broad => f.write_str("BROAD"),
            Narrow => f.write_str("NARROW"),
            Related => f.write_str("RELATED"),
        }
    }
}

impl FromPair for SynonymScope {
    const RULE: Rule = Rule::SynonymScope;
    unsafe fn from_pair_unchecked(pair: Pair<Rule>) -> Result<Self> {
        match pair.as_str() {
            "EXACT" => Ok(SynonymScope::Exact),
            "BROAD" => Ok(SynonymScope::Broad),
            "NARROW" => Ok(SynonymScope::Narrow),
            "RELATED" => Ok(SynonymScope::Related),
            _ => unreachable!(),
        }
    }
}
impl_fromstr!(SynonymScope);

/// A synonym, denoting an alternative name for the embedding entity.
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Synonym {
    text: QuotedString,
    scope: SynonymScope,
    syntype: Option<SynonymTypeId>,
    xrefs: Option<Vec<Xref>>,
}

impl Display for Synonym {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.text
            .fmt(f)
            .and(f.write_char(' '))
            .and(self.scope.fmt(f))?;

        if let Some(ref syntype) = self.syntype {
            f.write_char(' ').and(syntype.fmt(f))?;
        }

        if let Some(ref xrefs) = self.xrefs {
            f.write_str(" [")?;
            let mut it = xrefs.iter().peekable();
            while let Some(xref) = it.next() {
                xref.fmt(f)?;
                if it.peek().is_some() {
                    f.write_str(", ")?;
                }
            }
            f.write_char(']')?;
        }

        Ok(())
    }
}
