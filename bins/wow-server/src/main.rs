use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

use pow_common::{Md5PuzzleTask, Md5PuzzleSolution, PowRequest2};
use pow_common::PowResponse1;
use pow_common::PowResponse2;
use pow_common::PowResponseErr;

use rand::Rng;

use std::convert::TryInto;
use std::error::Error;
use std::net::SocketAddr;

use wow_common::{GetWisdomQuoteRequest, GetWisdomQuoteResponse};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = std::env::args().nth(1).unwrap_or("127.0.0.1:7878".to_string());
    let listener = TcpListener::bind(addr).await?;

    while let Ok((tcp_stream, socket_addr)) = listener.accept().await {
        println!("new client: {:?}", socket_addr);
        tokio::spawn(async move {
            if let Err(err) = handle_connection(tcp_stream, socket_addr).await {
                println!("failed to handle response: {}", err)
            }
        });
    }

    Ok(())
}

async fn handle_connection(mut stream: TcpStream, _socket_addr: SocketAddr) -> Result<(), Box<dyn Error>> {
    let mut buffer_1 = [0; 1024];
    // TODO: handle timeout
    stream.read(&mut buffer_1).await?;
    let request_1: bincode::Result<GetWisdomQuoteRequest> = bincode::deserialize(&buffer_1);
    // TODO: use socket_addr for question
    let question = "17296541".as_bytes();
    // TODO: enable POW based on concurrent requests being processed
    let pow_enabled = rand::thread_rng().gen_bool(1.0 / 2.0);


    let response_1: PowResponse1<GetWisdomQuoteResponse, Md5PuzzleTask> = match request_1 {
        // TODO: save rq for later use on step 2 if needed
        Ok(_rq) => {
            println!("got I1");
            if pow_enabled {
                let hint: [u8; 6] = question[2..].try_into()?;
                let hash: [u8; 16] = md5::compute(question).into();
                let puzzle = Md5PuzzleTask {
                    hint,
                    hash: hash.into(),
                };
                PowResponse1::R1(puzzle)
            } else {
                let wow_response = GetWisdomQuoteResponse {
                    quote: "NO POW QUOTE".to_string(),
                };
                PowResponse1::R2(wow_response)
            }
        }
        Err(err) => {
            println!("failed to deserialize request: {}", err);
            PowResponse1::Err(PowResponseErr::BadRequest(format!("{:?}", err)))
        }
    };

    let serialized_response_1 = bincode::serialize(&response_1)?;
    stream.write(&serialized_response_1).await?;
    stream.flush().await?;

    if let PowResponse1::R1(_) = response_1 {
        let mut buffer_2 = [0; 1024];
        stream.read(&mut buffer_2).await?;
        let request_2: bincode::Result<PowRequest2<Md5PuzzleSolution>> = bincode::deserialize(&buffer_2);

        let response_2: PowResponse2<GetWisdomQuoteResponse> = match request_2 {
            Ok(request) => {
                println!("got I2");
                if &request.puzzle_solution.answer == question {
                    let wow_response = GetWisdomQuoteResponse {
                        quote: "The best way out is always through.".to_string(),
                    };
                    PowResponse2::Ok(wow_response)
                } else {
                    PowResponse2::Err(PowResponseErr::WrongPuzzle)
                }
            }
            Err(err) => {
                println!("failed to deserialize request: {}", err);
                PowResponse2::Err(PowResponseErr::BadRequest(format!("{:?}", err)))
            }
        };

        let serialized_response_2 = bincode::serialize(&response_2)?;
        stream.write(&serialized_response_2).await?;
        stream.flush().await?;
    }

    Ok(())
}