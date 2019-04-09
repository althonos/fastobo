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
use pyo3::class::basic::CompareOp;

use crate::id::Url;
use crate::id::Ident;
use crate::id::IdentPrefix;
use crate::id::BaseIdent;
use crate::pv::PropertyValue;

// --- Macros ----------------------------------------------------------------

macro_rules! impl_richmp {
    ($self:ident, $other:ident, $op:ident, $(self . $attr:ident)&&*) => ({
        match $op {
            CompareOp::Eq => {
                if let Ok(ref clause) = $other.downcast_ref::<Self>() {
                    Ok(($($self.$attr == clause.$attr)&&*).to_object($other.py()))
                } else {
                    Ok(false.to_object($other.py()))
                }
            }
            CompareOp::Ne => {
                if let Ok(ref clause) = $other.downcast_ref::<Self>() {
                    Ok(($($self.$attr != clause.$attr)||*).to_object($other.py()))
                } else {
                    Ok(true.to_object($other.py()))
                }
            }
            _ => Ok($other.py().NotImplemented())
        }
    });
}

macro_rules! impl_repr {
    ($self:ident, $cls:ident($(self . $attr:ident),*)) => ({
        let gil = Python::acquire_gil();
        let py = gil.python();

        let fmt = PyString::new(
            py,
            concat!(stringify!($cls), "({!r})")
        ).to_object(py);

        fmt.call_method1(
            py, "format",
            ($($self . $attr . to_object(py) ,)*)
        )
    })
}


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
            FormatVersion(v) =>
                Py::new(py, FormatVersionClause::new(py, v))
                    .map(HeaderClause::FormatVersion),
            DataVersion(v) =>
                Py::new(py, DataVersionClause::new(py, v))
                    .map(HeaderClause::DataVersion),
            Date(dt) =>
                Py::new(py, DateClause::new(py, dt))
                    .map(HeaderClause::Date),
            SavedBy(name) =>
                Py::new(py, SavedByClause::new(py, name))
                    .map(HeaderClause::SavedBy),
            AutoGeneratedBy(name) =>
                Py::new(py, AutoGeneratedByClause::new(py, name))
                    .map(HeaderClause::AutoGeneratedBy),
            Import(i) =>
                Py::new(py, ImportClause::new(py, i))
                    .map(HeaderClause::Import),
            Subsetdef(s, q) =>
                Py::new(py, SubsetdefClause::new(py, s, q))
                    .map(HeaderClause::Subsetdef),
            SynonymTypedef(ty, desc, scope) =>
                //FIXME
                Py::new(py, SynonymTypedefClause::with_scope(py, ty, desc, scope))
                    .map(HeaderClause::SynonymTypedef),
            DefaultNamespace(ns) =>
                Py::new(py, DefaultNamespaceClause::new(py, ns))
                    .map(HeaderClause::DefaultNamespace),
            Idspace(prefix, url, desc) =>
                Py::new(py, IdspaceClause::with_description(py, prefix, url, desc))
                    .map(HeaderClause::Idspace),
            TreatXrefsAsEquivalent(prefix) =>
                Py::new(py, TreatXrefsAsEquivalentClause::new(py, prefix))
                    .map(HeaderClause::TreatXrefsAsEquivalent),
            TreatXrefsAsGenusDifferentia(p, r, c) =>
                Py::new(py, TreatXrefsAsGenusDifferentiaClause::new(py, p, r, c))
                    .map(HeaderClause::TreatXrefsAsGenusDifferentia),
            TreatXrefsAsReverseGenusDifferentia(p, r, c) =>
                Py::new(py, TreatXrefsAsReverseGenusDifferentiaClause::new(py, p, r, c))
                    .map(HeaderClause::TreatXrefsAsReverseGenusDifferentia),
            TreatXrefsAsRelationship(p, r) =>
                Py::new(py, TreatXrefsAsRelationshipClause::new(py, p, r))
                    .map(HeaderClause::TreatXrefsAsRelationship),
            TreatXrefsAsIsA(p) =>
                Py::new(py, TreatXrefsAsIsAClause::new(py, p))
                    .map(HeaderClause::TreatXrefsAsIsA),
            TreatXrefsAsHasSubclass(p) =>
                Py::new(py, TreatXrefsAsHasSubclassClause::new(py, p))
                    .map(HeaderClause::TreatXrefsAsHasSubclass),
            PropertyValue(pv) =>
                Py::new(py, PropertyValueClause::new(py, pv))
                    .map(HeaderClause::PropertyValue),
            Remark(r) =>
                Py::new(py, RemarkClause::new(py, r))
                    .map(HeaderClause::Remark),
            Ontology(ont) =>
                Py::new(py, OntologyClause::new(py, ont))
                    .map(HeaderClause::Ontology),
            OwlAxioms(ax) =>
                Py::new(py, OwlAxiomsClause::new(py, ax))
                    .map(HeaderClause::OwlAxioms),
            Unreserved(tag, value) =>
                Py::new(py, UnreservedClause::new(py, tag, value))
                    .map(HeaderClause::Unreserved)
        }.expect("could not allocate memory in Python heap")
    }
}

