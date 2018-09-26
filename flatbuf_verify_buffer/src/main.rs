extern crate flatbuffers;

extern {
    fn Ext_Handshake_PingMessage_verify_as_root(buf: *const ::std::os::raw::c_void, bufsiz: usize) -> ::std::os::raw::c_int;
}

// import the generated code
#[path = "./messages/ping_generated.rs"]
mod handshake_generated;
pub use handshake_generated::handshake::{get_root_as_ping_message, PingMessage, Version,
                                         PingMessageArgs};

use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;

fn main() {
    // Build up a serialized buffer algorithmically.
    // Initialize it with a capacity of 1024 bytes.
    let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(1024);

    // Create version
    let version: Version = Version::new(0, 1);

    // Create messages for the ping/pong
    let ping_message = builder.create_string("Ping");

    // Create ping message
    let ping_message= PingMessage::create(&mut builder, &PingMessageArgs{
        version: Some(&version),
        timestamp: 12345,
        message: Some(ping_message)
    });

    // Serialize the root of the object, without providing a file identifier
    builder.finish(ping_message, None);

    // Open the file to store the flat buffer
    let mut f = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("./ping_test.mon").unwrap();

    // Write the flat buffer to disk
    f.write(builder.finished_data()).unwrap();
    f.flush().unwrap();

    // Open the file
    let mut f = std::fs::File::open("./ping_test.mon").unwrap();
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).expect("file reading failed");

    // Verify the buffer
    let verification = unsafe { Ext_Handshake_PingMessage_verify_as_root(buf.as_ptr() as *const ::std::os::raw::c_void, buf.len()) };
    println!("Result of buffer verification {}", verification);

    // Get access to the root
    let ping_message = get_root_as_ping_message(&buf);

    // Get and test a field of the FlatBuffer's `struct`.
    assert!(ping_message.version().is_some());
    let version = ping_message.version().unwrap();
    let major = version.major();
    let minor = version.minor();
    assert_eq!(major, 0);
    assert_eq!(minor, 1);

    // Get and test timestamp field
    let timestamp = ping_message.timestamp();
    assert_eq!(timestamp, 12345);

    // Get and test message from the FlatBuffer
    assert_eq!(ping_message.message(), Some("Ping"));

    println!("The FlatBuffer was successfully created and accessed!");
}
