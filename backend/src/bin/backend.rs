use rocket_cors::{AllowedHeaders, AllowedOrigins};

#[macro_use]
extern crate rocket;

#[get("/hello")]
fn hello_get() -> String {
    "Hello, World!".into()
}

#[launch]
fn rocket() -> _ {
    let allowed_origins = AllowedOrigins::all();

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    rocket::build().mount("/", routes![hello_get]).attach(cors)
}
