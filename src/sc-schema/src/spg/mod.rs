pub use fluvio_controlplane_metadata::spg::*;

mod convert {

    use std::io::Error;
    use std::io::ErrorKind;
    use std::convert::TryInto;

    use crate::objects::*;
    use super::*;

    impl From<SpuGroupSpec> for AllCreatableSpec {
        fn from(spec: SpuGroupSpec) -> Self {
            Self::SpuGroup(spec)
        }
    }

    impl DeleteSpec for SpuGroupSpec {
        fn into_request<K>(key: K) -> DeleteRequest
        where
            K: Into<Self::DeleteKey>,
        {
            DeleteRequest::SpuGroup(key.into())
        }
    }

    impl ListSpec for SpuGroupSpec {
        type Filter = NameFilter;

        fn into_list_request(filters: Vec<Self::Filter>) -> ListRequest {
            ListRequest::SpuGroup(filters)
        }
    }

    impl TryInto<Vec<Metadata<SpuGroupSpec>>> for ListResponse {
        type Error = Error;

        fn try_into(self) -> Result<Vec<Metadata<SpuGroupSpec>>, Self::Error> {
            match self {
                ListResponse::SpuGroup(s) => Ok(s),
                _ => Err(Error::new(ErrorKind::Other, "not spg")),
            }
        }
    }

    impl From<MetadataUpdate<SpuGroupSpec>> for WatchResponse {
        fn from(update: MetadataUpdate<SpuGroupSpec>) -> Self {
            Self::SpuGroup(update)
        }
    }

    impl TryInto<MetadataUpdate<SpuGroupSpec>> for WatchResponse {
        type Error = Error;

        fn try_into(self) -> Result<MetadataUpdate<SpuGroupSpec>, Self::Error> {
            match self {
                WatchResponse::SpuGroup(m) => Ok(m),
                _ => Err(Error::new(ErrorKind::Other, "not spg")),
            }
        }
    }
}
