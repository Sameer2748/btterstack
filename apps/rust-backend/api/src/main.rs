use std::sync::{Arc, Mutex};
use poem::{post, Route, Server, get, handler, listener::TcpListener, EndpointExt, web::{Path, Json, Data}};

use crate::req_inputs::{CreateWebsiteRequest, SignUpUserInput,SignInUserInput};
use crate::req_outputs::{CreateWebsiteResponse,GetWebsiteResponse, SignUpUserOutput,SignInUserOutput};
use store::store::Store;

pub mod req_inputs;
pub mod req_outputs;


#[handler]
fn getwebsite(Path(id): Path<String>, Data(s): Data<&Arc<Mutex<Store>>>) -> Json<GetWebsiteResponse> {
    // store the url in the store library 
    let mut locked_s = s.lock().unwrap();
    let website = locked_s
        .get_website(id)
        .unwrap();
    Json(GetWebsiteResponse{url: website.url})

}
#[handler]
fn createwebsite(Json(data): Json<CreateWebsiteRequest>, Data(s): Data<&Arc<Mutex<Store>>>) -> Json<CreateWebsiteResponse> {
    let url: String = data.url;
    // store the url in the store library 
    let mut locked_s = s.lock().unwrap();
    let website = locked_s
        .create_website(String::from("dbf7faa7-f823-4887-9109-7dec0afe0f62"), url)
        .expect("Failed to persist website");

    let response: CreateWebsiteResponse = CreateWebsiteResponse {
        id: website.id
    };

    Json(response)
}

#[handler]
fn signupuser(Json(data): Json<SignUpUserInput>,  Data(s): Data<&Arc<Mutex<Store>>>) -> Json<SignUpUserOutput>{
    let name = data.name;
    let password = data.password;
    let mut locked_s = s.lock().unwrap();
    let id = locked_s
        .signup_user(name, password).expect("Failed to persist website");

    let response = SignUpUserOutput{
        id: id
    };
    Json(response)
}
#[handler]
fn signinuser(Json(data): Json<SignInUserInput>,  Data(s): Data<&Arc<Mutex<Store>>>) -> Json<SignInUserOutput>{
    let name = data.name;
    let password = data.password;
    let mut locked_s = s.lock().unwrap();
    let _exists = locked_s.signin_user(name.clone(), password).unwrap();

    let response = SignInUserOutput{
        jwt: String::from("sameer")
    };
    Json(response)
}

// flabor - multi-thread run the program in multithreading and current_thread will run it like javaascript single threadly 
#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), std::io::Error> {
    // sindle instance of db connection  and pass it to app so evey route cna use one connection but different vairant will be created for every thread so wont run connection limit 
    let s = Arc::new(Mutex::new(Store::default().unwrap()));
    let app = Route::new()
    .at("/status/:websiteId", get(getwebsite))
    .at("/website", post(createwebsite))
    .at("/user/signup", post(signupuser))
    .at("/user/signin", post(signinuser))
    .data(s);

    Server::new(TcpListener::bind("0.0.0.0:3002"))
        .run(app)
        .await
}