impl FromPy<HeaderClause> for fastobo::ast::HeaderClause {
    fn from_py(clause: HeaderClause, py: Python) -> Self {
        Self::from_py(&clause, py)
    }
}

// --- Base ------------------------------------------------------------------

/// A header clause, appearing in the OBO header frame.
#[pyclass(subclass)]
pub struct BaseHeaderClause {}

// --- FormatVersion ---------------------------------------------------------

/// A header clause indicating the format version of the OBO document.
#[pyclass(extends=BaseHeaderClause)]
#[derive(Debug, Clone)]
pub struct FormatVersionClause {
    version: obo::UnquotedString,
}

impl FormatVersionClause {
    pub fn new(_py: Python, version: obo::UnquotedString) -> Self {
        Self { version }
    }
}

impl Display for FormatVersionClause {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        obo::HeaderClause::from(self.clone()).fmt(f)
    }
}

impl From<FormatVersionClause> for obo::HeaderClause {
    fn from(clause: FormatVersionClause) -> Self {
        obo::HeaderClause::FormatVersion(clause.version)
    }
}

impl FromPy<FormatVersionClause> for obo::HeaderClause {
    fn from_py(clause: FormatVersionClause, _py: Python) -> Self {
        Self::from(clause)
    }
}

// WIP(@althonos):
// impl FormatVersionClause {
//     fn to_ref<'s>(&'s self) -> obo::HeaderClauseRef<'s> {
//         let s: &'s str = self.version.as_ref();
//         obo::HeaderClauseRef::FormatVersion(Cow::Borrowed(obo::UnquotedStr::new(s)))
//     }
// }

#[pymethods]
impl FormatVersionClause {
    #[new]
    fn __init__(obj: &PyRawObject, version: String) {
        obj.init(Self::new(obj.py(), obo::UnquotedString::new(version)));
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
        impl_repr!(self, FormatVersionClause(self.version))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }

    fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<PyObject> {
        impl_richmp!(self, other, op, self.version)
    }
}

// --- DataVersion -----------------------------------------------------------

#[pyclass(extends=BaseHeaderClause)]
#[derive(Clone, Debug)]
pub struct DataVersionClause {
    version: UnquotedString
}

impl DataVersionClause {
    pub fn new(_py: Python, version: UnquotedString) -> Self {
        Self {version}
    }
}

impl From<DataVersionClause> for obo::HeaderClause {
    fn from(clause: DataVersionClause) -> obo::HeaderClause {
        obo::HeaderClause::DataVersion(clause.version)
    }
}

impl FromPy<DataVersionClause> for obo::HeaderClause {
    fn from_py(clause: DataVersionClause, _py: Python) -> Self {
        Self::from(clause)
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
        obj.init(Self::new(obj.py(), UnquotedString::new(version)));
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
        impl_repr!(self, DataVersionClause(self.version))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }

    fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<PyObject> {
        impl_richmp!(self, other, op, self.version)
    }
}

