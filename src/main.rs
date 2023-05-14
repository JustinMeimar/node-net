use serde::{Serialize, Deserialize};
use serde_json::Deserializer;
use std::io::{BufReader, BufWriter, Write, Stdout};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message<T> {
    src: String, 
    dest: String,
    body: Body<T>, 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Body<T> {
    msg_id: Option<usize>,
    #[serde(flatten)]
    payload: T
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Init(InitData),
    InitOk,
    Echo(EchoData),
    EchoOk,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EchoData {
    echo: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct InitData {
    node_id: String,
    node_ids: Vec<String>
}

struct InitNode {
    id: usize
}

trait Available<T> {
    fn respond(&self, msg: &Message<T>, stdout: &mut BufWriter<Stdout>) -> std::io::Result<()>;
}

impl Available<Payload> for InitNode {
    fn respond(&self, msg: &Message<Payload>, stdout: &mut BufWriter<Stdout>) -> std::io::Result<()> {

        let reply : Message<Payload> = Message {
            src: msg.dest.clone(),
            dest: msg.src.clone(),
            body: Body {
                msg_id: Some(self.id),
                payload: Payload::InitOk
            }
        };

        let reply_str = serde_json::to_string(&reply);

        match reply_str {
            Ok(s) => {
                stdout.write_all(s.as_bytes())?;
                stdout.flush()?;
            }
            Err(e) => {
                println!("Error in writting the reply to stdout!\n, {:?}", e);
            }
        }
        // match msg.body.payload 

        Ok(())
    }
}

fn main() -> std::io::Result<()> {

    let stdout = std::io::stdout();
    let mut writer = BufWriter::new(stdout);
    let stdin = std::io::stdin(); 
    let reader = BufReader::new(stdin);
    let stream = Deserializer::from_reader(reader).into_iter::<Message<Payload>>();

    let init_node = InitNode {
        id : 1
    };

    for msg in stream {
        match msg {
            Ok(m) => {
                match m.body.payload {
                    Payload::Init(_) => {
                        init_node.respond(&m, &mut writer)?;
                    },
                    Payload::InitOk => {
                        println!("Hello, InitOk")
                    },
                    Payload::Echo(_) => {
                        init_node.respond(&m, &mut writer)?;
                    },
                    Payload::EchoOk => {
                        println!("Hello, EchoOk")
                    }
                }     
            } 
            Err(e) => {
                println!("Deserialization Error! {:?}", e);
            }
        }
    }

    let m : Message<Payload> = Message {
        src: "source".to_string(),
        dest: "destination".to_string(),
        body: Body {
            msg_id: Some(1),
            payload: Payload::Init(
                InitData {
                    node_id: "n1".to_string(),
                    node_ids: vec!["n1".to_string(), "n2".to_string()],
                }
            ),
        },
    };

    let s = serde_json::to_string_pretty(&m)?;

    // println!("{}", s);

    Ok(())
}