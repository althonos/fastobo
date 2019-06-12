use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::str::FromStr;

use pest::iterators::Pair;
use url::Url;

use crate::ast::*;
use crate::error::Error;
use crate::error::SyntaxError;
use crate::parser::FromPair;
use crate::parser::Rule;
use crate::share::Cow;
use crate::share::Redeem;
use crate::share::Share;

/// A clause appearing in a header frame.
///
/// Header clauses are used to add metadata to OBO documents. They are all
/// optional, but every document should *at least* contain a `FormatVersion`
/// clause, to help with interoperability and to make sure the semantics of
/// the right OBO specification are in use.
///
/// # Comparison
/// `HeaderClause` implements `PartialOrd` following the semantics of the OBO
/// specification: clauses will compare based on their serialization order
/// rather than on their alphabetic order; clauses of the same kind will be
/// ranked in the alphabetic order.
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "_derive", derive(OboClause))]
pub enum HeaderClause {
    #[cfg_attr(feature = "_derive", clause(tag = "format-version"))]
    FormatVersion(UnquotedString),
    #[cfg_attr(feature = "_derive", clause(tag = "data-version"))]
    DataVersion(UnquotedString),
    Date(NaiveDateTime),
    #[cfg_attr(feature = "_derive", clause(tag = "saved-by"))]
    SavedBy(UnquotedString),
    #[cfg_attr(feature = "_derive", clause(tag = "auto-generated-by"))]
    AutoGeneratedBy(UnquotedString),
    Import(Import),
    Subsetdef(SubsetIdent, QuotedString),
    SynonymTypedef(SynonymTypeIdent, QuotedString, Option<SynonymScope>),
    #[cfg_attr(feature = "_derive", clause(tag = "default-namespace"))]
    DefaultNamespace(NamespaceIdent),
    #[cfg_attr(feature = "_derive", clause(tag = "namespace-id-rule"))]
    NamespaceIdRule(UnquotedString),
    Idspace(IdentPrefix, Url, Option<QuotedString>),
    #[cfg_attr(feature = "_derive", clause(tag = "treat-xrefs-as-equivalent"))]
    TreatXrefsAsEquivalent(IdentPrefix),
    #[cfg_attr(feature = "_derive", clause(tag = "treat-xrefs-as-genus-differentia"))]
    TreatXrefsAsGenusDifferentia(IdentPrefix, RelationIdent, ClassIdent),
    #[cfg_attr(feature = "_derive", clause(tag = "treat-xrefs-as-reverse-genus-differentia"))]
    TreatXrefsAsReverseGenusDifferentia(IdentPrefix, RelationIdent, ClassIdent),
    #[cfg_attr(feature = "_derive", clause(tag = "treat-xrefs-as-relationship"))]
    TreatXrefsAsRelationship(IdentPrefix, RelationIdent),
    #[cfg_attr(feature = "_derive", clause(tag = "treat-xrefs-as-is_a"))]
    TreatXrefsAsIsA(IdentPrefix),
    #[cfg_attr(feature = "_derive", clause(tag = "treat-xrefs-as-has-subclass"))]
    TreatXrefsAsHasSubclass(IdentPrefix),
    // FIXME(@althonos): Add support for hidden comment and qualifiers.
    PropertyValue(PropertyValue),
    Remark(UnquotedString),
    Ontology(UnquotedString),
    #[cfg_attr(feature = "_derive", clause(tag = "owl-axioms"))]
    OwlAxioms(UnquotedString),
    #[cfg_attr(feature = "_derive", clause(tag = 0))]
    Unreserved(UnquotedString, UnquotedString),
}

