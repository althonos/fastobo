use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::str::FromStr;

use pest::iterators::Pair;
use url::Url;

use crate::ast::*;
use crate::error::Error;
use crate::error::Result;
use crate::parser::FromPair;
use crate::parser::Rule;

/// A clause appearing in a header frame.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum HeaderClause {
    FormatVersion(UnquotedString),
    DataVersion(UnquotedString),
    Date(NaiveDateTime),
    SavedBy(UnquotedString),
    AutoGeneratedBy(UnquotedString),
    Import(Import),
    Subsetdef(SubsetId, QuotedString),
    SynonymTypedef(SynonymTypeId, QuotedString, Option<SynonymScope>),
    DefaultNamespace(NamespaceId),
    Idspace(IdPrefix, Url, Option<QuotedString>),
    TreatXrefsAsEquivalent(IdPrefix),
    TreatXrefsAsGenusDifferentia(IdPrefix, RelationId, ClassId),
    TreatXrefsAsReverseGenusDifferentia(IdPrefix, RelationId, ClassId),
    TreatXrefsAsRelationship(IdPrefix, RelationId),
    TreatXrefsAsIsA(IdPrefix),
    TreatXrefsAsHasSubclass(IdPrefix),
    // FIXME(@althonos): Add support for hidden comment and qualifiers.
    PropertyValue(PropertyValue),
    Remark(UnquotedString),
    Ontology(UnquotedString),
    OwlAxioms(UnquotedString),
    Unreserved(UnquotedString, UnquotedString),
}

impl Display for HeaderClause {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::HeaderClause::*;
        match self {
            FormatVersion(version) => f.write_str("format-version: ").and(version.fmt(f)),
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
            TreatXrefsAsIsA(prefix) => f.write_str("treat-xrefs-as-is_a: ").and(prefix.fmt(f)),
            TreatXrefsAsHasSubclass(prefix) => f
                .write_str("treat-xrefs-as-has-subclass")
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
                let subset = SubsetId::from_pair_unchecked(inner.next().unwrap())?;
                let desc = QuotedString::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::Subsetdef(subset, desc))
            }
            Rule::SynonymTypedefTag => {
                let id = SynonymTypeId::from_pair_unchecked(inner.next().unwrap())?;
                let desc = QuotedString::from_pair_unchecked(inner.next().unwrap())?;
                let scope = match inner.next() {
                    Some(pair) => Some(SynonymScope::from_pair_unchecked(pair)?),
                    None => None,
                };
                Ok(HeaderClause::SynonymTypedef(id, desc, scope))
            }
            Rule::DefaultNamespaceTag => {
                let id = NamespaceId::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::DefaultNamespace(id))
            }
            Rule::IdspaceTag => {
                let prefix = IdPrefix::from_pair_unchecked(inner.next().unwrap())?;
                let url = Url::from_pair_unchecked(inner.next().unwrap())?;
                let desc = match inner.next() {
                    Some(pair) => Some(QuotedString::from_pair_unchecked(pair)?),
                    None => None,
                };
                Ok(HeaderClause::Idspace(prefix, url, desc))
            }
            Rule::TreatXrefsAsEquivalentTag => {
                let prefix = IdPrefix::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::TreatXrefsAsEquivalent(prefix))
            }
            Rule::TreatXrefsAsGenusDifferentiaTag => {
                let prefix = IdPrefix::from_pair_unchecked(inner.next().unwrap())?;
                let rel = RelationId::from_pair_unchecked(inner.next().unwrap())?;
                let cls = ClassId::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::TreatXrefsAsGenusDifferentia(prefix, rel, cls))
            }
            Rule::TreatXrefsAsReverseGenusDifferentiaTag => {
                let prefix = IdPrefix::from_pair_unchecked(inner.next().unwrap())?;
                let rel = RelationId::from_pair_unchecked(inner.next().unwrap())?;
                let cls = ClassId::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::TreatXrefsAsReverseGenusDifferentia(
                    prefix, rel, cls,
                ))
            }
            Rule::TreatXrefsAsRelationshipTag => {
                let prefix = IdPrefix::from_pair_unchecked(inner.next().unwrap())?;
                let rel = RelationId::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::TreatXrefsAsRelationship(prefix, rel))
            }
            Rule::TreatXrefsAsIsATag => {
                let prefix = IdPrefix::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::TreatXrefsAsIsA(prefix))
            }
            Rule::TreatXrefsAsHasSubclassTag => {
                let prefix = IdPrefix::from_pair_unchecked(inner.next().unwrap())?;
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
    use crate::ast::UnprefixedId;
    use crate::ast::UnquotedString;

    #[test]
    fn from_str() {
        let actual = HeaderClause::from_str("format-version: 1.2").unwrap();
        let expected = HeaderClause::FormatVersion(UnquotedString::new("1.2"));
        assert_eq!(actual, expected);

        let actual = HeaderClause::from_str("subsetdef: GO_SLIM \"GO Slim\"").unwrap();
        let expected = HeaderClause::Subsetdef(
            SubsetId::from(Id::from(UnprefixedId::new("GO_SLIM"))),
            QuotedString::new("GO Slim"),
        );
        assert_eq!(actual, expected);

        let actual = HeaderClause::from_str("date: 17:03:2019 20:16").unwrap();
        let expected = HeaderClause::Date(NaiveDateTime::new(17, 3, 2019, 20, 16));
        assert_eq!(actual, expected);

        let actual =
            HeaderClause::from_str("namespace-id-rule: * XAO:$sequence(7,5000,9999999)$").unwrap();
        let expected = HeaderClause::Unreserved(
            UnquotedString::new("namespace-id-rule"),
            UnquotedString::new("* XAO:$sequence(7,5000,9999999)$"),
        );
        assert_eq!(actual, expected);
    }

}
