use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::str::FromStr;
use std::string::ToString;

use fastobo::ast;
use fastobo::ast as obo;
use fastobo::ast::UnquotedString;
use fastobo::ast::QuotedString;
use fastobo::borrow::Cow;
use fastobo::borrow::Borrow;
use fastobo::borrow::ToOwned;

use pyo3::prelude::*;
use pyo3::PyTypeInfo;
use pyo3::PyNativeType;
use pyo3::types::PyTimeAccess;
use pyo3::types::PyDateAccess;
use pyo3::types::PyAny;
use pyo3::types::PyList;
use pyo3::types::PyDateTime;
use pyo3::types::PyString;
use pyo3::exceptions::RuntimeError;
use pyo3::exceptions::IndexError;
use pyo3::exceptions::TypeError;
use pyo3::exceptions::ValueError;
use pyo3::PySequenceProtocol;
use pyo3::PyGCProtocol;
use pyo3::PyObjectProtocol;
use pyo3::gc::PyTraverseError;
use pyo3::class::gc::PyVisit;
use pyo3::type_object::PyTypeCreate;

use crate::id::Url;
use crate::id::Ident;
use crate::id::IdentPrefix;
use crate::id::BaseIdent;
use crate::pv::PropertyValue;

// --- Conversion Wrapper ----------------------------------------------------

/// A thin wrapper for a reference to any possible `BaseHeaderClause` subclass.
#[derive(Debug, PyWrapper)]
#[wraps(BaseHeaderClause)]
pub enum HeaderClause {
    FormatVersion(Py<FormatVersionClause>),
    DataVersion(Py<DataVersionClause>),
    Date(Py<DateClause>),
    SavedBy(Py<SavedByClause>),
    AutoGeneratedBy(Py<AutoGeneratedByClause>),
    Import(Py<ImportClause>),
    Subsetdef(Py<SubsetdefClause>),
    SynonymTypedef(Py<SynonymTypedefClause>),
    DefaultNamespace(Py<DefaultNamespaceClause>),
    Idspace(Py<IdspaceClause>),
    TreatXrefsAsEquivalent(Py<TreatXrefsAsEquivalentClause>),
    TreatXrefsAsGenusDifferentia(Py<TreatXrefsAsGenusDifferentiaClause>),
    TreatXrefsAsReverseGenusDifferentia(Py<TreatXrefsAsReverseGenusDifferentiaClause>),
    TreatXrefsAsRelationship(Py<TreatXrefsAsRelationshipClause>),
    TreatXrefsAsIsA(Py<TreatXrefsAsIsAClause>),
    TreatXrefsAsHasSubclass(Py<TreatXrefsAsHasSubclassClause>),
    PropertyValue(Py<PropertyValueClause>),
    Remark(Py<RemarkClause>),
    Ontology(Py<OntologyClause>),
    OwlAxioms(Py<OwlAxiomsClause>),
    Unreserved(Py<UnreservedClause>),
}

