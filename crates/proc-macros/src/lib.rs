use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::Ident;
use structmeta::{Flag, StructMeta};
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Error, LitStr, Type};

fn type_filter(v: &Type, allow: &str) -> bool {
    if let Type::Path(path) = v {
        let seg = path.path.segments.first().unwrap();
        seg.ident == allow
    } else {
        false
    }
}

#[derive(StructMeta, Default)]
struct PrehniteDbAttributes {
    view_name: Option<LitStr>,
    table_name: Option<LitStr>,
}

impl PrehniteDbAttributes {
    fn parse(input: &DeriveInput) -> Self {
        input
            .attrs
            .iter()
            .rfind(|v| v.path().is_ident("prehnite_db"))
            .and_then(|x| x.parse_args().ok())
            .unwrap_or_default()
    }

    fn view_name(&self, struct_name: String) -> String {
        self.view_name
            .as_ref()
            .map(|v| v.value())
            .unwrap_or(self.table_name(struct_name))
    }

    fn table_name(&self, struct_name: String) -> String {
        self.table_name
            .as_ref()
            .map(|v| v.value())
            .unwrap_or(struct_name.to_case(Case::Snake))
    }
}

#[derive(StructMeta, Default)]
struct PrehniteDbFieldAttributes {
    skip: Flag,
    skip_update: Flag,
    skip_create: Flag,
    use_id: Flag,
    use_string_from: Flag,
    name: Option<LitStr>,
}

impl PrehniteDbFieldAttributes {
    pub fn skip_update(&self) -> bool {
        self.skip.value() || self.skip_update.value()
    }

    pub fn skip_create(&self) -> bool {
        self.skip.value() || self.skip_create.value()
    }

    pub fn use_id(&self) -> bool {
        self.use_id.value()
    }

    pub fn use_string_from(&self) -> bool {
        self.use_string_from.value()
    }
}

struct DbField {
    ty: Type,
    ident: Ident,
    attr: PrehniteDbFieldAttributes,
    field_name: String,
}

impl DbField {
    pub fn override_field_name(mut self) -> Self {
        self.attr.name.as_ref().map(|x| self.field_name = x.value());
        self
    }
}

fn parse_fields(data: DataStruct) -> Vec<DbField> {
    data.fields
        .into_iter()
        .map(|v| DbField {
            ty: v.ty,
            ident: v.ident.clone().unwrap(),
            attr: v
                .attrs
                .iter()
                .rfind(|v| v.path().is_ident("prehnite_db"))
                .and_then(|x| x.parse_args().ok())
                .unwrap_or_default(),
            field_name: v.ident.unwrap().to_string(),
        })
        .filter(|x| x.field_name != "id")
        .map(|x| x.override_field_name())
        .collect()
}

const MAX_BIND_COUNT: usize = 30000;

fn placeholder_helper(placeholder: impl AsRef<str>, count: usize) -> String {
    vec![placeholder.as_ref(); count].join(",")
}

