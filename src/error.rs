use std::fmt;
use std::ffi::NulError;
use ::Augeas;
use ::util::ptr_to_string;
use augeas_sys as raw;

pub enum Error {
    Augeas(AugeasError),
    Nul(NulError)
}

#[derive(Clone,PartialEq,Debug,Default)]
pub struct AugeasError {
    code          : raw::ErrorCode,
    message       : Option<String>,
    minor_message : Option<String>,
    details       : Option<String>
}

impl AugeasError {
    pub fn new_no_mem<S: Into<String>>(message: S) -> AugeasError {
        AugeasError {
            code : raw::ErrorCode::NoMem,
            message : Some(message.into()),
            .. Default::default()
        }
    }
}

impl ::std::error::Error for AugeasError {
    fn description(&self) -> &str {
        match self.message {
            None => "No description",
            Some(ref s) => s
        }
    }
}

fn maybe_write(f: &mut fmt::Formatter, opt : &Option<String>) -> fmt::Result {
    match *opt {
        Some(ref s) => write!(f, "      {}\n", s),
        None => Ok(())
    }
}

impl fmt::Display for AugeasError {
    // Write
    //   augeas error:{code}:{message}
    //                {minor_message}
    //                {details}
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let m = self.message.as_ref().map(String::as_ref).unwrap_or("");
        write!(f, "augeas error:{:?}:{}\n", self.code, m).
            and(maybe_write(f, &self.minor_message)).
            and(maybe_write(f, &self.details))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Augeas(ref err) => err.fmt(f),
            Error::Nul(ref err) => err.fmt(f)
        }
    }
}

impl From<NulError> for Error {
    fn from(err : NulError) -> Error {
        Error::Nul(err)
    }
}

impl <'a> From<&'a Augeas> for Error {
    fn from(aug: &'a Augeas) -> Error {
        let err = unsafe { raw::aug_error(aug.aug) };
        let msg = unsafe { ptr_to_string(raw::aug_error_message(aug.aug)) };
        let mmsg = unsafe { ptr_to_string(raw::aug_error_minor_message(aug.aug)) };
        let det = unsafe { ptr_to_string(raw::aug_error_details(aug.aug)) };
        Error::Augeas(AugeasError {
            code : err,
            message : msg,
            minor_message : mmsg,
            details : det
       })
    }
}