impl FromPy<fastobo::ast::HeaderClause> for HeaderClause {
    fn from_py(clause: fastobo::ast::HeaderClause, py: Python) -> Self {
        use fastobo::ast::HeaderClause::*;
        match clause {
            FormatVersion(v) => HeaderClause::FormatVersion(
                Py::new(py, FormatVersionClause::new(v)).unwrap()
            ),
            DataVersion(v) => HeaderClause::DataVersion(
                Py::new(py, DataVersionClause::new(v)).unwrap()
            ),
            Date(dt) =>
                Py::new(py, DateClause::new(dt))
                .map(HeaderClause::Date)
                .unwrap(),
            SavedBy(name) => HeaderClause::SavedBy(
                Py::new(py, SavedByClause::new(name)).unwrap()
            ),
            AutoGeneratedBy(name) => HeaderClause::AutoGeneratedBy(
                Py::new(py, AutoGeneratedByClause::new(name)).unwrap()
            ),
            Import(i) => HeaderClause::Import(
                Py::new(py, ImportClause::new(i)).unwrap()
            ),
            Subsetdef(s, q) => HeaderClause::Subsetdef(
                Py::new(py, SubsetdefClause::new(s, q)).unwrap()
            ),
            SynonymTypedef(ty, desc, scope) => HeaderClause::SynonymTypedef(
                //FIXME
                Py::new(py, SynonymTypedefClause::with_scope(ty, desc, scope)).unwrap()
            ),
            DefaultNamespace(ns) => HeaderClause::DefaultNamespace(
                Py::new(py, DefaultNamespaceClause::new(ns)).unwrap()
            ),
            Idspace(prefix, url, desc) => HeaderClause::Idspace(
                Py::new(py, IdspaceClause::with_description(prefix, url, desc)).unwrap()
            ),
            TreatXrefsAsEquivalent(prefix) => HeaderClause::TreatXrefsAsEquivalent(
                Py::new(py, TreatXrefsAsEquivalentClause::new(prefix)).unwrap()
            ),
            TreatXrefsAsGenusDifferentia(p, r, c) => HeaderClause::TreatXrefsAsGenusDifferentia(
                Py::new(py, TreatXrefsAsGenusDifferentiaClause::new(p, r, c)).unwrap()
            ),
            TreatXrefsAsReverseGenusDifferentia(p, r, c) => HeaderClause::TreatXrefsAsReverseGenusDifferentia(
                Py::new(py, TreatXrefsAsReverseGenusDifferentiaClause::new(p, r, c)).unwrap()
            ),
            TreatXrefsAsRelationship(p, r) => HeaderClause::TreatXrefsAsRelationship(
                Py::new(py, TreatXrefsAsRelationshipClause::new(p, r)).unwrap()
            ),
            TreatXrefsAsIsA(p) => HeaderClause::TreatXrefsAsIsA(
                Py::new(py, TreatXrefsAsIsAClause::new(p)).unwrap()
            ),
            TreatXrefsAsHasSubclass(p) => HeaderClause::TreatXrefsAsHasSubclass(
                Py::new(py, TreatXrefsAsHasSubclassClause::new(p)).unwrap()
            ),
            PropertyValue(pv) => HeaderClause::PropertyValue(
                Py::new(py, PropertyValueClause::new(pv)).unwrap()
            ),
            Remark(r) => HeaderClause::Remark(
                Py::new(py, RemarkClause::new(r)).unwrap()
            ),
            Ontology(ont) => HeaderClause::Ontology(
                Py::new(py, OntologyClause::new(ont)).unwrap()
            ),
            OwlAxioms(ax) => HeaderClause::OwlAxioms(
                Py::new(py, OwlAxiomsClause::new(ax)).unwrap()
            ),
            Unreserved(tag, value) =>
                Py::new(py, UnreservedClause::new(tag, value))
                .map(HeaderClause::Unreserved)
                .unwrap(),

        }
    }
}

impl From<fastobo::ast::HeaderClause> for HeaderClause {
    fn from(clause: fastobo::ast::HeaderClause) -> Self {
        let gil = Python::acquire_gil();
        Self::from_py(clause, gil.python())
    }
}

// --- Base ------------------------------------------------------------------

#[pyclass(subclass)]
pub struct BaseHeaderClause {}

// --- FormatVersion ---------------------------------------------------------

#[pyclass(extends=BaseHeaderClause)]
#[derive(Debug, Clone)]
pub struct FormatVersionClause {
    version: obo::UnquotedString,
}

impl FormatVersionClause {
    pub fn new(version: obo::UnquotedString) -> Self {
        Self { version }
    }
}

impl Display for FormatVersionClause {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        obo::HeaderClause::from(self.clone()).fmt(f)
    }
}

impl From<FormatVersionClause> for obo::HeaderClause {
    fn from(clause: FormatVersionClause) -> obo::HeaderClause {
        <obo::HeaderClauseRef as ToOwned>::to_owned(&clause.to_ref())
    }
}

