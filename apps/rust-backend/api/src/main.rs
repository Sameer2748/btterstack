use poem::{IntoResponse, post, Route, Server, get, handler, listener::TcpListener, web::Path};

#[handler]
fn getwebsite(Path((websiteId, city)): Path<(String, String)>) -> String {
    format!("hello: websiteId={}, city={}", websiteId, city)
}
#[handler]
fn createwebsite(Path(city): Path<String>) -> String {
    format!("hello: city={}", city)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
    .at("/status/:websiteId/:city", get(getwebsite))
    .at("/website/:city", post(createwebsite));
    Server::new(TcpListener::bind("0.0.0.0:3002"))
        .run(app)
        .await
}