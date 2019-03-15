#[macro_use]
extern crate tower_web;

use tower_web::middleware::deflate::DeflateMiddleware;
use tower_web::ServiceBuilder;

use flate2::Compression;

#[derive(Clone, Debug)]
struct HelloWorld;

#[derive(Debug, Extract)]
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
    }
}

pub fn main() {
    let addr = "127.0.0.1:8080".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    ServiceBuilder::new()
        .resource(HelloWorld)
        .middleware(DeflateMiddleware::new(Compression::best()))
        .run(&addr)
        .unwrap();
}