impl From<FormatVersionClause> for HeaderClause {
    fn from(clause: FormatVersionClause) -> HeaderClause {
        obo::HeaderClause::from(clause).into()
    }
}

impl FormatVersionClause {
    fn to_ref<'s>(&'s self) -> obo::HeaderClauseRef<'s> {
        let s: &'s str = self.version.as_ref();
        obo::HeaderClauseRef::FormatVersion(Cow::Borrowed(obo::UnquotedStr::new(s)))
    }
}

#[pymethods]
impl FormatVersionClause {
    #[new]
    fn __init__(obj: &PyRawObject, version: String) {
        obj.init(Self::new(fastobo::ast::UnquotedString::new(version)));
    }

    /// `str`: the OBO format version used in document.
    #[getter]
    fn get_version(&self) -> PyResult<&str> {
        Ok(self.version.as_str())
    }

    #[setter]
    fn set_version(&mut self, version: String) -> PyResult<()> {
        self.version = obo::UnquotedString::new(version);
        Ok(())
    }
}

#[pyproto]
impl PyObjectProtocol for FormatVersionClause {
    fn __repr__(&self) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let fmt = PyString::new(py, "FormatVersionClause({!r})").to_object(py);
        fmt.call_method1(py, "format", (self.version.as_str(),))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }
}

// --- DataVersion -----------------------------------------------------------

#[pyclass(extends=BaseHeaderClause)]
#[derive(Clone, Debug)]
pub struct DataVersionClause {
    version: UnquotedString
}

impl DataVersionClause {
    pub fn new(version: UnquotedString) -> Self {
        Self {version}
    }
}

impl From<DataVersionClause> for obo::HeaderClause {
    fn from(clause: DataVersionClause) -> obo::HeaderClause {
        obo::HeaderClause::DataVersion(clause.version)
    }
}

impl Display for DataVersionClause {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        obo::HeaderClause::from(self.clone())
            .fmt(f)
    }
}

#[pymethods]
impl DataVersionClause {
    #[new]
    fn __init__(obj: &PyRawObject, version: String) {
        obj.init(Self::new(UnquotedString::new(version)));
    }

    #[getter]
    fn get_version(&self) -> PyResult<&str> {
        Ok(self.version.as_str())
    }

    #[setter]
    fn set_version(&mut self, version: String) -> PyResult<()> {
        self.version = UnquotedString::new(version);
        Ok(())
    }
}



#[pyproto]
impl PyObjectProtocol for DataVersionClause {
    fn __repr__(&self) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let fmt = PyString::new(py, "DataVersionClause({!r})").to_object(py);
        fmt.call_method1(py, "format", (self.version.as_str(),))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }
}

// --- Date ------------------------------------------------------------------

#[pyclass(extends=BaseHeaderClause)]
#[derive(Clone, Debug)]
pub struct DateClause {
    date: obo::NaiveDateTime,
}

impl DateClause {
    pub fn new(date: obo::NaiveDateTime) -> Self {
        Self { date }
    }
}

impl From<DateClause> for obo::HeaderClause {
    fn from(clause: DateClause) -> obo::HeaderClause {
        obo::HeaderClause::Date(clause.date)
    }
}

impl Display for DateClause {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        obo::HeaderClause::from(self.clone()).fmt(f)
    }
}

#[pymethods]
impl DateClause {
    #[new]
    fn __init__(obj: &PyRawObject, date: &PyDateTime) {
        let dt = fastobo::ast::NaiveDateTime::new(
            date.get_day() as u8,
            date.get_month() as u8,
            date.get_year() as u16,
            date.get_hour() as u8,
            date.get_minute() as u8,
        );
        obj.init(Self::new(dt))
    }

    #[getter]
    fn get_date(&self) -> PyResult<Py<PyDateTime>> {
        PyDateTime::new(
            Python::acquire_gil().python(),
            self.date.year() as i32,
            self.date.month(),
            self.date.day(),
            self.date.hour(),
            self.date.minute(),
            0,
            0,
            None
        )
    }
}

