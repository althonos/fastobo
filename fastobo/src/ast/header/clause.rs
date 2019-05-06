use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::str::FromStr;

use pest::iterators::Pair;
use url::Url;

use crate::ast::*;
use crate::share::Share;
use crate::share::Cow;
use crate::share::Redeem;
use crate::error::Error;
use crate::error::Result;
use crate::parser::FromPair;
use crate::parser::Rule;


/// A clause appearing in a header frame.
///
/// Header clauses are used to add metadata to OBO documents. They are all
/// optional, but every document should *at least* contain a `FormatVersion`
/// clause, to help with interoperability and to make sure the semantics of
/// the right OBO specification are in use.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum HeaderClause {
    FormatVersion(UnquotedString),
    DataVersion(UnquotedString),
    Date(NaiveDateTime),
    SavedBy(UnquotedString),
    AutoGeneratedBy(UnquotedString),
    Import(Import),
    Subsetdef(SubsetIdent, QuotedString),
    SynonymTypedef(SynonymTypeIdent, QuotedString, Option<SynonymScope>),
    DefaultNamespace(NamespaceIdent),
    NamespaceIdRule(UnquotedString),
    Idspace(IdentPrefix, Url, Option<QuotedString>),
    TreatXrefsAsEquivalent(IdentPrefix),
    TreatXrefsAsGenusDifferentia(IdentPrefix, RelationIdent, ClassIdent),
    TreatXrefsAsReverseGenusDifferentia(IdentPrefix, RelationIdent, ClassIdent),
    TreatXrefsAsRelationship(IdentPrefix, RelationIdent),
    TreatXrefsAsIsA(IdentPrefix),
    TreatXrefsAsHasSubclass(IdentPrefix),
    // FIXME(@althonos): Add support for hidden comment and qualifiers.
    PropertyValue(PropertyValue),
    Remark(UnquotedString),
    Ontology(UnquotedString),
    OwlAxioms(UnquotedString),
    Unreserved(UnquotedString, UnquotedString),
}

// impl<'a> Borrow<'a, HeaderClauseRef<'a>> for HeaderClause {
//     fn borrow(&'a self) -> HeaderClauseRef<'a> {
//         match self {
//             HeaderClause::FormatVersion(ref v) =>
//                 HeaderClauseRef::FormatVersion(Cow::Borrowed(v.borrow())),
//             HeaderClause::TreatXrefsAsIsA(ref pref) =>
//                 HeaderClauseRef::TreatXrefsAsIsA(Cow::Borrowed(pref.borrow())),
//             _ => unimplemented!(),
//         }
//     }
// }

impl Display for HeaderClause {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::HeaderClause::*;
        match self {
            FormatVersion(ref version) =>
                f.write_str("format-version: ").and(version.fmt(f)),
            DataVersion(version) => f.write_str("data-version: ").and(version.fmt(f)),
            Date(date) => f.write_str("date: ").and(date.fmt(f)),
            SavedBy(person) => f.write_str("saved-by: ").and(person.fmt(f)),
            AutoGeneratedBy(thing) => f.write_str("auto-generated-by: ").and(thing.fmt(f)),
            Import(import) => f.write_str("import: ").and(import.fmt(f)),
            Subsetdef(subset, desc) => f
                .write_str("subsetdef: ")
                .and(subset.fmt(f))
                .and(f.write_char(' '))
                .and(desc.fmt(f)),
            SynonymTypedef(syntype, desc, optscope) => {
                f.write_str("synonymtypedef: ")
                    .and(syntype.fmt(f))
                    .and(f.write_char(' '))
                    .and(desc.fmt(f))?;
                match optscope {
                    Some(scope) => f.write_char(' ').and(scope.fmt(f)),
                    None => Ok(()),
                }
            }
            DefaultNamespace(ns) => f.write_str("default-namespace: ").and(ns.fmt(f)),
            NamespaceIdRule(r) => f.write_str("namespace-id-rule: ").and(r.fmt(f)),
            Idspace(prefix, url, optdesc) => {
                f.write_str("idspace: ")
                    .and(prefix.fmt(f))
                    .and(f.write_char(' '))
                    .and(url.fmt(f))?;
                match optdesc {
                    Some(desc) => f.write_char(' ').and(desc.fmt(f)),
                    None => Ok(()),
                }
            }
            TreatXrefsAsEquivalent(prefix) => f
                .write_str("treat-xrefs-as-equivalent: ")
                .and(prefix.fmt(f)),
            TreatXrefsAsGenusDifferentia(prefix, rel, cls) => f
                .write_str("treat-xrefs-as-genus-differentia: ")
                .and(prefix.fmt(f))
                .and(f.write_char(' '))
                .and(rel.fmt(f))
                .and(f.write_char(' '))
                .and(cls.fmt(f)),
            TreatXrefsAsReverseGenusDifferentia(prefix, rel, cls) => f
                .write_str("treat-xrefs-as-reverse-genus-differentia: ")
                .and(prefix.fmt(f))
                .and(f.write_char(' '))
                .and(rel.fmt(f))
                .and(f.write_char(' '))
                .and(cls.fmt(f)),
            TreatXrefsAsRelationship(prefix, rel) => f
                .write_str("treat-xrefs-as-relationship: ")
                .and(prefix.fmt(f))
                .and(f.write_char(' '))
                .and(rel.fmt(f)),
            TreatXrefsAsIsA(prefix) => f
                .write_str("treat-xrefs-as-is_a: ")
                .and(prefix.fmt(f)),
            TreatXrefsAsHasSubclass(prefix) => f
                .write_str("treat-xrefs-as-has-subclass: ")
                .and(prefix.fmt(f)),
            PropertyValue(pv) => f.write_str("property_value: ").and(pv.fmt(f)),
            Remark(remark) => f.write_str("remark: ").and(remark.fmt(f)),
            Ontology(ont) => f.write_str("ontology: ").and(ont.fmt(f)),
            OwlAxioms(axioms) => f.write_str("owl-axioms: ").and(axioms.fmt(f)),
            Unreserved(key, value) => key.fmt(f).and(f.write_str(": ")).and(value.fmt(f)),
        }
    }
}

