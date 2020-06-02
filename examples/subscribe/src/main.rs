use eventhub;

fn main() {
    let ev_client = eventhub::Eventhub::new("ws://eventhub.e24.no", "supersecret");
}