#[pyproto]
impl PyObjectProtocol for DateClause {
    fn __repr__(&self) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let fmt = PyString::new(py, "DateClause({!r})").to_object(py);
        fmt.call_method1(py, "format", (self.get_date()?, ))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }
}

// --- SavedBy ---------------------------------------------------------------

#[pyclass(extends=BaseHeaderClause)]
#[derive(Clone, Debug)]
pub struct SavedByClause {
    name: UnquotedString
}

impl SavedByClause {
    pub fn new(name: UnquotedString) -> Self  {
        Self {name}
    }
}

impl From<SavedByClause> for obo::HeaderClause {
    fn from(clause: SavedByClause) -> obo::HeaderClause {
        obo::HeaderClause::SavedBy(clause.name)
    }
}

impl Display for SavedByClause {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        obo::HeaderClause::from(self.clone()).fmt(f)
    }
}

#[pymethods]
impl SavedByClause {
    #[new]
    fn __init__(obj: &PyRawObject, version: String) {
        obj.init(Self::new(UnquotedString::new(version)));
    }

    #[getter]
    fn get_name(&self) -> PyResult<&str> {
        Ok(self.name.as_str())
    }

    #[setter]
    fn set_name(&mut self, name: String) -> PyResult<()> {
        self.name = UnquotedString::new(name);
        Ok(())
    }
}

#[pyproto]
impl PyObjectProtocol for SavedByClause {
    fn __repr__(&self) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let fmt = PyString::new(py, "SavedByClause({!r})").to_object(py);
        fmt.call_method1(py, "format", (self.name.as_str(), ))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }
}

// --- AutoGeneratedBy -------------------------------------------------------

#[pyclass(extends=BaseHeaderClause)]
#[derive(Clone, Debug)]
pub struct AutoGeneratedByClause {
    name: UnquotedString
}

impl AutoGeneratedByClause {
    pub fn new(name: UnquotedString) -> Self {
        Self { name }
    }
}

impl From<AutoGeneratedByClause> for obo::HeaderClause {
    fn from(clause: AutoGeneratedByClause) -> obo::HeaderClause {
        obo::HeaderClause::AutoGeneratedBy(clause.name)
    }
}

impl Display for AutoGeneratedByClause {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        obo::HeaderClause::from(self.clone()).fmt(f)
    }
}

#[pymethods]
impl AutoGeneratedByClause {
    #[new]
    fn __init__(obj: &PyRawObject, version: String) {
        obj.init(Self::new(UnquotedString::new(version)));
    }

    #[getter]
    fn get_name(&self) -> PyResult<&str> {
        Ok(self.name.as_ref())
    }

    #[setter]
    fn set_name(&mut self, name: String) -> PyResult<()> {
        self.name = UnquotedString::new(name);
        Ok(())
    }
}

// --- Import ----------------------------------------------------------------

#[pyclass(extends=BaseHeaderClause)]
#[derive(Clone, Debug)]
pub struct ImportClause {
    reference: obo::Import, // should be `Import`
}

impl ImportClause {
    pub fn new(reference: obo::Import) -> Self {
        Self { reference }
    }
}

impl From<ImportClause> for obo::HeaderClause {
    fn from(clause: ImportClause) -> Self {
        obo::HeaderClause::Import(clause.reference)
    }
}

#[pymethods]
impl ImportClause {
    #[new]
    pub fn __init__(obj: &PyRawObject, reference: &str) -> PyResult<()> {
        // FIXME(@althonos): should not be implicit here ?
        if let Ok(url) = url::Url::from_str(reference) {
            Ok(obj.init(Self::new(obo::Import::Url(url))))
        } else if let Ok(id) = obo::Ident::from_str(reference) {
            Ok(obj.init(Self::new(obo::Import::Abbreviated(id))))
        } else {
            ValueError::into(format!("invalid import: {:?}", reference))
        }
    }
}

// --- Subsetdef -------------------------------------------------------------

