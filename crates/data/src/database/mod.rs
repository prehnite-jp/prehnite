use sqlx::FromRow;

pub mod schema;
pub mod zstd_compressed_blob;

#[allow(unused)]
pub const SQLITE_BIND_LIMIT: i64 = 32766;

macro_rules! i64_sqlite_impl {
    ($x:ident) => {
        #[derive(
            ::core::fmt::Debug,
            ::core::clone::Clone,
            ::core::default::Default,
            ::core::cmp::PartialEq,
            ::core::cmp::PartialOrd,
            ::core::cmp::Ord,
            ::core::cmp::Eq,
            ::core::marker::Copy,
            ::serde::Deserialize,
            ::serde::Serialize,
            ::std::hash::Hash,
        )]
        pub struct $x(::std::primitive::i64);

        impl ::std::ops::Deref for $x {
            type Target = ::std::primitive::i64;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl ::sqlx::Encode<'_, ::sqlx::Sqlite> for $x {
            fn encode_by_ref(
                &self,
                buf: &mut <::sqlx::Sqlite as ::sqlx::Database>::ArgumentBuffer,
            ) -> ::std::result::Result<::sqlx::encode::IsNull, ::sqlx::error::BoxDynError> {
                <::std::primitive::i64 as ::sqlx::Encode<'_, ::sqlx::Sqlite>>::encode_by_ref(
                    &self.0, buf,
                )
            }
        }
        impl ::sqlx::Decode<'_, ::sqlx::Sqlite> for $x {
            fn decode(
                value: <::sqlx::Sqlite as ::sqlx::Database>::ValueRef<'_>,
            ) -> ::std::result::Result<Self, ::sqlx::error::BoxDynError> {
                ::std::result::Result::Ok(Self(<::std::primitive::i64 as ::sqlx::Decode<
                    '_,
                    ::sqlx::Sqlite,
                >>::decode(value)?))
            }
        }
        impl ::sqlx::Type<::sqlx::Sqlite> for $x {
            fn type_info() -> <::sqlx::Sqlite as ::sqlx::Database>::TypeInfo {
                <::std::primitive::i64 as ::sqlx::Type<::sqlx::Sqlite>>::type_info()
            }
        }
        impl ::std::fmt::Display for $x {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> std::fmt::Result {
                ::std::write!(f, "{}", self.0)
            }
        }
        impl $x {
            #[allow(unused)]
            pub(crate) fn new(id: ::std::primitive::i64) -> $x {
                $x(id)
            }
        }
    };
}

pub(crate) mod database_id {
    i64_sqlite_impl!(DocumentId);
    i64_sqlite_impl!(TagId);
    i64_sqlite_impl!(BinderId);
    i64_sqlite_impl!(BoundDocumentId);
}

#[allow(unused)]
#[derive(FromRow, Debug, Clone)]
pub(crate) struct SelectIdQueryResult<T> {
    pub(crate) id: T,
}
