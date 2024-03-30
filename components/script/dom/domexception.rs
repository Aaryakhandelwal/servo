/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct;
use js::rust::HandleObject;

use crate::dom::bindings::codegen::Bindings::DOMExceptionBinding::{
    DOMExceptionConstants, DOMExceptionMethods,
};
use crate::dom::bindings::error::Error;
use crate::dom::bindings::reflector::{
    reflect_dom_object, reflect_dom_object_with_proto, Reflector,
};
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::DOMString;
use crate::dom::globalscope::GlobalScope;

#[repr(u16)]
#[derive(Clone, Copy, Debug, Eq, JSTraceable, MallocSizeOf, Ord, PartialEq, PartialOrd)]
pub enum DOMErrorName {
    IndexSize = DOMExceptionConstants::INDEX_SIZE_ERR,
    HierarchyRequest = DOMExceptionConstants::HIERARCHY_REQUEST_ERR,
    WrongDocument = DOMExceptionConstants::WRONG_DOCUMENT_ERR,
    InvalidCharacter = DOMExceptionConstants::INVALID_CHARACTER_ERR,
    NoModificationAllowed = DOMExceptionConstants::NO_MODIFICATION_ALLOWED_ERR,
    NotFound = DOMExceptionConstants::NOT_FOUND_ERR,
    NotSupported = DOMExceptionConstants::NOT_SUPPORTED_ERR,
    InUseAttribute = DOMExceptionConstants::INUSE_ATTRIBUTE_ERR,
    InvalidState = DOMExceptionConstants::INVALID_STATE_ERR,
    Syntax = DOMExceptionConstants::SYNTAX_ERR,
    InvalidModification = DOMExceptionConstants::INVALID_MODIFICATION_ERR,
    Namespace = DOMExceptionConstants::NAMESPACE_ERR,
    InvalidAccess = DOMExceptionConstants::INVALID_ACCESS_ERR,
    Security = DOMExceptionConstants::SECURITY_ERR,
    Network = DOMExceptionConstants::NETWORK_ERR,
    Abort = DOMExceptionConstants::ABORT_ERR,
    TypeMismatch = DOMExceptionConstants::TYPE_MISMATCH_ERR,
    URLMismatch = DOMExceptionConstants::URL_MISMATCH_ERR,
    QuotaExceeded = DOMExceptionConstants::QUOTA_EXCEEDED_ERR,
    Timeout = DOMExceptionConstants::TIMEOUT_ERR,
    InvalidNodeType = DOMExceptionConstants::INVALID_NODE_TYPE_ERR,
    DataClone = DOMExceptionConstants::DATA_CLONE_ERR,
    NotReadable,
    Operation,
}

impl DOMErrorName {
    pub fn from(s: &DOMString) -> Option<DOMErrorName> {
        match s.as_ref() {
            "IndexSizeError" => Some(DOMErrorName::IndexSize),
            "HierarchyRequestError" => Some(DOMErrorName::HierarchyRequest),
            "WrongDocumentError" => Some(DOMErrorName::WrongDocument),
            "InvalidCharacterError" => Some(DOMErrorName::InvalidCharacter),
            "NoModificationAllowedError" => Some(DOMErrorName::NoModificationAllowed),
            "NotFoundError" => Some(DOMErrorName::NotFound),
            "NotSupportedError" => Some(DOMErrorName::NotSupported),
            "InUseAttributeError" => Some(DOMErrorName::InUseAttribute),
            "InvalidStateError" => Some(DOMErrorName::InvalidState),
            "SyntaxError" => Some(DOMErrorName::Syntax),
            "InvalidModificationError" => Some(DOMErrorName::InvalidModification),
            "NamespaceError" => Some(DOMErrorName::Namespace),
            "InvalidAccessError" => Some(DOMErrorName::InvalidAccess),
            "SecurityError" => Some(DOMErrorName::Security),
            "NetworkError" => Some(DOMErrorName::Network),
            "AbortError" => Some(DOMErrorName::Abort),
            "TypeMismatchError" => Some(DOMErrorName::TypeMismatch),
            "URLMismatchError" => Some(DOMErrorName::URLMismatch),
            "QuotaExceededError" => Some(DOMErrorName::QuotaExceeded),
            "TimeoutError" => Some(DOMErrorName::Timeout),
            "InvalidNodeTypeError" => Some(DOMErrorName::InvalidNodeType),
            "DataCloneError" => Some(DOMErrorName::DataClone),
            "NotReadableError" => Some(DOMErrorName::NotReadable),
            "OperationError" => Some(DOMErrorName::Operation),
            _ => None,
        }
    }
}

