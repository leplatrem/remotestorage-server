use actix::prelude::*;
use actix_web::*;
use diesel;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use models;
use schema;

pub struct DbExecutor(pub Pool<ConnectionManager<SqliteConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

pub struct GetDocument {
    pub name: String,
    pub folder: String,
}

impl Message for GetDocument {
    type Result = Result<models::Document, Error>;
}

impl Handler<GetDocument> for DbExecutor {
    type Result = Result<models::Document, Error>;

    fn handle(&mut self, msg: GetDocument, _: &mut Self::Context) -> Self::Result {
        use self::schema::documents::dsl::*;

        let conn: &SqliteConnection = &self.0.get().unwrap();

        let mut items = documents
            .filter(name.eq(&msg.name))
            .filter(folder.eq(&msg.folder))
            .load::<models::Document>(conn)
            .map_err(|_| error::ErrorInternalServerError("Error loading document"))?;

        match items.pop() {
            Some(v) => Ok(v),
            None => Err(error::ErrorNotFound("Unknown document")),
        }
    }
}

pub struct UpdateDocument {
    pub name: String,
    pub folder: String,
}

impl Message for UpdateDocument {
    type Result = Result<models::Document, Error>;
}

impl Handler<UpdateDocument> for DbExecutor {
    type Result = Result<models::Document, Error>;

    fn handle(&mut self, msg: UpdateDocument, _: &mut Self::Context) -> Self::Result {
        use self::schema::documents::dsl::*;

        let new_document = models::NewDocument {
            name: &msg.name,
            folder: &msg.folder,
        };

        let conn: &SqliteConnection = &self.0.get().unwrap();

        diesel::insert_into(documents)
            .values(&new_document)
            .execute(conn)
            .map_err(|_| error::ErrorInternalServerError("Error inserting document"))?;

        let mut items = documents
            .filter(name.eq(&msg.name))
            .filter(folder.eq(&msg.folder))
            .load::<models::Document>(conn)
            .map_err(|_| error::ErrorInternalServerError("Error loading document"))?;

        Ok(items.pop().unwrap())
    }
}

pub struct ListDocuments {
    pub folder: String,
}

impl Message for ListDocuments {
    type Result = Result<Vec<models::Document>, Error>;
}

impl Handler<ListDocuments> for DbExecutor {
    type Result = Result<Vec<models::Document>, Error>;

    fn handle(&mut self, msg: ListDocuments, _: &mut Self::Context) -> Self::Result {
        use self::schema::documents::dsl::*;

        let conn: &SqliteConnection = &self.0.get().unwrap();

        let items = documents
            .filter(folder.eq(&msg.folder))
            .load::<models::Document>(conn)
            .map_err(|_| error::ErrorInternalServerError("Error listing documents"))?;

        Ok(items)
    }
}
