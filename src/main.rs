#[macro_use]
extern crate diesel;
#[macro_use]
extern crate tower_web;

use tower_web::extract::{Context, Extract, Immediate};
use tower_web::middleware::deflate::DeflateMiddleware;
use tower_web::util::BufStream;
use tower_web::ServiceBuilder;

use flate2::Compression;
use std::str;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

use models::{Expense, NewExpense};
use schema::expenses::dsl::expenses;

#[derive(Clone, Debug)]
struct HelloWorld;

#[derive(Debug, Extract, Response)]
struct MyData {
    foo: usize,
    bar: Option<String>,
}

/// This will be the JSON response
#[derive(Response)]
struct HelloResponse {
    message: &'static str,
}

impl_web! {
    impl HelloWorld {
        #[get("/")]
        fn hello_world(&self) -> Result<String, ()> {
            Ok("Hello world".to_string())
        }

        #[get("/hello/:name")]
        fn greet(&self, name: String) -> Result<String, ()> {
            Ok(format!("Hello, {}", name))
        }

        #[post("/data")]
        fn data(&self, body: MyData) -> Result<String, ()> {
            Ok(format!("Hello, {:?}", body))
        }

        #[post("/return-data")]
        #[content_type("json")]
        fn return_data(&self, body: MyData) -> Result<MyData, ()> {
            Ok(body)
        }

        #[get("/json")]
        #[content_type("json")]
        fn hello_world2(&self) -> Result<HelloResponse, ()> {
            Ok(HelloResponse {
                message: "hello world",
            })
        }

        #[get("/vec")]
        #[content_type("json")]
        fn get_vec(&self) -> Result<Vec<usize>, ()> {
            let mut vec = Vec::new();
            vec.push(1);
            vec.push(5);

            Ok(vec)
        }

        #[post("/request-body")]
        #[content_type("plain")]
        fn request_body(&self, body: Vec<u8>, param: MyParam) -> Result<String, ()> {
            Ok(format!("{} We received: \n{}", param.bar, str::from_utf8(&body).unwrap()))
        }

        #[post("/request-body-length")]
        fn request_body_length(&self, body: Vec<u8>) -> Result<String, ()> {
            Ok(format!("We received {} bytes", body.len()))
        }

        #[get("/expenses")]
        #[content_type("json")]
        fn get_expenses(&self, param: MyParam) -> Result<Vec<Expense>, ()> {
            let results = expenses
                // .limit(1)
                .load::<Expense>(&param.connection)
                .expect("Error loading expenses");

            Ok(results)
        }

        #[post("/expenses")]
        #[content_type("json")]
        fn post_expenses(&self, param: MyParam, body: Vec<u8>) -> Result<Expense, ()> {
            let json_string:&str = str::from_utf8(&body).unwrap();
            let expense: NewExpense = serde_json::from_str(json_string).unwrap();
            diesel::insert_into(schema::expenses::table)
                        .values(&expense)
                        .execute(&param.connection)
                        .expect("Error saving expense");

            use schema::expenses::dsl::id;
            let inserted_expense:Expense = schema::expenses::table.order(id.desc()).first(&param.connection).unwrap();
            Ok(inserted_expense)
        }
    }
}

struct MyConfig {
    foo: String,
}
struct MyParam {
    bar: String,
    connection: diesel::SqliteConnection,
}

impl<B: BufStream> Extract<B> for MyParam {
    type Future = Immediate<MyParam>;

    fn extract(context: &Context) -> Self::Future {
        let config = context.config::<MyConfig>().unwrap();

        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in env file");
        let conn = SqliteConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

        let expense = NewExpense {
            name: "Expense 10",
            amount: 18.20,
        };

        // diesel::insert_into(schema::expenses::table)
        //     .values(&expense)
        //     .execute(&conn)
        //     .expect("Error saving expense");

        let param = MyParam {
            bar: config.foo.clone(),
            connection: conn,
        };
        Immediate::ok(param)
    }
}

pub fn main() {
    let addr = "127.0.0.1:8080".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    ServiceBuilder::new()
        .resource(HelloWorld)
        .config(MyConfig {
            foo: "bar".to_owned(),
        })
        .middleware(DeflateMiddleware::new(Compression::best()))
        .run(&addr)
        .unwrap();
}
