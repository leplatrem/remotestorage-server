use actix::prelude::Addr;
use actix_web::{AsyncResponder, FutureResponse, HttpResponse, Path, State};
use db::{DbExecutor, GetDocument, ListDocuments, UpdateDocument};
use futures::Future;

/// State with DbExecutor address
pub struct AppState {
    pub db: Addr<DbExecutor>,
}

#[derive(Deserialize)]
pub struct DocumentPath {
    folder: String,
    name: String,
}

pub fn get_documents(
    (folder, state): (Path<String>, State<AppState>),
) -> FutureResponse<HttpResponse> {
    println!("List folder: {}", &folder);
    state
        .db
        .send(ListDocuments {
            folder: folder.into_inner(),
        })
        .from_err()
        .and_then(|res| match res {
            Ok(documents) => Ok(HttpResponse::Ok().json(documents)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

pub fn get_document(
    (info, state): (Path<DocumentPath>, State<AppState>),
) -> FutureResponse<HttpResponse> {
    println!("Get document: {}/{}", info.folder, info.name);
    state
        .db
        .send(GetDocument {
            name: info.name.to_string(),
            folder: info.folder.to_string(),
        })
        .from_err()
        .and_then(|res| match res {
            Ok(document) => Ok(HttpResponse::Ok().json(document)),
            Err(_) => Ok(HttpResponse::NotFound().body("")),
        })
        .responder()
}

pub fn update_document(
    (info, state): (Path<DocumentPath>, State<AppState>),
) -> FutureResponse<HttpResponse> {
    println!("Store document: {}/{}", info.folder, info.name);
    state
        .db
        .send(UpdateDocument {
            name: info.name.to_string(),
            folder: info.folder.to_string(),
        })
        .from_err()
        .and_then(|res| match res {
            Ok(document) => Ok(HttpResponse::Ok().json(document)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}
