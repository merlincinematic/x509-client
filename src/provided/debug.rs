use crate::api::{X509Iterator, X509IteratorError};
use bytes::Bytes;
use std::convert::Infallible;
use std::iter;

pub struct DebugX509Iterator(Bytes);

impl From<Bytes> for DebugX509Iterator {
    fn from(bytes: Bytes) -> Self {
        Self(bytes)
    }
}

impl From<DebugX509Iterator> for Bytes {
    fn from(src: DebugX509Iterator) -> Self {
        src.0
    }
}
impl IntoIterator for DebugX509Iterator {
    type Item = Bytes;
    type IntoIter = std::iter::Once<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        iter::once(self.0)
    }
}

impl X509IteratorError for Infallible {}

impl X509Iterator for DebugX509Iterator {
    type X509IteratorError = Infallible;

    fn from_cer<T: AsRef<[u8]>>(src: T) -> Result<Self, Self::X509IteratorError> {
        Ok(Self(Bytes::copy_from_slice(src.as_ref())))
    }

    fn from_pem<T: AsRef<[u8]>>(src: T) -> Result<Self, Self::X509IteratorError> {
        Ok(Self(Bytes::copy_from_slice(src.as_ref())))
    }

    fn from_pkcs7<T: AsRef<[u8]>>(src: T) -> Result<Self, Self::X509IteratorError> {
        Ok(Self(Bytes::copy_from_slice(src.as_ref())))
    }
}
