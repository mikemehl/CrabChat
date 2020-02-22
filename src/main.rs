pub mod CrabChatServer
{
    use std::sync::{Arc, Mutex, mpsc};
    use std::io::{Read, Write};
    use std::net::{TcpListener, TcpStream};
    use std::thread;

    pub type ClientListArc = Arc<Mutex<Vec<TcpStream>>>;

    pub fn start_server() -> std::io::Result<()>
    {
        let listener = TcpListener::bind("127.0.0.1:4098")?;

        // Alright, create out client list. Dynamically allocate and wrap it up in a mutex and
        // Atomic Reference Count (Arc) so that we can allow multiple threads to use it (but not
        // simultaneously). This looks like the way things are done in Rust.
        let client_list : ClientListArc = Arc::new(Mutex::new(Vec::new()));

        // Spawn the write thread.
        let (tx, rx) : (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();
        let client_clone = client_list.clone();
        thread::spawn(move || 
        {
            write_thread(rx, client_clone);
        });

        for mut stream in listener.incoming() 
        {
            match stream
            {
                Ok(stream) =>
                {
                    println!("Connecting!!!");

                    // Remember, calling functions on variables moves ownership!
                    // So, clone the things we don't want to lose ownership of.
                    let client_clone = client_list.clone();
                    let tx_clone = tx.clone();
                    add_client(stream, client_clone, tx_clone);
                },
                Err(e) => { println!("ERROR CONNECTING"); }
            }
        }

        drop(listener);
        Ok(())
    }

    fn add_client(stream : TcpStream, 
                  client_list : ClientListArc, 
                  tx : mpsc::Sender<String>)
    {
        println!("Added new client...");
        thread::spawn(move ||
        {
            // The read thread for this client.
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
