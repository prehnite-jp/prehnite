use serde::Deserialize;
use sqlx::encode::IsNull;
use sqlx::error::BoxDynError;
use sqlx::{Database, Decode, Encode, Sqlite, Type};
use std::borrow::Cow;
use std::io::Cursor;

const COMPRESSION_LEVEL: i32 = 12;

#[derive(Debug, Clone, Default)]
pub struct ZstdCompressedBlob(Vec<u8>);

impl Type<Sqlite> for ZstdCompressedBlob {
    fn type_info() -> <Sqlite as Database>::TypeInfo {
        <Vec<u8> as Type<Sqlite>>::type_info()
    }
}

impl ZstdCompressedBlob {
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.clone()
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }

    pub const fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    pub fn json<T: for<'de> Deserialize<'de>>(&self) -> serde_json::Result<T> {
        serde_json::from_slice(self.as_slice())
    }

    pub fn text(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(self.as_slice())
    }
}

impl<'r> Decode<'r, Sqlite> for ZstdCompressedBlob {
    fn decode(value: <Sqlite as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(Self(zstd::decode_all(<&[u8] as Decode<Sqlite>>::decode(
            value,
        )?)?))
    }
}

impl<'r> Encode<'r, Sqlite> for ZstdCompressedBlob {
    fn encode_by_ref(
        &self,
        buf: &mut <Sqlite as Database>::ArgumentBuffer,
    ) -> Result<IsNull, BoxDynError> {
        <Vec<u8> as sqlx::Encode<'_, Sqlite>>::encode(
            zstd::encode_all(Cursor::new(self.0.as_slice()), COMPRESSION_LEVEL)?,
            buf,
        )
    }
    fn produces(&self) -> Option<<Sqlite as Database>::TypeInfo> {
        <Vec<u8> as ::sqlx::encode::Encode<'r, Sqlite>>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        <Vec<u8> as ::sqlx::encode::Encode<'r, Sqlite>>::size_hint(&self.0)
    }
}

impl From<Vec<u8>> for ZstdCompressedBlob {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}
