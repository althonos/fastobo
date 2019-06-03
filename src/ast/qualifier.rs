use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::str::FromStr;

use pest::iterators::Pair;

use crate::ast::*;
use crate::error::Result;
use crate::parser::FromPair;
use crate::parser::QuickFind;
use crate::parser::Rule;

/// A qualifier, possibly used as a trailing modifier.
#[derive(Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct Qualifier {
    key: RelationIdent,
    value: QuotedString,
}

impl Qualifier {
    /// Create a new `Qualifier` from the given identifier and value.
    pub fn new(key: RelationIdent, value: QuotedString) -> Self {
        Self { key, value }
    }

    /// Get a reference to the key of the qualifier.
    pub fn key(&self) -> &RelationIdent {
        &self.key
    }

    /// Get a mutable reference to the key of the qualifier.
    pub fn key_mut(&mut self) -> &mut RelationIdent {
        &mut self.key
    }

    /// Get a reference to the value of the qualifier.
    pub fn value(&self) -> &QuotedString {
        &self.value
    }

    /// Get a mutable reference to the value of the qualifier.
    pub fn value_mut(&mut self) -> &mut QuotedString {
        &mut self.value
    }
}

impl Display for Qualifier {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.key
            .fmt(f)
            .and(f.write_char('='))
            .and(self.value.fmt(f))
    }
}

impl<'i> FromPair<'i> for Qualifier {
    const RULE: Rule = Rule::Qualifier;
    unsafe fn from_pair_unchecked(pair: Pair<Rule>) -> Result<Self> {
        let mut inner = pair.into_inner();
        let key = RelationIdent::from_str(inner.next().unwrap().as_str())?;
        let value = QuotedString::from_pair_unchecked(inner.next().unwrap())?;
        Ok(Qualifier { key, value })
    }
}
impl_fromstr!(Qualifier);

impl Identified for Qualifier {
    fn as_id(&self) -> &Ident {
        self.key.as_ref()
    }
    fn as_id_mut(&mut self) -> &mut Ident {
        self.key.as_mut()
    }
}

/// A list containing zero or more `Qualifier`s.
#[derive(Clone, Default, Debug, Hash, Eq, OpaqueTypedef, Ord, PartialEq, PartialOrd)]
#[opaque_typedef(allow_mut_ref)]
#[opaque_typedef(derive(
    AsRef(Inner, Self),
    AsMut(Inner, Self),
    Deref,
    DerefMut,
    Into(Inner),
    FromInner,
    PartialEq(Inner),
))]
pub struct QualifierList {
    qualifiers: Vec<Qualifier>,
}

impl QualifierList {
    pub fn new(qualifiers: Vec<Qualifier>) -> Self {
        Self { qualifiers }
    }

    pub fn sort(&mut self) {
        self.qualifiers.sort_unstable();
    }
}

impl Display for QualifierList {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let mut qualifiers = self.qualifiers.iter().peekable();
        f.write_char('{')?;
        while let Some(qual) = qualifiers.next() {
            qual.fmt(f)?;
            if qualifiers.peek().is_some() {
                f.write_str(", ")?;
            }
        }
        f.write_char('}')
    }
}

impl<Q> FromIterator<Q> for QualifierList
where
    Q: Into<Qualifier>,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Q>,
    {
        Self::new(iter.into_iter().map(Into::into).collect())
    }
}

impl<'i> FromPair<'i> for QualifierList {
    const RULE: Rule = Rule::QualifierList;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
        let mut qualifiers = Vec::new();
        for pair in pair.into_inner() {
            qualifiers.push(Qualifier::from_pair_unchecked(pair)?);
        }
        Ok(QualifierList { qualifiers })
    }
}
impl_fromstr!(QualifierList);

impl IntoIterator for QualifierList {
    type Item = Qualifier;
    type IntoIter = <Vec<Qualifier> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.qualifiers.into_iter()
    }
}

impl Orderable for QualifierList {
    fn sort(&mut self) {
        self.qualifiers.sort_unstable();
    }
    fn is_sorted(&self) -> bool {
        for i in 1..self.qualifiers.len() {
            if self.qualifiers[i - 1] > self.qualifiers[i] {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn from_str() {
        let actual = Qualifier::from_str("comment=\"NYBG:Dario_Cavaliere\"").unwrap();
        let expected = Qualifier::new(
            RelationIdent::from(Ident::from(UnprefixedIdent::new(String::from("comment")))),
            QuotedString::new(String::from("NYBG:Dario_Cavaliere")),
        );
        assert_eq!(actual, expected);
    }

    mod list {

        use super::*;

        #[test]
        fn from_str() {
            // FIXME(@althonos)
            match QualifierList::from_str(
                "{comment=\"NYBG:Dario_Cavaliere\", comment=\"NYBG:Brandon_Sinn\"}",
            ) {
                Ok(_) => (),
                Err(e) => panic!("{}", e),
            }
        }
    }

}