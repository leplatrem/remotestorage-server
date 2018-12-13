#![allow(proc_macro_derive_resolution_fallback)]

use super::schema::documents;

#[derive(Serialize, Queryable)]
pub struct Document {
    pub name: String,
    pub folder: String,
}

#[derive(Insertable)]
#[table_name = "documents"]
pub struct NewDocument<'a> {
    pub name: &'a str,
    pub folder: &'a str,
}
