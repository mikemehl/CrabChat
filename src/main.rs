pub mod CrabChatServer
{
    use std::sync::{Arc, Mutex, mpsc};
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

        // Spawn the write thread.
        let (tx, rx) : (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();
        thread::spawn(move || 
        {
            write_thread(rx, client_list.clone());
        });

        for mut stream in listener.incoming() 
        {
            match stream
            {
                Ok(stream) =>
                {
                    println!("Connecting!!!");

                    // Remember, calling functions on variables moves ownership!
                    // So, we need to copy the reference to be used below.
                    add_client(stream, Arc::clone(&client_list), tx.clone());
                },
                Err(e) => { println!("ERROR CONNECTING"); }
            }
        }

        drop(listener);
        Ok(())
    }

    fn add_client(mut stream : TcpStream, client_list : ClientListArc, tx : mpsc::Sender<String>)
    {
        println!("Added new client...");
        thread::spawn(move ||
        {
            // The read thread for this client.
            let mut data = [0 as u8; 4096];
            while match read_stream.lock().unwrap().read(&mut data)
            {
                Ok(msg) => 
                {
                    let msg = String::from_utf8(data.to_vec()).expect("INVALID MESSAGE");
                    println!("Msg received: {}", msg);
                    tx.send(msg);
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

    fn write_thread(rx : mpsc::Receiver<String>, client_list : ClientListArc)
    {
        loop
        {
            let next_msg = rx.recv().unwrap();
        }
    }
}

fn main() {
    println!("Hello, world!");
    CrabChatServer::start_server();
}