// --- Date ------------------------------------------------------------------

#[pyclass(extends=BaseHeaderClause)]
#[derive(Clone, Debug)]
pub struct DateClause {
    date: obo::NaiveDateTime,
}

impl DateClause {
    pub fn new(_py: Python, date: obo::NaiveDateTime) -> Self {
        Self { date }
    }
}

impl From<DateClause> for obo::HeaderClause {
    fn from(clause: DateClause) -> obo::HeaderClause {
        obo::HeaderClause::Date(clause.date)
    }
}

impl FromPy<DateClause> for obo::HeaderClause {
    fn from_py(clause: DateClause, _py: Python) -> Self {
        Self::from(clause)
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
        obj.init(Self::new(obj.py(), dt))
    }

    #[getter]
    fn get_date(&self) -> PyResult<Py<PyDateTime>> {
        let py = unsafe { Python::assume_gil_acquired() };
        PyDateTime::new(
            py,
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
    pub fn new(_py: Python, name: UnquotedString) -> Self  {
        Self {name}
    }
}

impl From<SavedByClause> for obo::HeaderClause {
    fn from(clause: SavedByClause) -> obo::HeaderClause {
        obo::HeaderClause::SavedBy(clause.name)
    }
}

impl FromPy<SavedByClause> for obo::HeaderClause {
    fn from_py(clause: SavedByClause, _py: Python) -> obo::HeaderClause {
        Self::from(clause)
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
        obj.init(Self::new(obj.py(), UnquotedString::new(version)));
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
        impl_repr!(self, SavedByClause(self.name))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }

    fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<PyObject> {
        impl_richmp!(self, other, op, self.name)
    }
}

// --- AutoGeneratedBy -------------------------------------------------------

#[pyclass(extends=BaseHeaderClause)]
#[derive(Clone, Debug)]
pub struct AutoGeneratedByClause {
    name: UnquotedString
}

impl AutoGeneratedByClause {
    pub fn new(_py: Python, name: UnquotedString) -> Self {
        Self { name }
    }
}

impl From<AutoGeneratedByClause> for obo::HeaderClause {
    fn from(clause: AutoGeneratedByClause) -> obo::HeaderClause {
        obo::HeaderClause::AutoGeneratedBy(clause.name)
    }
}

impl FromPy<AutoGeneratedByClause> for obo::HeaderClause {
    fn from_py(clause: AutoGeneratedByClause, _py: Python) -> obo::HeaderClause {
        Self::from(clause)
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
        obj.init(Self::new(obj.py(), UnquotedString::new(version)));
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

#[pyproto]
impl PyObjectProtocol for AutoGeneratedByClause {

    fn __repr__(&self) -> PyResult<PyObject> {
        impl_repr!(self, AutoGeneratedByClause(self.name))
    }

    fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<PyObject> {
        impl_richmp!(self, other, op, self.name)
    }
}

// --- Import ----------------------------------------------------------------

#[pyclass(extends=BaseHeaderClause)]
#[derive(Clone, Debug)]
pub struct ImportClause {
    reference: obo::Import, // should be `Import` ?
}

impl ImportClause {
    pub fn new(_py: Python, reference: obo::Import) -> Self {
        Self { reference }
    }
}

impl From<ImportClause> for obo::HeaderClause {
    fn from(clause: ImportClause) -> Self {
        obo::HeaderClause::Import(clause.reference)
    }
}

impl FromPy<ImportClause> for obo::HeaderClause {
    fn from_py(clause: ImportClause, _py: Python) -> Self {
        Self::from(clause)
    }
}

#[pymethods]
impl ImportClause {
    // FIXME(@althonos): should not be implicit here ?
    #[new]
    pub fn __init__(obj: &PyRawObject, reference: &str) -> PyResult<()> {
        let py = obj.py();
        if let Ok(url) = url::Url::from_str(reference) {
            Ok(obj.init(Self::new(py, obo::Import::Url(url))))
        } else if let Ok(id) = obo::Ident::from_str(reference) {
            Ok(obj.init(Self::new(py, obo::Import::Abbreviated(id))))
        } else {
            ValueError::into(format!("invalid import: {:?}", reference))
        }
    }
}

#[pyproto]
impl PyObjectProtocol for ImportClause {
    fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<PyObject> {
        impl_richmp!(self, other, op, self.reference)
    }
}

// --- Subsetdef -------------------------------------------------------------

#[pyclass(extends=BaseHeaderClause)]
#[derive(Clone, Debug)]
pub struct SubsetdefClause {
    subset: Ident, // FIXME: Py<Ident>,
    description: QuotedString,
}

impl SubsetdefClause {
    pub fn new<I>(py: Python, subset: I, description: QuotedString) -> Self
    where
        I: IntoPy<Ident>
    {
        Self {
            subset: subset.into_py(py),
            description
        }
    }
}

impl FromPy<SubsetdefClause> for obo::HeaderClause {
    fn from_py(clause: SubsetdefClause, py: Python) -> Self {
        obo::HeaderClause::Subsetdef(
            obo::SubsetIdent::from_py(clause.subset, py),
            clause.description
        )
    }
}

impl Display for SubsetdefClause {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let gil = Python::acquire_gil();
        let py = gil.python();
        obo::HeaderClause::from_py(self.clone(), py).fmt(f)
    }
}

#[pymethods]
impl SubsetdefClause {

    #[new]
    fn __init__(obj: &PyRawObject, subset: Ident, description: String) -> PyResult<()> {
        let py = obj.py();
        Ok(obj.init(Self::new(py, subset, QuotedString::new(description))))
    }

    #[getter]
    fn get_subset(&self) -> PyResult<PyObject> {
        let py = unsafe { Python::assume_gil_acquired() };
        Ok(self.subset.to_object(py))
    }

    #[setter]
    fn set_subset(&mut self, subset: Ident) -> PyResult<()> {
        self.subset = subset;
        Ok(())
    }

}

#[pyproto]
impl PyObjectProtocol for SubsetdefClause {
    fn __repr__(&self) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let r = self.subset.to_object(py).call_method0(py, "__repr__")?;
        let fmt = PyString::new(py, "SubsetdefClause({}, {!r})").to_object(py);
        fmt.call_method1(py, "format", (r, self.description.as_str()))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }
}

// --- SynonymTypedef --------------------------------------------------------

#[pyclass(extends=BaseHeaderClause)]
#[derive(Clone, Debug)]
pub struct SynonymTypedefClause {
    typedef: Ident,  // FIXME: should be Py<Ident>
    description: QuotedString,
    scope: Option<fastobo::ast::SynonymScope>,  // FIXME: Python type
}

impl SynonymTypedefClause {
    pub fn new<T, D>(py: Python, typedef: T, description: D) -> Self
    where
        T: IntoPy<Ident>,
        D: Into<QuotedString>,
    {
        Self {
            typedef: typedef.into_py(py),
            description: description.into(),
            scope: None
        }
    }

    pub fn with_scope<T, D, S>(py: Python, typedef: T, description: D, scope: S) -> Self
    where
        T: IntoPy<Ident>,
        D: Into<QuotedString>,
        S: Into<Option<fastobo::ast::SynonymScope>>
    {
        Self {
            typedef: typedef.into_py(py),
            description: description.into(),
            scope: scope.into()
        }
    }
}

impl FromPy<SynonymTypedefClause> for obo::HeaderClause {
    fn from_py(clause: SynonymTypedefClause, py: Python) -> Self {
        obo::HeaderClause::SynonymTypedef(
            clause.typedef.into_py(py),
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
        obj.init(Self::with_scope(obj.py(), typedef, desc, sc));
    }
}

// --- DefaultNamespace ------------------------------------------------------

#[pyclass(extends=BaseHeaderClause)]
#[derive(Clone, Debug)]
pub struct DefaultNamespaceClause {
    #[pyo3(get, set)]
    namespace: Ident,    // should be Py<Ident>
}

impl DefaultNamespaceClause {
    pub fn new<I>(py: Python, namespace: I) -> Self
    where
        I: IntoPy<Ident>,
    {
        Self { namespace: namespace.into_py(py) }
    }
}

impl Display for DefaultNamespaceClause {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let gil = Python::acquire_gil();
        let py = gil.python();
        obo::HeaderClause::from_py(self.clone(), py).fmt(f)
    }
}

impl FromPy<DefaultNamespaceClause> for obo::HeaderClause {
    fn from_py(clause: DefaultNamespaceClause, py: Python) -> Self {
        obo::HeaderClause::DefaultNamespace(clause.namespace.into_py(py))
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
            Ident::from_py(id, py)
        } else {
            return TypeError::into("expected str or Ident for 'namespace'");
        };
        Ok(obj.init(Self::new(py, ident)))
    }
}

#[pyproto]
impl PyObjectProtocol for DefaultNamespaceClause {
    fn __repr__(&self) -> PyResult<PyObject> {

        let gil = Python::acquire_gil();
        let py = gil.python();

        let ns = self.namespace.to_object(py);
        let nsref = ns.as_ref(py);

        let fmt = PyString::new(py, "DefaultNamespaceClause({})").to_object(py);
        fmt.call_method1(py, "format", (nsref.repr()?, ))

    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.clone().to_string())
    }

    fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<PyObject> {
        impl_richmp!(self, other, op, self.namespace)
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
    pub fn new<I, U>(_py: Python, prefix: I, url: U) -> Self
    where
        I: Into<IdentPrefix>,
        U: Into<Url>,
    {
        Self {
            prefix: prefix.into(),
            url: url.into(),
            description: None
        }
    }

    pub fn with_description<I, U, D>(_py: Python, prefix: I, url: U, description: D) -> Self
    where
        I: Into<IdentPrefix>,
        U: Into<Url>,
        D: Into<Option<QuotedString>>
    {
        Self {
            prefix: prefix.into(),
            url: url.into(),
            description: description.into()
        }
    }
}

impl From<IdspaceClause> for obo::HeaderClause {
    fn from(clause: IdspaceClause) -> Self {
        obo::HeaderClause::Idspace(
            clause.prefix.into(), clause.url.into(), clause.description,
        )
    }
}

impl FromPy<IdspaceClause> for obo::HeaderClause {
    fn from_py(clause: IdspaceClause, _py: Python) -> Self {
        obo::HeaderClause::from(clause)
    }
}

// --- TreatXrefsAsEquivalentClause ------------------------------------------

#[pyclass(extends=BaseHeaderClause)]
#[derive(Clone, Debug)]
pub struct TreatXrefsAsEquivalentClause {
    idspace: IdentPrefix,   // Should be `IdentPrefix`
}

impl TreatXrefsAsEquivalentClause {
    pub fn new<I>(_py: Python, idspace: I) -> Self
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

impl FromPy<TreatXrefsAsEquivalentClause> for obo::HeaderClause {
    fn from_py(clause: TreatXrefsAsEquivalentClause, _py: Python) -> Self {
        Self::from(clause)
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
    pub fn new<I, R, F>(py: Python, idspace: I, relation: R, filler: F) -> Self
    where
        I: Into<IdentPrefix>,
        R: IntoPy<Ident>,
        F: IntoPy<Ident>
    {
        Self {
            idspace: idspace.into(),
            relation: relation.into_py(py),
            filler: filler.into_py(py),
        }
    }
}

impl FromPy<TreatXrefsAsGenusDifferentiaClause> for obo::HeaderClause {
    fn from_py(clause: TreatXrefsAsGenusDifferentiaClause, py: Python) -> Self {
        obo::HeaderClause::TreatXrefsAsGenusDifferentia(
            clause.idspace.into(),
            clause.relation.into_py(py),
            clause.filler.into_py(py),
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
    pub fn new<I, R, F>(py: Python, idspace: I, relation: R, filler: F) -> Self
    where
        I: Into<IdentPrefix>,
        R: IntoPy<Ident>,
        F: IntoPy<Ident>
    {
        Self {
            idspace: idspace.into(),
            relation: relation.into_py(py),
            filler: filler.into_py(py),
        }
    }
}

impl FromPy<TreatXrefsAsReverseGenusDifferentiaClause> for obo::HeaderClause {
    fn from_py(clause: TreatXrefsAsReverseGenusDifferentiaClause, py: Python) -> Self {
        obo::HeaderClause::TreatXrefsAsReverseGenusDifferentia(
            clause.idspace.into(),
            clause.relation.into_py(py),
            clause.filler.into_py(py)
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
    pub fn new<I, R>(py: Python, idspace: I, relation: R) -> Self
    where
        I: Into<IdentPrefix>,
        R: IntoPy<Ident>,
    {
        Self {
            idspace: idspace.into(),
            relation: relation.into_py(py),
        }
    }
}

impl FromPy<TreatXrefsAsRelationshipClause> for obo::HeaderClause {
    fn from_py(clause: TreatXrefsAsRelationshipClause, py: Python) -> Self {
        obo::HeaderClause::TreatXrefsAsRelationship(
            clause.idspace.into(),
            clause.relation.into_py(py)
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
    pub fn new<I>(_py: Python, idspace: I) -> Self
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

impl FromPy<TreatXrefsAsIsAClause> for obo::HeaderClause {
    fn from_py(clause: TreatXrefsAsIsAClause, _py: Python) -> obo::HeaderClause {
        Self::from(clause)
    }
}


// --- TreatXrefsAsHasSubclassClause -----------------------------------------

#[pyclass(extends=BaseHeaderClause)]
#[derive(Clone, Debug)]
pub struct TreatXrefsAsHasSubclassClause {
    #[pyo3(get)]
    idspace: IdentPrefix,
}

impl TreatXrefsAsHasSubclassClause {
    pub fn new<I>(_py: Python, idspace: I) -> Self
    where
        I: Into<IdentPrefix>
    {
        Self { idspace: idspace.into() }
    }
}


impl From<TreatXrefsAsHasSubclassClause> for obo::HeaderClause {
    fn from(clause: TreatXrefsAsHasSubclassClause) -> Self {
        obo::HeaderClause::TreatXrefsAsIsA(clause.idspace.into())
    }
}

impl FromPy<TreatXrefsAsHasSubclassClause> for obo::HeaderClause {
    fn from_py(clause: TreatXrefsAsHasSubclassClause, _py: Python) -> Self {
        Self::from(clause)
    }
}

#[pymethods]
impl TreatXrefsAsHasSubclassClause {
    #[setter]
    fn set_idspace(&mut self, idspace: &PyAny) -> PyResult<()> {
        if let Ok(i) = idspace.downcast_ref::<IdentPrefix>() {
            self.idspace = i.clone();
            Ok(())
        } else if let Ok(s) = idspace.downcast_ref::<PyString>() {
            let i = ast::IdentPrefix::new(s.to_string()?.to_string());
            self.idspace = IdentPrefix::new(i);
            Ok(())
        } else {
            TypeError::into("expected str or IdentPrefix")
        }
    }
}

#[pyproto]
impl PyObjectProtocol for TreatXrefsAsHasSubclassClause {
    fn __repr__(&self) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let fmt = PyString::new(py, "OwlAxiomsClause({!r})").to_object(py);
        fmt.call_method1(py, "format", (self.idspace.as_ref(py).as_str(),))
    }
}

// --- PropertyValue ---------------------------------------------------------

#[pyclass(extends=BaseHeaderClause)]
#[derive(Clone, Debug)]
pub struct PropertyValueClause {
    inner: PropertyValue,
}

impl PropertyValueClause {
    fn new<P>(py: Python, property_value: P) -> Self
    where
        P: IntoPy<PropertyValue>
    {
        Self { inner: property_value.into_py(py) }
    }
}

impl FromPy<PropertyValueClause> for ast::HeaderClause {
    fn from_py(clause: PropertyValueClause, py: Python) -> ast::HeaderClause {
        ast::HeaderClause::PropertyValue(clause.inner.into_py(py))
    }
}

// --- Remark ----------------------------------------------------------------

#[pyclass(extends=BaseHeaderClause)]
#[derive(Clone, Debug)]
pub struct RemarkClause {
    remark: UnquotedString
}

impl RemarkClause {
    pub fn new(_py: Python, remark: ast::UnquotedString) -> Self {
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

impl FromPy<RemarkClause> for obo::HeaderClause {
    fn from_py(clause: RemarkClause, _py: Python) -> Self {
        Self::from(clause)
    }
}

#[pymethods]
impl RemarkClause {
    #[new]
    fn __init__(obj: &PyRawObject, remark: String) -> PyResult<()> {
        let py = obj.py();
        Ok(obj.init(Self::new(py, UnquotedString::new(remark))))
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

    fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<PyObject> {
        impl_richmp!(self, other, op, self.remark)
    }
}

// --- Ontology --------------------------------------------------------------

#[pyclass(extends=BaseHeaderClause)]
#[derive(Clone, Debug)]
pub struct OntologyClause {
    ontology: UnquotedString
}

impl OntologyClause {
    pub fn new(_py: Python, ontology: UnquotedString) -> Self {
        Self { ontology }
    }
}

impl From<OntologyClause> for obo::HeaderClause {
    fn from(clause: OntologyClause) -> Self {
        obo::HeaderClause::Ontology(clause.ontology)
    }
}

impl FromPy<OntologyClause> for obo::HeaderClause {
    fn from_py(clause: OntologyClause, _py: Python) -> Self {
        Self::from(clause)
    }
}

#[pymethods]
impl OntologyClause {
    #[new]
    fn __init__(obj: &PyRawObject, ontology: String) -> PyResult<()> {
        let py = obj.py();
        Ok(obj.init(Self::new(py, UnquotedString::new(ontology))))
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

    fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<PyObject> {
        impl_richmp!(self, other, op, self.ontology)
    }
}

// --- OwlAxioms -------------------------------------------------------------

#[pyclass(extends=BaseHeaderClause)]
#[derive(Debug, Clone)]
pub struct OwlAxiomsClause {
    axioms: UnquotedString
}

impl OwlAxiomsClause {
    pub fn new(_py: Python, axioms: UnquotedString) -> Self {
        Self { axioms }
    }
}

impl From<OwlAxiomsClause> for obo::HeaderClause {
    fn from(clause: OwlAxiomsClause) -> Self {
        obo::HeaderClause::OwlAxioms(clause.axioms)
    }
}

impl FromPy<OwlAxiomsClause> for obo::HeaderClause {
    fn from_py(clause: OwlAxiomsClause, _py: Python) -> Self {
        Self::from(clause)
    }
}

#[pymethods]
impl OwlAxiomsClause {
    #[new]
    fn __init__(obj: &PyRawObject, axioms: String) -> PyResult<()> {
        let py = obj.py();
        Ok(obj.init(Self::new(py, UnquotedString::new(axioms))))
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

    fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<PyObject> {
        impl_richmp!(self, other, op, self.axioms)
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
    pub fn new(_py: Python, tag: UnquotedString, value: UnquotedString) -> Self {
        Self { tag, value }
    }
}

impl From<UnreservedClause> for obo::HeaderClause {
    fn from(clause: UnreservedClause) -> obo::HeaderClause {
        obo::HeaderClause::Unreserved(clause.tag, clause.value)
    }
}

impl FromPy<UnreservedClause> for obo::HeaderClause {
    fn from_py(clause: UnreservedClause, _py: Python) -> Self {
        Self::from(clause)
    }
}

impl Display for UnreservedClause {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        obo::HeaderClause::from(self.clone()).fmt(f)
    }
}

#[pymethods]
impl UnreservedClause {
    #[new]
    fn __init__(obj: &PyRawObject, tag: String, value: String) {
        let py = obj.py();
        obj.init(Self::new(py, UnquotedString::new(tag), UnquotedString::new(value)))
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

    fn __str__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }

    fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<PyObject> {
        impl_richmp!(self, other, op, self.tag && self.value)
    }
}
