use fastobo_derive_internal::FromStr;
use fastobo_derive_internal::OboClause;

use crate::ast::*;
use crate::error::SyntaxError;
use crate::parser::FromPair;
use crate::semantics::OboClause;
use crate::syntax::Rule;

/// A clause appearing in a typedef frame.
#[derive(Clone, Debug, Eq, Hash, FromStr, Ord, OboClause, PartialEq, PartialOrd)]
pub enum TypedefClause {
    #[clause(cardinality = "ZeroOrOne")]
    IsAnonymous(bool),
    #[clause(cardinality = "ZeroOrOne")]
    Name(Box<UnquotedString>),
    #[clause(cardinality = "One")]
    Namespace(Box<NamespaceIdent>),
    AltId(Box<Ident>),
    #[clause(cardinality = "ZeroOrOne")]
    Def(Box<Definition>),
    #[clause(cardinality = "ZeroOrOne")]
    Comment(Box<UnquotedString>),
    Subset(Box<SubsetIdent>),
    Synonym(Box<Synonym>),
    Xref(Box<Xref>),
    PropertyValue(Box<PropertyValue>),
    #[clause(cardinality = "ZeroOrOne")]
    Domain(Box<ClassIdent>), // QUESTION(@althonos): Should be ID ?
    #[clause(cardinality = "ZeroOrOne")]
    Range(Box<ClassIdent>), // QUESTION(@althonos): same.
    #[clause(cardinality = "ZeroOrOne")]
    Builtin(bool),
    HoldsOverChain(Box<RelationIdent>, Box<RelationIdent>),
    #[clause(cardinality = "ZeroOrOne")]
    IsAntiSymmetric(bool),
    #[clause(cardinality = "ZeroOrOne")]
    IsCyclic(bool),
    #[clause(cardinality = "ZeroOrOne")]
    IsReflexive(bool),
    #[clause(cardinality = "ZeroOrOne")]
    IsSymmetric(bool),
    #[clause(cardinality = "ZeroOrOne")]
    IsAsymmetric(bool),
    #[clause(cardinality = "ZeroOrOne")]
    IsTransitive(bool),
    #[clause(cardinality = "ZeroOrOne")]
    IsFunctional(bool),
    #[clause(cardinality = "ZeroOrOne")]
    IsInverseFunctional(bool),
    IsA(Box<RelationIdent>),
    #[clause(cardinality = "NotOne")]
    IntersectionOf(Box<RelationIdent>),
    #[clause(cardinality = "NotOne")]
    UnionOf(Box<RelationIdent>),
    EquivalentTo(Box<RelationIdent>),
    DisjointFrom(Box<RelationIdent>),
    #[clause(cardinality = "ZeroOrOne")]
    InverseOf(Box<RelationIdent>),
    TransitiveOver(Box<RelationIdent>),
    EquivalentToChain(Box<RelationIdent>, Box<RelationIdent>),
    DisjointOver(Box<RelationIdent>),
    Relationship(Box<RelationIdent>, Box<RelationIdent>),
    #[clause(cardinality = "ZeroOrOne")]
    IsObsolete(bool),
    ReplacedBy(Box<RelationIdent>),
    Consider(Box<Ident>),
    #[clause(cardinality = "ZeroOrOne")]
    CreatedBy(Box<UnquotedString>),
    #[clause(cardinality = "ZeroOrOne")]
    CreationDate(Box<IsoDateTime>),
    ExpandAssertionTo(Box<QuotedString>, Box<XrefList>),
    ExpandExpressionTo(Box<QuotedString>, Box<XrefList>),
    #[clause(cardinality = "ZeroOrOne")]
    IsMetadataTag(bool),
    #[clause(cardinality = "ZeroOrOne")]
    IsClassLevel(bool),
}

impl From<Definition> for TypedefClause {
    fn from(d: Definition) -> Self {
        TypedefClause::Def(Box::new(d))
    }
}

impl<'i> FromPair<'i> for Line<TypedefClause> {
    const RULE: Rule = Rule::TypedefClauseLine;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self, SyntaxError> {
        let mut inner = pair.into_inner();
        let clause = TypedefClause::from_pair_unchecked(inner.next().unwrap())?;
        let eol = inner.next().unwrap();
        Ok(Eol::from_pair_unchecked(eol)?.and_inner(clause))
    }
}

