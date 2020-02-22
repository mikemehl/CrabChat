pub mod CrabChatServer
{
    use std::sync::{Arc, Mutex, mpsc};
    use std::io::{Read, Write};
    use std::net::{TcpListener, TcpStream};
    use std::thread;

    enum ThreadMsg
    {
        NewConnection(Arc<Mutex<TcpStream>>),
        NewMessage(String),
    }

    pub fn start_server() -> std::io::Result<()>
    {
        let listener = TcpListener::bind("127.0.0.1:4098")?;

        // Spawn the write thread.
        let (tx, rx) : (mpsc::Sender<ThreadMsg>, mpsc::Receiver<ThreadMsg>) = mpsc::channel();
        thread::spawn(move || 
        {
            write_thread(rx);
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
                    let tx_clone = tx.clone();
                    add_client(stream, tx_clone);
                },
                Err(e) => { println!("ERROR CONNECTING"); }
            }
        }

        drop(listener);
        Ok(())
    }

    fn add_client(stream : TcpStream, 
                  tx : mpsc::Sender<ThreadMsg>)
    {
        println!("Added new client...");
        thread::spawn(move ||
        {
            // The read thread for this client.
            loop
            {
            }
        });
    }

    fn write_thread(rx : mpsc::Receiver<ThreadMsg>)
    {
        loop
        {
            let next_msg = rx.recv();
        }
    }
}

fn main() {
    println!("Hello, world!");
    CrabChatServer::start_server();
}
