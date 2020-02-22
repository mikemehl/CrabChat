pub mod CrabChatServer
{
    use std::sync::{Arc, Mutex};
    use std::io::{Read, Write};
    use std::net::{TcpListener, TcpStream};
    use std::thread;

    pub type StreamArc = Arc<Mutex<TcpStream>>;
    pub type ClientListArc = Arc<Mutex<Vec<StreamArc>>>;
    pub type OutMsgQArc = Arc<Mutex<Vec<String>>>;

    pub fn start_server() -> std::io::Result<()>
    {
        let listener = TcpListener::bind("127.0.0.1:4098")?;

        // Alright, create out client list. Dynamically allocate and wrap it up in a mutex and
        // Atomic Reference Count (Arc) so that we can allow multiple threads to use it (but not
        // simultaneously). This looks like the way things are done in Rust.
        let client_list : ClientListArc = Arc::new(Mutex::new(Vec::new()));
        let msg_q : OutMsgQArc = Arc::new(Mutex::new(Vec::new()));

        for mut stream in listener.incoming() 
        {
            match stream
            {
                Ok(stream) =>
                {
                    println!("Connecting!!!");

                    // Remember, calling functions on variables moves ownership!
                    // So, we need to copy the reference to be used below.
                    add_client(stream, Arc::clone(&client_list), Arc::clone(&msg_q));
                },
                Err(e) => { println!("ERROR CONNECTING"); }
            }
        }

        drop(listener);
        Ok(())
    }

    fn add_client(mut stream : TcpStream, client_list : ClientListArc, msg_q : OutMsgQArc)
    {
        println!("WE GOT EM");
        // The stream needs to be accessed from multiple threads still (UGHHHH). 
        // So, let's follow the pattern lol.
        let read_stream = Arc::new(Mutex::new(stream));
        client_list.lock().unwrap().push(Arc::clone(&read_stream));
        thread::spawn(move ||
        {
            // The read thread for this client.
            let mut data = [0 as u8; 4096];
            while match read_stream.lock().unwrap().read(&mut data)
            {
                Ok(msg) => 
                {
                    println!("Got the message ok...");
                    msg_q.lock().unwrap().push("MESSAGE".to_string());
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

    fn write_thread()
    {
        // Loops forever trying to send any collected outgoing messages.
    }

}

fn main() {
    println!("Hello, world!");
    CrabChatServer::start_server();
}