impl<'i> FromPair<'i> for HeaderClause {
    const RULE: Rule = Rule::HeaderClause;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
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

// WIP(@athonos)
//
// #[derive(Clone, Debug)]
// pub enum HeaderClauseRef<'a> {
//     FormatVersion(Cow<'a, &'a UnquotedStr>),
//     // DataVersion(Cow<'a, &'a UnquotedStr>),
//     // Date(Cow<'a, NaiveDateTime>),
//     // SavedBy(Cow<'a, &'a UnquotedStr>),
//     // AutoGeneratedBy(Cow<'a, &'a UnquotedStr>),
//     // Import(Cow<'a, &'a Import>),
//     // SubsetDef(SubsetId, Cow<'a, &'a UnquotedStr>),
//     // SynonymTypedef(SynonymTypeId, Cow<'a, &'a UnquotedStr>),
//
//
//     TreatXrefsAsIsA(Cow<'a, IdPrefix<'a>>),
//
// }
//
// impl<'a> Display for HeaderClauseRef<'a> {
//     fn fmt(&self, f: &mut Formatter) -> FmtResult {
//         use self::HeaderClauseRef::*;
//         match self {
//             FormatVersion(version) =>
//                 f.write_str("format-version: ")
//                     .and(version.fmt(f)),
//             TreatXrefsAsIsA(prefix) =>
//                 f.write_str("treat-xrefs-as-is_a: ")
//                     .and(prefix.fmt(f)),
//         }
//     }
// }
//
// impl<'a> ToOwned<'a> for HeaderClauseRef<'a> {
//     type Owned = HeaderClause;
//     fn to_owned(&'a self) -> Self::Owned {
//         match self {
//             HeaderClauseRef::FormatVersion(ref v) =>
//                 HeaderClause::FormatVersion(ToOwned::to_owned(v)),
//             HeaderClauseRef::TreatXrefsAsIsA(ref v) =>
//                 HeaderClause::TreatXrefsAsIsA(ToOwned::to_owned(v))
//         }
//     }
// }

#[cfg(test)]
mod tests {

    use pretty_assertions::assert_eq;
    use super::*;

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
        let expected = HeaderClause::NamespaceIdRule(
            UnquotedString::new("* XAO:$sequence(7,5000,9999999)$"),
        );
        assert_eq!(actual, expected);

        let actual = HeaderClause::from_str("treat-xrefs-as-relationship: TEST rel").unwrap();
        let expected = HeaderClause::TreatXrefsAsRelationship(
            IdentPrefix::new("TEST"),
            RelationIdent::from(UnprefixedIdent::new("rel"))
        );
        assert_eq!(actual, expected);

        let actual =
            HeaderClause::from_str("tag: value").unwrap();
        let expected = HeaderClause::Unreserved(
            UnquotedString::new("tag"),
            UnquotedString::new("value"),
        );
        assert_eq!(actual, expected);
    }
}