#[proc_macro_derive(CreateRecord, attributes(prehnite_db))]
pub fn derive_c(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_ident = &input.ident;
    let attributes = PrehniteDbAttributes::parse(&input);
    let table_name = attributes.table_name(struct_ident.to_string());
    let view_name = attributes.view_name(struct_ident.to_string());

    if let Data::Struct(data) = input.data {
        let fields: Vec<DbField> = parse_fields(data)
            .into_iter()
            .filter(|x| !x.attr.skip_create())
            .collect();
        let placeholder_count = fields.len();
        let placeholder_in = placeholder_helper("?", placeholder_count);
        let placeholder = format!("({})", placeholder_in);
        let columns1 = fields.iter().map(|x| &x.field_name);
        let fields_normal = fields
            .iter()
            .filter(|x| !x.attr.use_id() && !x.attr.use_string_from());
        let fields_use_id = fields
            .iter()
            .filter(|x| !type_filter(&x.ty, "Option") && x.attr.use_id());
        let opt_fields_use_id = fields
            .iter()
            .filter(|x| type_filter(&x.ty, "Option") && x.attr.use_id());
        let fields_to_string = fields.iter().filter(|x| {
            !type_filter(&x.ty, "Option") && !x.attr.use_id() && x.attr.use_string_from()
        });
        let opt_fields_to_string = fields.iter().filter(|x| {
            type_filter(&x.ty, "Option") && !x.attr.use_id() && x.attr.use_string_from()
        });
        let fields_normal_ident = fields_normal.clone().map(|x| &x.ident);
        let fields_use_id_ident = fields_use_id.clone().map(|x| &x.ident);
        let opt_fields_use_id_ident = opt_fields_use_id.clone().map(|x| &x.ident);
        let fields_to_string_ident = fields_to_string.clone().map(|x| &x.ident);
        let opt_fields_to_string_ident = opt_fields_to_string.clone().map(|x| &x.ident);
        let columns2 = fields_normal
            .chain(
                fields_use_id
                    .chain(opt_fields_use_id.chain(fields_to_string.chain(opt_fields_to_string))),
            )
            .map(|x| x.field_name.clone())
            .collect::<Vec<String>>()
            .join(",");
        quote::quote! {
            impl #struct_ident {
                /// 値がSomeの場合に登録を実行する、[`Self::register()`]の糖衣関数
                pub async fn register_optional(val: std::option::Option<Self>, conn: &mut sqlx::SqliteConnection, is_on_conflict_do_nothing: bool) -> sqlx::Result<std::option::Option<Self>> {
                    std::result::Result::Ok(match val {
                        std::option::Option::Some(v) => std::option::Option::Some(v.register(conn, is_on_conflict_do_nothing).await?),
                        std::option::Option::None => None
                    })
                }

                /// 1つのレコードを登録するための、[`Self::register_many()`]の糖衣関数
                pub async fn register(&self, conn: &mut sqlx::SqliteConnection, is_on_conflict_do_nothing: bool) -> sqlx::Result<Self> {
                    let values = &Self::register_many(&std::vec![self.clone()], conn, is_on_conflict_do_nothing).await?;
                    if values.is_empty() {
                        std::result::Result::Err(sqlx::Error::RowNotFound)
                    } else {
                        std::result::Result::Ok(values[0].clone())
                    }
                }

                #[doc="複数レコードを一括で登録します。"]
                #[doc=std::concat!("1クエリあたり、[`", stringify!(#MAX_BIND_COUNT), "`]件の値が登録されます。")] // MAX_BIND_COUNT
                #[doc="# SQL"]
                #[doc="以下のクエリが実行されます。"]
                #[doc="- ..はプレースホルダの省略です。"]
                #[doc="- ON CONFLICT DO NOTHING はフラグが `true` の場合に有効化されます。"]
                #[doc="```sql"]
                #[doc=std::concat!("INSERT INTO ", #table_name, " (")]
                #(#[doc=concat!(#columns1, ",")])*
                #[doc=")"]
                #[doc="VALUES (..) [ON CONFLICT DO NOTHING] RETURNING id"]
                #[doc="```"]
                pub async fn register_many(values: &[Self], conn: &mut sqlx::SqliteConnection, is_on_conflict_do_nothing: bool) -> sqlx::Result<std::vec::Vec<Self>> {
                    if values.is_empty() {
                        return std::result::Result::Ok(std::vec![]);
                    }
                    let mut v = Vec::new();
                    for i in values.chunks(#MAX_BIND_COUNT / #placeholder_count) {
                        let sql = std::format!(
                            std::concat!(
                                "INSERT INTO ",
                                #table_name,
                                "(",
                                #columns2,
                                ") VALUES {} {} RETURNING id"
                            ),
                            std::vec![#placeholder; i.len()].join(","),
                            if is_on_conflict_do_nothing { "ON CONFLICT DO NOTHING" } else { "" }
                        );
                        let mut query = sqlx::query_as(sql.as_str());
                        for j in i {
                            query = query
                                #(.bind(j.#fields_normal_ident.clone()))*
                                #(.bind(j.#fields_use_id_ident.id as i64))*
                                #(.bind(j.#opt_fields_use_id_ident.as_ref().map(|x| x.id as i64)))*
                                #(.bind(String::from(j.#fields_to_string_ident.as_ref())))*
                                #(.bind(j.#opt_fields_to_string_ident.as_ref().map(|x| String::from(x))))*;
                        }
                        let id_list: Vec<crate::db::schema::ReturningId> = query.fetch_all(&mut *conn).await?;
                        let sql = format!(std::concat!("SELECT * FROM ", #view_name, " WHERE id IN ({})"), std::vec![#placeholder_in; i.len()].join(","));
                        let mut query = sqlx::query_as(sql.as_str());
                        for i in id_list {
                            query = query.bind(i.id);
                        }
                        v.extend(query.fetch_all(&mut *conn).await?);
                    }
                    std::result::Result::Ok(v)
                }
            }
        }.into()
    } else {
        Error::new_spanned(
            struct_ident,
            "This derive macro can only be used with `struct`.",
        )
        .to_compile_error()
        .into()
    }
}

#[proc_macro_derive(ReadRecord, attributes(prehnite_db))]
pub fn derive_r(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_ident = &input.ident;
    let attributes = PrehniteDbAttributes::parse(&input);
    let view_name = attributes.view_name(struct_ident.to_string());

    if let Data::Struct(_) = input.data {
        quote::quote! {
            impl #struct_ident {
                #[doc=std::concat!(#view_name, "からすべてのレコードを取得します。")]
                #[doc="# SQL"]
                #[doc="以下のクエリが実行されます。"]
                #[doc="```sql"]
                #[doc=std::concat!("SELECT * FROM ", #view_name)]
                #[doc="```"]
                pub async fn select_all(conn: &mut sqlx::SqliteConnection) ->  sqlx::Result<std::vec::Vec<Self>> {
                    sqlx::query_as(std::concat!("SELECT * FROM ", #view_name))
                        .fetch_all(conn).await
                }
                #[doc=concat!(#view_name, "から対応するidのレコードを取得します。")]
                #[doc="# SQL"]
                #[doc="以下のクエリが実行されます。"]
                #[doc="```sql"]
                #[doc=std::concat!("SELECT * FROM ", #view_name)]
                #[doc="WHERE id=?"]
                #[doc="```"]
                pub async fn from_id(conn: &mut sqlx::SqliteConnection, id: i64) -> sqlx::Result<std::option::Option<Self>> {
                    sqlx::query_as(std::concat!("SELECT * FROM ", #view_name, " WHERE id = ?"))
                    .bind(id)
                    .fetch_optional(conn)
                    .await
                }
            }
        }
            .into()
    } else {
        Error::new_spanned(
            struct_ident,
            "This derive macro can only be used with `struct`.",
        )
        .to_compile_error()
        .into()
    }
}

#[proc_macro_derive(UpdateRecord, attributes(prehnite_db))]
pub fn derive_u(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_ident = &input.ident;
    let attributes = PrehniteDbAttributes::parse(&input);
    let table_name = attributes.table_name(struct_ident.to_string());

    if let Data::Struct(data) = input.data {
        let fields: Vec<DbField> = parse_fields(data)
            .into_iter()
            .filter(|x| !x.attr.skip_update())
            .collect();
        let fields_normal = fields
            .iter()
            .filter(|x| !x.attr.use_id() && !x.attr.use_string_from());
        let fields_use_id = fields
            .iter()
            .filter(|x| !type_filter(&x.ty, "Option") && x.attr.use_id());
        let opt_fields_use_id = fields
            .iter()
            .filter(|x| type_filter(&x.ty, "Option") && x.attr.use_id());
        let fields_to_string = fields.iter().filter(|x| {
            !type_filter(&x.ty, "Option") && !x.attr.use_id() && x.attr.use_string_from()
        });
        let opt_fields_to_string = fields.iter().filter(|x| {
            type_filter(&x.ty, "Option") && !x.attr.use_id() && x.attr.use_string_from()
        });
        let fields_normal_ident = fields_normal.clone().map(|x| &x.ident);
        let fields_use_id_ident = fields_use_id.clone().map(|x| &x.ident);
        let opt_fields_use_id_ident = opt_fields_use_id.clone().map(|x| &x.ident);
        let fields_to_string_ident = fields_to_string.clone().map(|x| &x.ident);
        let opt_fields_to_string_ident = opt_fields_to_string.clone().map(|x| &x.ident);
        let update_set_clause: String = fields_normal
            .chain(
                fields_use_id
                    .chain(opt_fields_use_id.chain(fields_to_string.chain(opt_fields_to_string))),
            )
            .map(|x| format!("{}=?", x.field_name))
            .collect::<Vec<String>>()
            .join(",");

        quote::quote! {
            impl #struct_ident {
                #[doc=std::concat!("[`Self::id`]に対応するレコードの値を更新します。")]
                #[doc="# Panics"]
                #[doc=std::concat!("[`Self::id`]が`0`の場合、パニックを発生させます。")]
                #[doc="# SQL"]
                #[doc="以下のクエリが実行されます。"]
                #[doc="```sql"]
                #[doc=std::concat!("UPDATE ", #table_name)]
                #[doc=std::concat!("SET ", #update_set_clause)]
                #[doc="WHERE id=?"]
                #[doc="```"]
                #[tracing::instrument]
                pub async fn update(&self, conn: &mut sqlx::SqliteConnection) -> sqlx::Result<()> {
                    if self.id == 0 {
                        tracing::error!("Unexpected data is included!! Self: {:#?}", self);
                        std::panic!()
                    }
                    let mut query = sqlx::query(std::concat!(
                        "UPDATE ",
                        #table_name,
                        " SET ",
                        #update_set_clause,
                        " WHERE id=?"
                    ));
                    query
                    #(.bind(self.#fields_normal_ident.clone()))*
                    #(.bind(self.#fields_use_id_ident.id as i64))*
                    #(.bind(self.#opt_fields_use_id_ident.as_ref().map(|x| x.id as i64)))*
                    #(.bind(String::from(self.#fields_to_string_ident.as_ref())))*
                    #(.bind(self.#opt_fields_to_string_ident.as_ref().map(|x| String::from(x))))*
                    .bind(self.id)
                    .execute(conn)
                    .await?;
                    std::result::Result::Ok(())
                }
            }
        }
        .into()
    } else {
        Error::new_spanned(
            struct_ident,
            "This derive macro can only be used with `struct`.",
        )
        .to_compile_error()
        .into()
    }
}

#[proc_macro_derive(DeleteRecord, attributes(prehnite_db))]
pub fn derive_d(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_ident = &input.ident;
    let attributes = PrehniteDbAttributes::parse(&input);
    let table_name = attributes.table_name(struct_ident.to_string());

    if let Data::Struct(_) = input.data {
        quote::quote! {
            impl #struct_ident {
                #[doc=concat!("[`Self::id`]に対応するレコードを削除します。")]
                #[doc="# Panics"]
                #[doc=concat!("[`Self::id`]が`0`の場合、パニックを発生させます。")]
                #[doc="# SQL"]
                #[doc="以下のクエリが実行されます。"]
                #[doc="```sql"]
                #[doc=std::concat!("DELETE FROM ", #table_name)]
                #[doc="WHERE id=?"]
                #[doc="```"]
                #[tracing::instrument]
                pub async fn delete(self, conn: &mut sqlx::SqliteConnection) -> sqlx::Result<()> {
                    if self.id == 0 {
                        tracing::error!("Unexpected data is included!! Self: {:#?}", self);
                        std::panic!()
                    }
                    sqlx::query(std::concat!("DELETE FROM ", #table_name, " WHERE id=?"))
                        .bind(self.id)
                        .execute(conn)
                        .await?;
                    std::result::Result::Ok(())
                }
            }
        }
        .into()
    } else {
        Error::new_spanned(
            struct_ident,
            "This derive macro can only be used with `struct`.",
        )
        .to_compile_error()
        .into()
    }
}
