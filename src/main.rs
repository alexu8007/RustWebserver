use rouille::{Response, Server, router};
use std::fs::File;
use std::io::Read;




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

                /* 
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

                let file_path = format!("C:\\Users\\Alex\\firstProgram\\{}.txt", param); 

                println!("Attempting to open file: {}", file_path);
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