impl<'i> FromPair<'i> for TypedefClause {
    const RULE: Rule = Rule::TypedefClause;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self, SyntaxError> {
        let mut inner = pair.into_inner();
        match inner.next().unwrap().as_rule() {
            Rule::IsAnonymousTag => {
                let b = bool::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::IsAnonymous(b))
            }
            Rule::NameTag => {
                let n = UnquotedString::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::Name(Box::new(n)))
            }
            Rule::NamespaceTag => {
                let ns = NamespaceIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::Namespace(Box::new(ns)))
            }
            Rule::AltIdTag => {
                let id = Ident::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::AltId(Box::new(id)))
            }
            Rule::DefTag => {
                let def = Definition::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::Def(Box::new(def)))
            }
            Rule::CommentTag => {
                let comment = UnquotedString::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::Comment(Box::new(comment)))
            }
            Rule::SubsetTag => {
                let id = SubsetIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::Subset(Box::new(id)))
            }
            Rule::SynonymTag => {
                let syn = Synonym::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::Synonym(Box::new(syn)))
            }
            Rule::XrefTag => {
                let xref = Xref::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::Xref(Box::new(xref)))
            }
            Rule::PropertyValueTag => {
                let pv = PropertyValue::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::PropertyValue(Box::new(pv)))
            }
            Rule::DomainTag => {
                let id = ClassIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::Domain(Box::new(id)))
            }
            Rule::RangeTag => {
                let id = ClassIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::Range(Box::new(id)))
            }
            Rule::BuiltinTag => {
                let b = bool::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::Builtin(b))
            }
            Rule::HoldsOverChainTag => {
                let r1 = RelationIdent::from_pair_unchecked(inner.next().unwrap())?;
                let r2 = RelationIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::HoldsOverChain(Box::new(r1), Box::new(r2)))
            }
            Rule::IsAntiSymmetricTag => {
                let b = bool::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::IsAntiSymmetric(b))
            }
            Rule::IsCyclicTag => {
                let b = bool::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::IsCyclic(b))
            }
            Rule::IsReflexiveTag => {
                let b = bool::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::IsReflexive(b))
            }
            Rule::IsSymmetricTag => {
                let b = bool::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::IsSymmetric(b))
            }
            Rule::IsAsymmetricTag => {
                let b = bool::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::IsAsymmetric(b))
            }
            Rule::IsTransitiveTag => {
                let b = bool::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::IsTransitive(b))
            }
            Rule::IsFunctionalTag => {
                let b = bool::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::IsFunctional(b))
            }
            Rule::IsInverseFunctionalTag => {
                let b = bool::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::IsInverseFunctional(b))
            }
            Rule::IsATag => {
                let id = RelationIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::IsA(Box::new(id)))
            }
            Rule::IntersectionOfTag => {
                let id = RelationIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::IntersectionOf(Box::new(id)))
            }
            Rule::UnionOfTag => {
                let id = RelationIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::UnionOf(Box::new(id)))
            }
            Rule::EquivalentToTag => {
                let id = RelationIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::EquivalentTo(Box::new(id)))
            }
            Rule::DisjointFromTag => {
                let id = RelationIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::DisjointFrom(Box::new(id)))
            }
            Rule::InverseOfTag => {
                let id = RelationIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::InverseOf(Box::new(id)))
            }
            Rule::TransitiveOverTag => {
                let id = RelationIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::TransitiveOver(Box::new(id)))
            }
            Rule::EquivalentToChainTag => {
                let r1 = RelationIdent::from_pair_unchecked(inner.next().unwrap())?;
                let r2 = RelationIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::EquivalentToChain(Box::new(r1), Box::new(r2)))
            }
            Rule::DisjointOverTag => {
                let id = RelationIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::DisjointOver(Box::new(id)))
            }
            Rule::RelationshipTag => {
                let r1 = RelationIdent::from_pair_unchecked(inner.next().unwrap())?;
                let r2 = RelationIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::Relationship(Box::new(r1), Box::new(r2)))
            }
            Rule::IsObsoleteTag => {
                let b = bool::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::IsObsolete(b))
            }
            Rule::ReplacedByTag => {
                let id = RelationIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::ReplacedBy(Box::new(id)))
            }
            Rule::ConsiderTag => {
                let id = Ident::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::Consider(Box::new(id)))
            }
            Rule::CreatedByTag => {
                let person = UnquotedString::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::CreatedBy(Box::new(person)))
            }
            Rule::CreationDateTag => {
                let date = IsoDateTime::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::CreationDate(Box::new(date)))
            }
            Rule::ExpandAssertionToTag => {
                let desc = QuotedString::from_pair_unchecked(inner.next().unwrap())?;
                let xrefs = XrefList::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::ExpandAssertionTo(
                    Box::new(desc),
                    Box::new(xrefs),
                ))
            }
            Rule::ExpandExpressionToTag => {
                let desc = QuotedString::from_pair_unchecked(inner.next().unwrap())?;
                let xrefs = XrefList::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::ExpandExpressionTo(
                    Box::new(desc),
                    Box::new(xrefs),
                ))
            }
            Rule::IsMetadataTagTag => {
                let b = bool::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::IsMetadataTag(b))
            }
            Rule::IsClassLevelTag => {
                let b = bool::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::IsClassLevel(b))
            }
            _ => unreachable!(),
        }
    }
}