#[pyclass(extends=BaseHeaderClause)]
#[derive(Clone, Debug)]
pub struct SubsetdefClause {
    subset: Ident,
    description: QuotedString,
}

impl SubsetdefClause {
    pub fn new<I>(subset: I, description: QuotedString) -> Self
    where
        I: Into<Ident>
    {
        Self {
            subset: subset.into(),
            description
        }
    }
}

impl From<SubsetdefClause> for obo::HeaderClause {
    fn from(clause: SubsetdefClause) -> Self {
        obo::HeaderClause::Subsetdef(clause.subset.into(), clause.description)
    }
}

impl Display for SubsetdefClause {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        obo::HeaderClause::from(self.clone()).fmt(f)
    }
}

#[pymethods]
impl SubsetdefClause {
    // FIXME
    // #[new]
    // fn __init__(obj: &PyRawObject, subset: &PyAny, description: String) -> PyResult<()> {
    //     let py = obj.py();
    //     let ident = if py.is_instance::<BaseIdent, PyAny>(subset)? {
    //         Ident::extract(subset)?
    //     } else if py.is_instance::<PyString, PyAny>(subset)? {
    //         let s: &PyString = FromPyObject::extract(subset)?;
    //         ast::Ident::from_str(&s.to_string()?)?
    //     } else {
    //         return TypeError::into("expected str or Ident for 'subset'");
    //     };
    //     Ok(obj.init(Self::new(ident, QuotedString::new(description))))
    // }
}

#[pyproto]
impl PyObjectProtocol for SubsetdefClause {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }
}

// --- SynonymTypedef --------------------------------------------------------

#[pyclass(extends=BaseHeaderClause)]
#[derive(Clone, Debug)]
pub struct SynonymTypedefClause {
    typedef: Ident,
    description: QuotedString,
    scope: Option<fastobo::ast::SynonymScope>,  // FIXME: Python type
}

impl SynonymTypedefClause {
    pub fn new<T, D>(typedef: T, description: D) -> Self
    where
        T: Into<Ident>,
        D: Into<QuotedString>,
    {
        Self { typedef: typedef.into(), description: description.into(), scope: None }
    }

    pub fn with_scope<T, D, S>(typedef: T, description: D, scope: S) -> Self
    where
        T: Into<Ident>,
        D: Into<QuotedString>,
        S: Into<Option<fastobo::ast::SynonymScope>>
    {
        Self {typedef: typedef.into(), description: description.into(), scope: scope.into() }
    }
}

impl From<SynonymTypedefClause> for obo::HeaderClause {
    fn from(clause: SynonymTypedefClause) -> Self {
        obo::HeaderClause::SynonymTypedef(
            clause.typedef.into(),
            clause.description,
            clause.scope,
        )
    }
}

#[pymethods]
impl SynonymTypedefClause {
    #[new]
    fn __init__(obj: &PyRawObject, typedef: Ident, description: String, scope: Option<String>) {

        let desc = fastobo::ast::QuotedString::new(description);
        let sc = scope.map(|s| fastobo::ast::SynonymScope::from_str(&s).unwrap()); // FIXME
        obj.init(Self::with_scope(typedef, desc, sc));
    }
}

// --- DefaultNamespace ------------------------------------------------------

#[pyclass(extends=BaseHeaderClause)]
#[derive(Clone, Debug)]
pub struct DefaultNamespaceClause {
    #[pyo3(get, set)]
    namespace: Ident,    // should be `NamespaceIdent`
}

impl DefaultNamespaceClause {
    pub fn new<I>(namespace: I) -> Self
    where
        I: Into<Ident>,
    {
        Self { namespace: namespace.into() }
    }
}

impl From<DefaultNamespaceClause> for obo::HeaderClause {
    fn from(clause: DefaultNamespaceClause) -> Self {
        obo::HeaderClause::DefaultNamespace(From::from(clause.namespace))
    }
}