#[dom_struct]
pub struct DOMException {
    reflector_: Reflector,
    message: DOMString,
    name: DOMString,
}

impl DOMException {
    fn get_error_data_by_code(code: DOMErrorName) -> (DOMString, DOMString) {
        let message = match &code {
            DOMErrorName::IndexSize => "The index is not in the allowed range.",
            DOMErrorName::HierarchyRequest => {
                "The operation would yield an incorrect node tree."
            },
            DOMErrorName::WrongDocument => "The object is in the wrong document.",
            DOMErrorName::InvalidCharacter => "The string contains invalid characters.",
            DOMErrorName::NoModificationAllowed => "The object can not be modified.",
            DOMErrorName::NotFound => "The object can not be found here.",
            DOMErrorName::NotSupported => "The operation is not supported.",
            DOMErrorName::InUseAttribute => "The attribute already in use.",
            DOMErrorName::InvalidState => "The object is in an invalid state.",
            DOMErrorName::Syntax => "The string did not match the expected pattern.",
            DOMErrorName::InvalidModification => "The object can not be modified in this way.",
            DOMErrorName::Namespace => "The operation is not allowed by Namespaces in XML.",
            DOMErrorName::InvalidAccess => {
                "The object does not support the operation or argument."
            },
            DOMErrorName::Security => "The operation is insecure.",
            DOMErrorName::Network => "A network error occurred.",
            DOMErrorName::Abort => "The operation was aborted.",
            DOMErrorName::TypeMismatch => "The given type does not match any expected type.",
            DOMErrorName::URLMismatch => "The given URL does not match another URL.",
            DOMErrorName::QuotaExceeded => "The quota has been exceeded.",
            DOMErrorName::Timeout => "The operation timed out.",
            DOMErrorName::InvalidNodeType => {
                "The supplied node is incorrect or has an incorrect ancestor for this operation."
            },
            DOMErrorName::DataClone => "The object can not be cloned.",
            DOMErrorName::NotReadable => "The I/O read operation failed.",
            DOMErrorName::Operation => {
                "The operation failed for an operation-specific reason."
            },
        };

        (
            DOMString::from(message),
            DOMString::from(format!("{:?}", code)),
        )
    }

    fn new_inherited(message_: DOMString, name_: DOMString) -> DOMException {
        DOMException {
            reflector_: Reflector::new(),
            message: message_,
            name: name_,
        }
    }

    pub fn new(global: &GlobalScope, code: DOMErrorName) -> DomRoot<DOMException> {
        let (message, name) = DOMException::get_error_data_by_code(code);

        reflect_dom_object(Box::new(DOMException::new_inherited(message, name)), global)
    }

    #[allow(non_snake_case)]
    pub fn Constructor(
        global: &GlobalScope,
        proto: Option<HandleObject>,
        message: DOMString,
        name: DOMString,
    ) -> Result<DomRoot<DOMException>, Error> {
        Ok(reflect_dom_object_with_proto(
            Box::new(DOMException::new_inherited(message, name)),
            global,
            proto,
        ))
    }

    // not an IDL stringifier, used internally
    pub fn stringifier(&self) -> DOMString {
        DOMString::from(format!("{}: {}", self.name, self.message))
    }
}

impl DOMExceptionMethods for DOMException {
    // https://heycam.github.io/webidl/#dom-domexception-code
    fn Code(&self) -> u16 {
        match DOMErrorName::from(&self.name) {
            Some(code) if code <= DOMErrorName::DataClone => code as u16,
            _ => 0,
        }
    }

    // https://heycam.github.io/webidl/#idl-DOMException-error-names
    fn Name(&self) -> DOMString {
        self.name.clone()
    }

    // https://heycam.github.io/webidl/#error-names
    fn Message(&self) -> DOMString {
        self.message.clone()
    }
}
