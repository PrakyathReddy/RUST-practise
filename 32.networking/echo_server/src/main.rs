use std::io::{Read, Result, Write};

fn main() -> Result<()> {
    let listener = std::net::TcpListener::bind("0.0.0.0.12345")?;

    // this single threaded server can handle only one incoming
    // connection at a time
    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut buffer = [0u8; 4096];
        let count = stream.read(&mut buffer)?;
        stream.write_all(&buffer[0..count])?;
    }

    Ok(())
}

/*

This is a simple example of a single-threaded TCP echo server written in Rust. Let's break it down:

1. `use std::io::{Read, Result, Write};`: This line imports the `Read`, `Result`, and `Write` traits from the `std::io` module. These traits provide methods for reading from and writing to byte streams.

2. `fn main() -> Result<()>`: This is the entry point to the program. The `Result<()>` return type indicates that the function will either return `Ok(())` if it runs successfully, or it will return an `Err` that encapsulates information about what went wrong.

3. `let listener = std::net::TcpListener::bind("0.0.0.0:12345")?;`: This line creates a new TCP listener that binds to the specified IP address and port. The `?` operator is used to handle any errors that may occur during the bind operation. If an error occurs, the error will be returned from the function immediately, and the subsequent code will not be executed.

4. `for stream in listener.incoming()`: This starts a loop that iterates over the incoming TCP connections to the server. For each incoming connection, the loop will execute the body of the loop.

5. `let mut stream = stream?;`: This line attempts to accept the incoming connection. If the connection is successfully accepted, it becomes a readable and writable TCP stream. If an error occurs during the accept operation, the error is returned from the function immediately.

6. `let mut buffer = [0u8; 4096];`: This line declares a buffer of 4096 bytes that will be used to read data from the TCP stream.

7. `let count = stream.read(&mut buffer)?;`: This line reads data from the TCP stream into the buffer. The number of bytes read is stored in `count`.

8. `stream.write_all(&buffer[0..count])?;`: This line writes the data that was just read back to the TCP stream, essentially echoing the data back to the client.

9. `Ok(())`: If the function has not exited early due to an error, it returns `Ok(())` to indicate that it has run successfully.

Please note that because this is a single-threaded server, it can only handle one connection at a time. If a second client attempts to connect while the server is busy with the first client, the second client will have to wait until the server is free. This might not be the most efficient way to handle multiple clients, and a multi-threaded or async server would be better suited for handling multiple concurrent clients.

 */
