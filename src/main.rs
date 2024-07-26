use rouille::{Response, Server, router};
use std::fs::File;
use std::io::Read;

pub fn main(){

    let server = Server::new("localhost:9091", move |request| {
    router!(request,
        (GET) (/) => {
            Response::text("hello world")
        },
        (GET) (/newEndpoint) => {
            Response::text("different endpoint")
        },
        (GET) (/download) => {
            let file_path = "C:\\Users\\Alex\\firstProgram\\Cargo.toml"; 
            println!("Attempting to open file: {}", file_path);
            match File::open(file_path) {
                Ok(mut file) => {
                    let mut file_content = Vec::new();
                    match file.read_to_end(&mut file_content) {
                        Ok(_) => {
                            println!("File read successfully");
                            Response::from_data("application/octet-stream", file_content)
                                .with_additional_header("Content-Disposition", "attachment; filename=\"Cargo.toml\"")
                        },
                        Err(e) => {
                            println!("Failed to read file: {}", e);
                            Response::empty_404()
                        }
                    }
                },
                Err(e) => {
                    println!("Failed to open file: {}", e);
                    Response::empty_404()
                }
            }
        },
        _ => {
            Response::empty_404()
        }
    )
    
}).unwrap();

println!("Listening on {:?}", server.server_addr());

server.run();
}