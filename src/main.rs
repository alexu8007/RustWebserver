/* 
/    Copyright: 2024 A.U.G. Signals Ltd.
/    File: src/main.rs
/    Description: This file is the entry point for the application. It sets up and runs the HTTP server.
/    Created by: Alex Ungureanu
*/

use rouille::{Response, Server, router};
use std::fs::{self, File};
use std::path::Path;
use std::io::Read;

/// Handles an HTTP request by reading the content of a specified file or directory.
/// 
/// This function attempts to open the provided `file_path`. If the path points to a directory,
/// it reads all the files within the directory, concatenates their contents, and returns the
/// combined content as the response. If the path points to a file, it reads the file's content
/// and returns it as the response. If any errors occur during these operations, an appropriate
/// 404 response is returned.
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the path to the file or directory to be read.
///
/// # Returns
///
/// * `Response` - An HTTP response containing the content of the file or directory, or a 404
///   response if an error occurs.
///
/// # Errors
///
/// This function will return a 404 response in the following cases:
/// * The provided path does not exist.
/// * The provided path is not accessible due to permissions.
/// * Any file within the directory cannot be read.
/// * The content of any file cannot be converted to a UTF-8 string.
///
/// # Examples
///
/// ```
// let response = handle_request("/path/to/file_or_directory");
/// ```
pub fn handle_request(file_path: &str) -> Response {
    println!("Attempting to open path: {}", file_path);
    let path = Path::new(file_path);

    if path.is_dir() {
        let mut combined_content = String::new();
        match fs::read_dir(path) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            let path = entry.path();
                            if path.is_file() {
                                match File::open(&path) {
                                    Ok(mut file) => {
                                        let mut file_content = Vec::new();
                                        match file.read_to_end(&mut file_content) {
                                            Ok(_) => {
                                                match String::from_utf8(file_content) {
                                                    Ok(file_string) => {
                                                        combined_content.push_str(&file_string);
                                                        combined_content.push('\n');
                                                    },
                                                    Err(e) => {
                                                        println!("Failed to convert file content to string: {}", e);
                                                    }
                                                }
                                            },
                                            Err(e) => {
                                                println!("Failed to read file: {}", e);
                                            }
                                        }
                                    },
                                    Err(e) => {
                                        println!("Failed to open file: {}", e);
                                    }
                                }
                            }
                        },
                        Err(e) => {
                            println!("Failed to read directory entry: {}", e);
                        }
                    }
                }
                Response::text(combined_content)
            },
            Err(e) => {
                println!("Failed to read directory: {}", e);
                Response::empty_404()
            }
        }
    } else {
        match File::open(file_path) {
            Ok(mut file) => {
                let mut file_content = Vec::new();
                match file.read_to_end(&mut file_content) {
                    Ok(_) => {
                        println!("File read successfully");
                        match String::from_utf8(file_content) {
                            Ok(file_string) => {
                                Response::text(file_string)
                            },
                            Err(e) => {
                                println!("Failed to convert file content to string: {}", e);
                                Response::empty_404()
                            }
                        }
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
    }
}

/// The main function that sets up and runs the HTTP server.
///
/// This function initializes a server listening on `IP_ADDRESS` and defines several routes
/// to handle different HTTP requests. The routes include:
///
/// * `GET /` - Returns a default text response.
/// * `GET /newEndpoint` - Returns a different text response.
/// * `GET /download` - Reads a file specified by a query parameter and returns its content.
/// * `POST /upload_creds` - Reads the request body and returns it as a text response.
///
/// The `/download` route constructs a file path based on a query parameter and calls the
/// `handle_request` function to read and return the file content. If the file does not exist,
/// a 404 response is returned.
///
/// The server logs its listening address and runs indefinitely.
///
pub fn main() {
    let server = Server::new("localhost:9091", move |request| {
        router!(request,
            (GET) (/) => {
                Response::text("default endpoint")
            },
            (GET) (/newEndpoint) => {
                Response::text("different endpoint")
            },
            (GET) (/download) => {

                let param = request.get_param("param").unwrap_or_else(|| "default".to_string());

                let file_path = format!("C:\\Users\\Alex\\firstProgram\\{}", param); 

                /* 

                UNCOMMENT TO ALLOW KEYCLOAK PACKAGE VERIFICATION
                ---> CHANGE FROM GET TO POST <---

                let data = Request::body().expect("Failed to read request body");

                keycloak = parse(&data).unwrap().expect("Failed to parse JSON");

                if (keycloak.authenticated){
                    let client_id = keycloak["subject"];
                    Response::text("You are authenticated")
                    file_path = (format!*(C:\\Users\\Alex\\firstProgram\\certificates\\{}", client_id));
                    if (file_path.exists()){
                        continue
                    } else {
                        Response::empty_404()
                    }
                }
                */

                handle_request(&file_path)
            },
            (POST) (/upload_creds) => {
                let mut data = request.data().expect("Failed to read request body");
                let mut body = String::new();
                data.read_to_string(&mut body).expect("Failed to read body to string");
                Response::text(format!("Received body: {}", body))
            },
            _ => {
                Response::empty_404()
            }
        )
    }).unwrap();

    println!("Listening on {:?}", server.server_addr());

    server.run();
}