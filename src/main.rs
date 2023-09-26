use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use local_ip_address::local_ip;
use std::net::TcpStream;
use std::io::Write;
use std::sync::{Arc, Mutex};

// Define a global server address using an Arc and Mutex for thread safety
lazy_static::lazy_static! {
    static ref SERVER_ADDRESS: Arc<Mutex<String>> = Arc::new(Mutex::new("127.0.0.1:1234".to_owned()));
}

// Function to send a command to the server
fn send_command(command: &str) -> Result<(), std::io::Error> {
    let server_address = SERVER_ADDRESS.lock().unwrap();
    let mut stream = TcpStream::connect(&*server_address)?;
    println!("Sending command {} to server {}", command, server_address);
    let mut string_buffer = String::from(command);
    string_buffer.push('\n');
    stream.write_all(string_buffer.as_bytes())?;
    Ok(())
}

// Handler for the main web page
async fn index(_req: HttpRequest) -> HttpResponse {
    let server_address = SERVER_ADDRESS.lock().unwrap();
    let html = include_str!("index.html")
        .replace("{{server_address}}", &*server_address);
    HttpResponse::Ok().body(html)
}

// Handler to send a command to the server
async fn send_command_handler(info: web::Path<String>) -> HttpResponse {
    if let Err(err) = send_command(&info) {
        eprintln!("Error sending command: {:?}", err);
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().body(format!("Command sent to server: {}", info))
}

// Handler to update the server address
async fn update_server_address(new_address: web::Json<String>) -> HttpResponse {
    let mut server_address = SERVER_ADDRESS.lock().unwrap();
    *server_address = new_address.into_inner();
    HttpResponse::Ok().body(format!("Server address updated to: {}", *server_address))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!(
        "Scoreboard Plus Remote v{} started",
        env!("CARGO_PKG_VERSION")
    );
    let my_local_ip = local_ip().unwrap();
    println!("Please connect to webinterface on 127.0.0.1:5000 or {}:5000", my_local_ip);
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/command/{info}").route(web::get().to(send_command_handler)))
            .service(web::resource("/update_address").route(web::post().to(update_server_address)))
    })
    .bind("0.0.0.0:5000")?
    .run()
    .await
}