// #[cfg(feature = "ext")]
// impl crate::ext::CardinalityBound for crate::ast::HeaderClause {
//     fn cardinality(&self) -> crate::ext::Cardinality {
//         use self::HeaderClause::*;
//         use crate::ext::Cardinality::*;
//         match self {
//             FormatVersion(_) => ZeroOrOne,
//             DataVersion(_) => ZeroOrOne,
//             Date(_) => ZeroOrOne,
//             SavedBy(_) => ZeroOrOne,
//             AutoGeneratedBy(_) => ZeroOrOne,
//             Import(_) => Any,
//             Subsetdef(_, _) => Any,
//             SynonymTypedef(_, _, _) => Any,
//             DefaultNamespace(_) => ZeroOrOne,
//             NamespaceIdRule(_) => ZeroOrOne,
//             Idspace(_, _, _) => Any,
//             TreatXrefsAsEquivalent(_) => Any,
//             TreatXrefsAsGenusDifferentia(_, _, _) => Any,
//             TreatXrefsAsReverseGenusDifferentia(_, _, _) => Any,
//             TreatXrefsAsRelationship(_, _) => Any,
//             TreatXrefsAsIsA(_) => Any,
//             TreatXrefsAsHasSubclass(_) => Any,
//             // FIXME(@althonos): Add support for hidden comment and qualifiers.
//             PropertyValue(_) => Any,
//             Remark(_) => Any,
//             Ontology(_) => ZeroOrOne,
//             OwlAxioms(_) => Any,
//             Unreserved(_, _) => Any,
//         }
//     }
// }