#[pymethods]
impl DefaultNamespaceClause {
    #[new]
    fn __init__(obj: &PyRawObject, namespace: &PyAny) -> PyResult<()> {
        let py = obj.py();
        let ident = if py.is_instance::<BaseIdent, PyAny>(namespace)? {
            Ident::extract(namespace)?
        } else if py.is_instance::<PyString, PyAny>(namespace)? {
            let s: &PyString = FromPyObject::extract(namespace)?;
            let id = ast::Ident::from_str(&s.to_string()?).unwrap(); // FIXME
            Ident::from(id)
        } else {
            return TypeError::into("expected str or Ident for 'namespace'");
        };
        Ok(obj.init(Self::new(ident)))
    }
}

// --- IdspaceClause ---------------------------------------------------------

#[pyclass(extends=BaseHeaderClause)]
#[derive(Clone, Debug)]
pub struct IdspaceClause {
    prefix: IdentPrefix,
    url: Url,
    description: Option<QuotedString>,
}

impl IdspaceClause {
    pub fn new<I, U>(prefix: I, url: U) -> Self
    where
        I: Into<IdentPrefix>,
        U: Into<Url>,
    {
        Self { prefix: prefix.into(), url: url.into(), description: None }
    }

    pub fn with_description<I, U, D>(prefix: I, url: U, description: D) -> Self
    where
        I: Into<IdentPrefix>,
        U: Into<Url>,
        D: Into<Option<QuotedString>>
    {
        Self { prefix: prefix.into(), url: url.into(), description: description.into() }
    }
}

impl From<IdspaceClause> for obo::HeaderClause {
    fn from(clause: IdspaceClause) -> Self {
        obo::HeaderClause::Idspace(
            clause.prefix.into(), clause.url.into(), clause.description,
        )
    }
}

// --- TreatXrefsAsEquivalentClause ------------------------------------------

#[pyclass(extends=BaseHeaderClause)]
#[derive(Clone, Debug)]
pub struct TreatXrefsAsEquivalentClause {
    idspace: IdentPrefix,   // Should be `IdentPrefix`
}

impl TreatXrefsAsEquivalentClause {
    pub fn new<I>(idspace: I) -> Self
    where
        I: Into<IdentPrefix>
    {
        Self { idspace: idspace.into() }
    }
}

impl From<TreatXrefsAsEquivalentClause> for obo::HeaderClause {
    fn from(clause: TreatXrefsAsEquivalentClause) -> Self {
        obo::HeaderClause::TreatXrefsAsEquivalent(clause.idspace.into())
    }
}

// --- TreatXrefsAsGenusDifferentiaClause ------------------------------------

#[pyclass(extends=BaseHeaderClause)]
#[derive(Clone, Debug)]
pub struct TreatXrefsAsGenusDifferentiaClause {
    idspace: IdentPrefix,
    relation: Ident,
    filler: Ident,
}

impl TreatXrefsAsGenusDifferentiaClause {
    pub fn new<I, R, F>(idspace: I, relation: R, filler: F) -> Self
    where
        I: Into<IdentPrefix>,
        R: Into<Ident>,
        F: Into<Ident>
    {
        Self {
            idspace: idspace.into(),
            relation: relation.into(),
            filler: filler.into(),
        }
    }
}

impl From<TreatXrefsAsGenusDifferentiaClause> for obo::HeaderClause {
    fn from(clause: TreatXrefsAsGenusDifferentiaClause) -> Self {
        obo::HeaderClause::TreatXrefsAsGenusDifferentia(
            clause.idspace.into(),
            clause.relation.into(),
            clause.filler.into()
        )
    }
}

// --- TreatXrefsAsReverseGenusDifferentiaClause -----------------------------

#[pyclass(extends=BaseHeaderClause)]
#[derive(Clone, Debug)]
pub struct TreatXrefsAsReverseGenusDifferentiaClause {
    idspace: IdentPrefix,   // Should be `IdentPrefix`
    relation: Ident,  // Should be `RelationId`
    filler: Ident,    // Should be `ClassId`
}

