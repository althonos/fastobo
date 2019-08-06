#![cfg_attr(feature = "_doc", feature(doc_cfg, external_doc))]
#![cfg_attr(feature = "_doc", doc(include = "../README.md"))]
#![warn(clippy::all)]
#![allow(dead_code, unused_imports)]

#[macro_use]
extern crate err_derive;
#[macro_use]
extern crate opaque_typedef_macros;

#[macro_use]
extern crate fastobo_derive_internal;
extern crate fastobo_syntax;

#[cfg(feature = "memchr")]
extern crate memchr;
extern crate opaque_typedef;
extern crate ordered_float;
extern crate pest;
#[cfg(test)]
extern crate textwrap;
extern crate url;

#[macro_use]
pub mod parser;

pub mod ast;
pub mod error;
pub mod semantics;
pub mod share;
pub mod visit;

use std::convert::TryFrom;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;

use self::ast::OboDoc;
use self::error::Error;
use self::error::Result;
use self::parser::FrameReader;

// ---------------------------------------------------------------------------

/// Parse an OBO document from a string.
#[inline]
pub fn from_str<S: AsRef<str>>(src: S) -> Result<OboDoc> {
    OboDoc::from_str(src.as_ref()).map_err(Error::from)
}

/// Parse an OBO document from a `BufRead` implementor.
#[inline]
pub fn from_reader<B: BufRead>(r: B) -> Result<OboDoc> {
    FrameReader::new(r)
        .map_err(Error::from)
        .and_then(|r| OboDoc::try_from(r))
}

/// Parse an OBO document from a file on the local filesystem.
#[inline]
pub fn from_file<P: AsRef<Path>>(path: P) -> Result<OboDoc> {
    let pathref = path.as_ref();
    File::open(pathref)
        .map(BufReader::new)
        .map_err(From::from)
        .and_then(|r| from_reader(r))
        .map_err(|e| {
            if let Error::SyntaxError { error } = e {
                error.with_path(&pathref.to_string_lossy()).into()
            } else {
                e
            }
        })
}

// ---------------------------------------------------------------------------

#[inline]
pub fn to_writer<W>(mut writer: W, doc: &OboDoc) -> Result<()>
where
    W: Write,
{
    write!(writer, "{}", doc.header())?;
    if !doc.header().is_empty() && !doc.entities().is_empty() {
        write!(writer, "\n")?;
    }
    for entity in doc.entities() {
        write!(writer, "{}", entity)?;
    }
    Ok(())
}

#[inline]
pub fn to_file<P: AsRef<Path>>(path: P, doc: &OboDoc) -> Result<()> {
    File::create(path)
        .map_err(From::from)
        .and_then(|r| to_writer(r, doc).map_err(From::from))
}
