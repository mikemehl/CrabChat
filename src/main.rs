pub mod CrabChatServer
{
    use std::sync::{Arc, Mutex};
    use std::net::{TcpListener, TcpStream};
    use std::thread;

    pub fn start_server() -> std::io::Result<()>
    {
        let listener = TcpListener::bind("127.0.0.1:4098")?;

        // Alright, create out client list. Dynamically allocate and wrap it up in a mutex and
        // Atomic Reference Count (Arc) so that we can allow multiple threads to use it (but not
        // simultaneously). This looks like the way things are done in Rust.
        let client_list : Arc<Mutex<Vec<std::net::TcpStream>>> = Arc::new(Mutex::new(Vec::new()));

        for stream in listener.incoming() 
        {
            match stream
            {
                Ok(stream) =>
                {
                    println!("Connecting!!!");

                    // Remember, calling functions on variables moves ownership!
                    // So, we need to copy the reference to be used below.
                    add_client(stream, Arc::clone(&client_list));
                },
                Err(e) => { println!("ERROR CONNECTING"); }
            }
        }

        drop(listener);
        Ok(())
    }

    fn add_client(mut stream : TcpStream, client_list : Arc<Mutex<Vec<TcpStream>>>)
    {
        println!("WE GOT EM");
        client_list.lock().unwrap().push(stream);
        thread::spawn(||
        {
            let mut data = [0 as u8; 4096];
            while match stream.read(&mut data)
            {
                Ok(msg) => 
                {
                    println!("Got the message ok...");
                    true
                },
                Err(e) => 
                {
                    println!("Something has gone horribly wrong!!");
                    false
                }
            } {} // } ends match scope, {} is actually the while loop body.
        });
    }

}

fn main() {
    println!("Hello, world!");
    CrabChatServer::start_server();
}
