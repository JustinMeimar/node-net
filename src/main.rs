use serde::{Serialize, Deserialize};
use serde_json::Deserializer;
use std::io::BufReader;

struct Node {
    id: usize
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message {
    src: String, 
    dst: String,
    body: Body, 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Body {
    Echo(EchoBody),
    Broadcast(BroadcastBody),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EchoBody {
    #[serde(rename="type")]
    ty: String,
    msg: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BroadcastBody {
    #[serde(rename="type")]
    ty: String,
    msg: String, 
    timeout: String
}

fn main() {

    let stdin = std::io::stdin();
    let reader = BufReader::new(stdin);
    let stream = Deserializer::from_reader(reader).into_iter::<Message>();
    
    for msg in stream {
        match msg {
            Ok(m) => {
                println!("Deserialization Success! {}", m.src.to_string());
            } 
            Err(e) => {
                println!("Deserialization Error! {:?}", e);
            }
        }
    }

}