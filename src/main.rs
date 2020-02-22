pub mod CrabChatServer
{
    use std::sync::{Arc, Mutex, mpsc};
    use std::io::{Read, Write, ErrorKind};
    use std::net::{TcpListener, TcpStream};
    use std::thread;

    enum ThreadMsg
    {
        NewConnection(mpsc::Sender<ThreadMsg>),
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

        for stream in listener.incoming() 
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

    fn add_client(mut stream : TcpStream, 
                  tx_write : mpsc::Sender<ThreadMsg>)
    {
        println!("Added new client...");
        thread::spawn(move ||
        {
            // The read thread for this client.
            // Setup a communication channel with the write thread.
            let (tx, rx) : (mpsc::Sender<ThreadMsg>, mpsc::Receiver<ThreadMsg>) = mpsc::channel();
            let new_connection_msg = ThreadMsg::NewConnection(tx);
            tx_write.send(new_connection_msg);

            // Now, we start looping.
            loop
            {
                // First, check if we've received anything to echo from the write thread.
                let write_thread_msg = rx.try_recv();
                match(write_thread_msg)
                {
                    Ok(msg) =>
                    {
                        // TODO: Send the message to the client! 
                    },
                    Err(e) => { println!("ERROR RECEIVING MESSAGE FROM WRITE THREAD"); }
                }

                // Then, check for any incoming messages from the socket.
                let mut data_buf = [0u8; 1024];
                let socket_msg = stream.read_to_end(&mut data_buf.to_vec());
                match(socket_msg)
                {
                    Ok(msg) =>
                    {
                        // TODO: Send the message to the write thread!   
                    },
                    Err(ref e) if e.kind() == ErrorKind::WouldBlock =>
                    {
                        // TODO: How to handle this case? 
                    }
                    Err(e) => { println!("SOMETHING EVIL"); }
                }
            }
        });
    }

    fn write_thread(rx : mpsc::Receiver<ThreadMsg>)
    {
        let mut channel_list : Vec<mpsc::Sender<ThreadMsg>> = Vec::new();
        loop
        {
            let next_msg = rx.recv();
            match(next_msg)
            {
                Ok(msg) =>
                {
                    // Handle the message by type.
                    match(msg)
                    {
                        ThreadMsg::NewConnection(tx_channel) => { channel_list.push(tx_channel); },
                        ThreadMsg::NewMessage(msg) =>
                        {
                            // TODO: Send the message to all threads.
                        }
                    }
                },
                Err(e) => { println!("Error receiving message!!!"); }
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    CrabChatServer::start_server();
}
