use simple_server::{Server, StatusCode};


fn main() {
   let servers = Server::new(|request , mut response|{

        let body = String::from_utf8(request.body().to_vec()).unwrap();
       println!("request {} {} body {}",request.method(),request.uri(),body);

       match(request.uri().path(),request.method().as_str()){
           ("/","GET") =>{
               Ok(response.body(String::from("Hello from get request").into_bytes())?)
           },
           ("/form","POST") => {
               Ok(response.body(body.into_bytes())?)
           },
           (_,_) => {
               response.status(StatusCode::NOT_FOUND);
               Ok(response.body(String::from("invalid reuqest").into_bytes())?)
            }
       }
   });
   servers.listen("127.0.0.1","5000");
}
