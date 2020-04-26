use simple_server::{Server, StatusCode};


fn main() {
   let servers = Server::new(|request , mut response|{
       println!("request {} {}",request.method(),request.uri());
      Ok(response.body(String::from("hello ").into_bytes())?)
   });
   servers.listen("127.0.0.1","5000");
}
