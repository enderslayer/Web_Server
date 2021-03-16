use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::Read;
use std::str;
use std::string::String;
use std::io::prelude::*;
use std::path::PathBuf;
use std::path::Path;
use std::env;
use std::fs;
use std::sync::{Arc, Mutex};
use std::fs::File;


fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("localhost:8888")?;
    let request = Arc::new(Mutex::new(0));
    let valid = Arc::new(Mutex::new(0));


    // accept connections and process them serially
    for stream in listener.incoming() {

        let stream = stream?;
        let request = request.clone();
        let valid = valid.clone();

        thread::spawn(move || {
            handle_client(stream,valid);

            let mut request = request.lock().unwrap();
            *request += 1;
            println!("Number of Requests: {:?}",request);
            // println!("Number a Valid requests: {:?}",&valid);
        });


    }
    Ok(())
}

fn handle_client(mut stream: TcpStream, valid: Arc<Mutex<u64>>) {
    println!("{:?}",stream.peer_addr().unwrap());
     let mut buffer = [0; 500];
     let mut message = String::from("");
     let cur_dir = env::current_dir().unwrap();
     println!("{:?}",cur_dir);



     loop {
         stream.read(&mut buffer[..]).unwrap();
         let m = std::str::from_utf8(&buffer[..]);

         message.push_str(m.unwrap());


         println!("{}",message);
         if message.ends_with("\u{0}"){
             break;
         }
         if message.ends_with("\r\n\r\n"){
             break;
         }
         if message.get(message.len()-1..message.len()).unwrap() == "\n\n"{
             break;
         }
     }
     let temp_vec :Vec<String> = message.splitn(2,"/").map(|x| x.to_string()).collect();
     let filename = temp_vec[1].splitn(2, " ").next().unwrap();
     let size = message.len();


     let file = PathBuf::from(filename);
     // match file {
     //     Ok() => {
             let abs = fs::canonicalize(&file).unwrap();
             let check = abs.to_str().unwrap();

             let main = cur_dir.to_str().unwrap();


             if check.contains(main) {
                 let mut open = File::open(abs).unwrap();
                 let mut stuff  = String::new();
                 open.read_to_string(&mut stuff);

                 let mut valid = valid.lock().unwrap();
                 *valid += 1;
                 println!("Number of Valid Requests: {:?}", valid);

                 stream.write(stuff.as_bytes());

             } else {
                 stream.write(error(size).as_bytes());
             }
     //     },
     //     Err(error) => println!("error")
     // };



     stream.write(reply(size,filename).as_bytes());
     stream.flush().unwrap()

}

 fn reply(size: usize,file: &str) -> String{
     let response = format!("HTTP/1.1 200 OK \r
 Content-Type: text/html; charset=UTF-8 \r
 Content-Length: {} \r\n\r
 <html> \r
 <body> \r
 <h1>Message received</h1> \r
 Requested file:{}<br> \r
 </body> \r
 </html>\r\n",size,file);
     return response;
 }
fn error(len: usize) -> String{
    let response = format!("HTTP/1.1 200 OK \r
 Content-Type: text/html; charset=UTF-8 \r
 Content-Length: {} \r\n\r
 <html> \r
 <body> \r
 <h1> HTTP/1.1 403 Forbidden </h1> \r
  \r
 </body> \r
 </html>\r\n",len);
    return response
}