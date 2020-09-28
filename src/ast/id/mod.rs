//! Identifiers used in OBO documents.
//!
//! `Ident` refers to an *owned* identifier, while `Id` refers to its *borrowed*
//! counterpart.

mod ident;
mod local;
mod prefix;
mod prefixed;
mod subclasses;
mod unprefixed;
mod url;

pub use self::ident::Ident;
pub use self::local::IdentLocal;
pub use self::prefix::IdentPrefix;
pub use self::prefixed::PrefixedIdent;
pub use self::subclasses::ClassIdent;
pub use self::subclasses::InstanceIdent;
pub use self::subclasses::NamespaceIdent;
pub use self::subclasses::RelationIdent;
pub use self::subclasses::SubsetIdent;
pub use self::subclasses::SynonymTypeIdent;
pub use self::unprefixed::UnprefixedIdent;
pub use self::url::Url;
