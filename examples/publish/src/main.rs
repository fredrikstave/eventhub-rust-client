extern crate eventhub;

fn main() {
    let evClient = eventhub::Eventhub("ws://eventhub.e24.no", "myAuthToken");

    match evClient.connect() {
      Some(conn) => conn.publish("examples/rust", "This is a test message!"),
      None => println!("Could not establish connection"),
    }
}