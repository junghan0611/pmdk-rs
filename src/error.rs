//
// Copyright (c) 2019 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use failure::{Backtrace, Context, Fail, ResultExt};

use pmdk_sys::obj::pmemobj_errormsg;
use std::convert::Into;
use std::ffi::CStr;
use std::fmt;

#[derive(Debug)]
pub struct Error {
    inner: Context<Kind>,
}

#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum Kind {
    #[fail(display = "Generic Error")]
    GenericError,

    #[fail(display = "PMDK status Error")]
    PmdkError(String),

    #[fail(display = "PMDK get error message failed")]
    PmdkNoMsgError,
}

impl Fail for Error {
    fn cause(&self) -> Option<&dyn Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

impl Error {
    pub fn kind(&self) -> Kind {
        self.inner.get_context().clone()
    }

    pub fn obj_error() -> Error {
        unsafe {
            let msg = pmemobj_errormsg();
            if msg.is_null() {
                Kind::PmdkNoMsgError.into()
            } else {
                CStr::from_ptr(msg)
                    .to_owned()
                    .into_string()
                    .map(|msg| Kind::PmdkError(msg).into())
                    .wrap_err(Kind::PmdkNoMsgError)
                    .unwrap_or_else(|e| e)
            }
        }
    }
}

impl From<Kind> for Error {
    fn from(kind: Kind) -> Self {
        Self {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<Kind>> for Error {
    fn from(inner: Context<Kind>) -> Self {
        Self { inner }
    }
}

pub(crate) trait WrapErr<T, E>: ResultExt<T, E> {
    fn wrap_err(self, kind: Kind) -> Result<T, Error>;
}

#[allow(clippy::use_self)]
impl<T, E> WrapErr<T, E> for Result<T, E>
    where
        Self: ResultExt<T, E>,
{
    fn wrap_err(self, kind: Kind) -> Result<T, Error> {
        self.context(kind).map_err(Into::into)
    }
}