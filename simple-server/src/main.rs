use simple_server::{Server, StatusCode};


fn main() {
   let servers = Server::new(|request , mut response|{
       println!("request {} {}",request.method(),request.uri());

       match(request.uri().path(),request.method().as_str()){
           ("/","GET") =>{
               Ok(response.body(String::from("Hello from get request").into_bytes())?)
           },
           ("/form","POST") => {
               Ok(response.body(String::from("Hello from post request ").into_bytes())?)
           },
           (_,_) => Ok(response.body(String::from("invalid reuqest").into_bytes())?)
       }
   });
   servers.listen("127.0.0.1","5000");
}