impl TreatXrefsAsReverseGenusDifferentiaClause {
    pub fn new<I, R, F>(idspace: I, relation: R, filler: F) -> Self
    where
        I: Into<IdentPrefix>,
        R: Into<Ident>,
        F: Into<Ident>
    {
        Self {
            idspace: idspace.into(),
            relation: relation.into(),
            filler: filler.into(),
        }
    }
}

impl From<TreatXrefsAsReverseGenusDifferentiaClause> for obo::HeaderClause {
    fn from(clause: TreatXrefsAsReverseGenusDifferentiaClause) -> Self {
        obo::HeaderClause::TreatXrefsAsReverseGenusDifferentia(
            clause.idspace.into(),
            clause.relation.into(),
            clause.filler.into()
        )
    }
}

// --- TreatXrefsAsRelationshipClause ----------------------------------------

#[pyclass(extends=BaseHeaderClause)]
#[derive(Clone, Debug)]
pub struct TreatXrefsAsRelationshipClause {
    idspace: IdentPrefix,
    relation: Ident,
}

impl TreatXrefsAsRelationshipClause {
    pub fn new<I, R>(idspace: I, relation: R) -> Self
    where
        I: Into<IdentPrefix>,
        R: Into<Ident>,
    {
        Self {
            idspace: idspace.into(),
            relation: relation.into(),
        }
    }
}

impl From<TreatXrefsAsRelationshipClause> for obo::HeaderClause {
    fn from(clause: TreatXrefsAsRelationshipClause) -> Self {
        obo::HeaderClause::TreatXrefsAsRelationship(
            clause.idspace.into(),
            clause.relation.into()
        )
    }
}

// --- TreatXrefsAsIsA -------------------------------------------------------

#[pyclass(extends=BaseHeaderClause)]
#[derive(Clone, Debug)]
pub struct TreatXrefsAsIsAClause {
    idspace: IdentPrefix,
}

impl TreatXrefsAsIsAClause {
    pub fn new<I>(idspace: I) -> Self
    where
        I: Into<IdentPrefix>,
    {
        Self {
            idspace: idspace.into(),
        }
    }
}

impl From<TreatXrefsAsIsAClause> for obo::HeaderClause {
    fn from(clause: TreatXrefsAsIsAClause) -> obo::HeaderClause {
        obo::HeaderClause::TreatXrefsAsIsA(clause.idspace.into())
    }
}

// --- TreatXrefsAsHasSubclassClause -----------------------------------------

#[pyclass(extends=BaseHeaderClause)]
#[derive(Clone, Debug)]
pub struct TreatXrefsAsHasSubclassClause {
    idspace: IdentPrefix,
}

impl TreatXrefsAsHasSubclassClause {
    pub fn new<I>(idspace: I) -> Self
    where
        I: Into<IdentPrefix>
    {
        Self { idspace: idspace.into() }
    }
}

impl From<TreatXrefsAsHasSubclassClause> for obo::HeaderClause {
    fn from(clause: TreatXrefsAsHasSubclassClause) -> obo::HeaderClause {
        obo::HeaderClause::TreatXrefsAsIsA(clause.idspace.into())
    }
}


// --- PropertyValue ---------------------------------------------------------

#[pyclass(extends=BaseHeaderClause)]
#[derive(Clone, Debug)]
pub struct PropertyValueClause {
    inner: PropertyValue,
}

impl PropertyValueClause {
    fn new<P>(property_value: P) -> Self
    where
        P: Into<PropertyValue>
    {
        Self { inner: property_value.into() }
    }
}

impl From<PropertyValueClause> for ast::HeaderClause {
    fn from(clause: PropertyValueClause) -> ast::HeaderClause {
        ast::HeaderClause::PropertyValue(clause.inner.into())
    }
}

// --- Remark ----------------------------------------------------------------

#[pyclass(extends=BaseHeaderClause)]
#[derive(Clone, Debug)]pub struct RemarkClause {
    remark: UnquotedString
}

impl RemarkClause {
    pub fn new(remark: ast::UnquotedString) -> Self {
        Self { remark }
    }
}

