//! Helpers for the urlencoded module
//!
//! Provides one liners for the common usage of urlencoded

use ::iron::prelude::*;
use ::iron::status;
use ::std::fmt::{Display,Formatter};
use ::std::fmt::Error as FmtError;
use ::std::error::Error as StdError;
use super::{UrlEncodedBody,UrlEncodedQuery,QueryMap};

/// Error returned when the requested parameter is missing
#[derive(Debug, PartialEq)]
pub struct MissingParamError {
    name: String
}

impl StdError for MissingParamError {
    fn description(&self) -> &str {
        "Missing parameter"
    }
}

impl Display for MissingParamError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "Missing parameter {}", self.name)
    }
}

/// Returns the parameters hashmap constructed from the request body
///
/// # Examples
/// ```
/// fn request_handler(req: &mut Request) => IronResult<Response> {
///     let params = try!(require_body_params(req));
/// ```
pub fn require_body_params(req: &mut Request) -> IronResult<QueryMap> {
    req.get::<UrlEncodedBody>()
        .map_err(|err| IronError::new(err, status::BadRequest))
}

/// Returns the parameters hashmap constructed from the request query
///
/// # Examples
/// ```
/// fn request_handler(req: &mut Request) => IronResult<Response> {
///     let params = try!(require_query_params(req));
/// 
/// ```
pub fn require_query_params(req: &mut Request) -> IronResult<QueryMap> {
    req.get::<UrlEncodedQuery>()
        .map_err(|err| IronError::new(err, status::BadRequest))
}

/// Returns the first parameter for a given parameter name, or a `MissingParamError`
///
/// # Examples
/// ```
/// fn request_handler(req: &mut Request) => IronResult<Response> {
///     let params = try!(require_body_params(req));
///     let search = try!(require_parameter(&params, "search"));
/// 
/// ```
pub fn require_parameter<'a, T: Into<String>>(hashmap: &'a QueryMap, name: T) -> IronResult<&'a String> {
    let name_val = name.into();
    hashmap.get(&name_val)
        .and_then(|vals| vals.first())
        .ok_or(IronError::new(MissingParamError { name: name_val }, status::BadRequest))
}

#[test]
fn test_require_single() {
    let mut hash = QueryMap::new();
    hash.insert("var".to_string(), vec!["value".to_string()]);
    let val = require_parameter(&hash, "var").unwrap();
    assert_eq!(val, "value");
}
