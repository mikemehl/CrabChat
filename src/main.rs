
pub mod CrabChatServer
{

    pub fn start_server() -> std::io::Result<()>
    {
        let listener = std::net::TcpListener::bind("127.0.0.1:4098")?;

        for stream in listener.incoming() 
        {
            match stream
            {
                Ok(stream) =>
                {
                    println!("Connecting!!!");
                    std::thread::spawn(move || 
                    {
                        handle_client(stream);
                    });
                },
                Err(e) => { println!("ERROR CONNECTING"); }
            }
        }

        drop(listener);
        Ok(())
    }

    fn handle_client(stream : std::net::TcpStream)
    {
        println!("YO WE GOT A CONNECTION!");
    }

}

fn main() {
    println!("Hello, world!");
    CrabChatServer::start_server();
}