impl Display for RemarkClause {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let clause: obo::HeaderClause = self.clone().into();
        clause.fmt(f)
    }
}

impl From<RemarkClause> for obo::HeaderClause {
    fn from(clause: RemarkClause) -> Self {
        obo::HeaderClause::Remark(clause.remark)
    }
}

#[pyproto]
impl PyObjectProtocol for RemarkClause {
    fn __repr__(&self) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let fmt = PyString::new(py, "RemarkClause({!r})").to_object(py);
        fmt.call_method1(py, "format", (self.remark.as_str(),))
    }
}

// --- Ontology --------------------------------------------------------------

#[pyclass(extends=BaseHeaderClause)]
#[derive(Clone, Debug)]
pub struct OntologyClause {
    ontology: UnquotedString
}

impl OntologyClause {
    pub fn new(ontology: ast::UnquotedString) -> Self {
        Self { ontology }
    }
}

impl From<OntologyClause> for obo::HeaderClause {
    fn from(clause: OntologyClause) -> Self {
        obo::HeaderClause::Ontology(clause.ontology)
    }

}

#[pyproto]
impl PyObjectProtocol for OntologyClause {
    fn __repr__(&self) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let fmt = PyString::new(py, "OntologyClause({!r})").to_object(py);
        fmt.call_method1(py, "format", (self.ontology.as_str(),))
    }
}

// --- OwlAxioms -------------------------------------------------------------

#[pyclass(extends=BaseHeaderClause)]
#[derive(Debug, Clone)]
pub struct OwlAxiomsClause {
    axioms: UnquotedString
}

impl OwlAxiomsClause {
    pub fn new(axioms: ast::UnquotedString) -> Self {
        Self { axioms }
    }
}

impl From<OwlAxiomsClause> for obo::HeaderClause {
    fn from(clause: OwlAxiomsClause) -> Self {
        obo::HeaderClause::OwlAxioms(clause.axioms)
    }
}

#[pyproto]
impl PyObjectProtocol for OwlAxiomsClause {
    fn __repr__(&self) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let fmt = PyString::new(py, "OwlAxiomsClause({!r})").to_object(py);
        fmt.call_method1(py, "format", (self.axioms.as_str(),))
    }
}

// --- UnreservedClause ------------------------------------------------------

#[pyclass(extends=BaseHeaderClause)]
#[derive(Clone, Debug)]
pub struct UnreservedClause {
    tag: UnquotedString,
    value: UnquotedString
}

impl UnreservedClause {
    pub fn new(tag: UnquotedString, value: UnquotedString) -> Self {
        Self { tag, value }
    }
}

impl From<UnreservedClause> for obo::HeaderClause {
    fn from(clause: UnreservedClause) -> obo::HeaderClause {
        obo::HeaderClause::Unreserved(clause.tag, clause.value)
    }
}

#[pymethods]
impl UnreservedClause {
    #[new]
    fn __init__(obj: &PyRawObject, tag: String, value: String) {
        obj.init(Self::new(UnquotedString::new(tag), UnquotedString::new(value)))
    }

    #[getter]
    fn get_tag(&self) -> PyResult<&str> {
        Ok(self.tag.as_str())
    }

    #[setter]
    fn set_tag(&mut self, tag: String) -> PyResult<()> {
        self.tag = UnquotedString::new(tag);
        Ok(())
    }

    #[getter]
    fn get_value(&self) -> PyResult<&str> {
        Ok(self.tag.as_str())
    }

    #[setter]
    fn set_value(&mut self, value: String) -> PyResult<()> {
        self.value = UnquotedString::new(value);
        Ok(())
    }
}

#[pyproto]
impl PyObjectProtocol for UnreservedClause {
    fn __repr__(&self) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let fmt = PyString::new(py, "UnreservedClause({!r}, {!r})").to_object(py);
        fmt.call_method1(py, "format", (self.tag.as_str(), self.value.as_str()))
    }
}