impl<'i> FromPair<'i> for HeaderClause {
    const RULE: Rule = Rule::HeaderClause;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self, SyntaxError> {
        let mut inner = pair.into_inner();
        let tag = inner.next().unwrap();
        match tag.as_rule() {
            Rule::FormatVersionTag => {
                let version = UnquotedString::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::FormatVersion(version))
            }
            Rule::DataVersionTag => {
                let version = UnquotedString::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::DataVersion(version))
            }
            Rule::DateTag => {
                let date = NaiveDateTime::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::Date(date))
            }
            Rule::SavedByTag => {
                let person = UnquotedString::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::SavedBy(person))
            }
            Rule::AutoGeneratedByTag => {
                let soft = UnquotedString::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::AutoGeneratedBy(soft))
            }
            Rule::ImportTag => {
                let import = Import::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::Import(import))
            }
            Rule::SubsetdefTag => {
                let subset = SubsetIdent::from_pair_unchecked(inner.next().unwrap())?;
                let desc = QuotedString::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::Subsetdef(subset, desc))
            }
            Rule::SynonymTypedefTag => {
                let id = SynonymTypeIdent::from_pair_unchecked(inner.next().unwrap())?;
                let desc = QuotedString::from_pair_unchecked(inner.next().unwrap())?;
                let scope = match inner.next() {
                    Some(pair) => Some(SynonymScope::from_pair_unchecked(pair)?),
                    None => None,
                };
                Ok(HeaderClause::SynonymTypedef(id, desc, scope))
            }
            Rule::DefaultNamespaceTag => {
                let id = NamespaceIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::DefaultNamespace(id))
            }
            Rule::NamespaceIdRuleTag => {
                let value = UnquotedString::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::NamespaceIdRule(value))
            }
            Rule::IdspaceTag => {
                let prefix = IdentPrefix::from_pair_unchecked(inner.next().unwrap())?;
                let url = Url::from_pair_unchecked(inner.next().unwrap())?;
                let desc = match inner.next() {
                    Some(pair) => Some(QuotedString::from_pair_unchecked(pair)?),
                    None => None,
                };
                Ok(HeaderClause::Idspace(prefix, url, desc))
            }
            Rule::TreatXrefsAsEquivalentTag => {
                let prefix = IdentPrefix::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::TreatXrefsAsEquivalent(prefix))
            }
            Rule::TreatXrefsAsGenusDifferentiaTag => {
                let prefix = IdentPrefix::from_pair_unchecked(inner.next().unwrap())?;
                let rel = RelationIdent::from_pair_unchecked(inner.next().unwrap())?;
                let cls = ClassIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::TreatXrefsAsGenusDifferentia(prefix, rel, cls))
            }
            Rule::TreatXrefsAsReverseGenusDifferentiaTag => {
                let prefix = IdentPrefix::from_pair_unchecked(inner.next().unwrap())?;
                let rel = RelationIdent::from_pair_unchecked(inner.next().unwrap())?;
                let cls = ClassIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::TreatXrefsAsReverseGenusDifferentia(
                    prefix, rel, cls,
                ))
            }
            Rule::TreatXrefsAsRelationshipTag => {
                let prefix = IdentPrefix::from_pair_unchecked(inner.next().unwrap())?;
                let rel = RelationIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::TreatXrefsAsRelationship(prefix, rel))
            }
            Rule::TreatXrefsAsIsATag => {
                let prefix = IdentPrefix::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::TreatXrefsAsIsA(prefix))
            }
            Rule::TreatXrefsAsHasSubclassTag => {
                let prefix = IdentPrefix::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::TreatXrefsAsHasSubclass(prefix))
            }
            Rule::PropertyValueTag => {
                let pv = PropertyValue::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::PropertyValue(pv))
            }
            Rule::RemarkTag => {
                let remark = UnquotedString::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::Remark(remark))
            }
            Rule::OntologyTag => {
                let ont = UnquotedString::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::Ontology(ont))
            }
            Rule::OwlAxiomsTag => {
                let axioms = UnquotedString::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::OwlAxioms(axioms))
            }
            Rule::Unreserved => {
                let tag = UnquotedString::from_str(tag.as_str())?;
                let value = UnquotedString::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::Unreserved(tag, value))
            }
            _ => unreachable!(),
        }
    }
}
impl_fromstr!(HeaderClause);

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn tag() {
        let clause = HeaderClause::FormatVersion(UnquotedString::from("1.2"));
        assert_eq!(clause.tag(), "format-version");

        let clause = HeaderClause::Unreserved(UnquotedString::from("something"), UnquotedString::new(String::new()));
        assert_eq!(clause.tag(), "something");
    }

    #[test]
    fn from_str() {
        let actual = HeaderClause::from_str("format-version: 1.2").unwrap();
        let expected = HeaderClause::FormatVersion(UnquotedString::new(String::from("1.2")));
        assert_eq!(actual, expected);

        let actual = HeaderClause::from_str("subsetdef: GO_SLIM \"GO Slim\"").unwrap();
        let expected = HeaderClause::Subsetdef(
            SubsetIdent::from(UnprefixedIdent::new("GO_SLIM")),
            QuotedString::new("GO Slim"),
        );
        assert_eq!(actual, expected);

        let actual = HeaderClause::from_str("date: 17:03:2019 20:16").unwrap();
        let expected = HeaderClause::Date(NaiveDateTime::new(17, 3, 2019, 20, 16));
        assert_eq!(actual, expected);

        let actual =
            HeaderClause::from_str("namespace-id-rule: * XAO:$sequence(7,5000,9999999)$").unwrap();
        let expected =
            HeaderClause::NamespaceIdRule(UnquotedString::new("* XAO:$sequence(7,5000,9999999)$"));
        assert_eq!(actual, expected);

        let actual = HeaderClause::from_str("treat-xrefs-as-relationship: TEST rel").unwrap();
        let expected = HeaderClause::TreatXrefsAsRelationship(
            IdentPrefix::new("TEST"),
            RelationIdent::from(UnprefixedIdent::new("rel")),
        );
        assert_eq!(actual, expected);

        let actual = HeaderClause::from_str("tag: value").unwrap();
        let expected =
            HeaderClause::Unreserved(UnquotedString::new("tag"), UnquotedString::new("value"));
        assert_eq!(actual, expected);
    }

    #[test]
    fn partial_cmp() {
        macro_rules! assert_lt {
            ($l:ident, $r:ident) => {
                assert!($l < $r, "'{}' < '{}' is not true!", $l, $r)
            };
        }

        let fv1 = HeaderClause::FormatVersion(UnquotedString::new("1.4"));
        let fv2 = HeaderClause::FormatVersion(UnquotedString::new("2"));
        assert_lt!(fv1, fv2);

        let dv1 = HeaderClause::DataVersion(UnquotedString::new("1.4"));
        let dv2 = HeaderClause::DataVersion(UnquotedString::new("2"));
        assert_lt!(dv1, dv2);

        assert_lt!(fv1, dv1);
        assert_lt!(fv1, dv2);
        assert_lt!(fv2, dv1);
        assert_lt!(fv2, dv2);
    }
}
