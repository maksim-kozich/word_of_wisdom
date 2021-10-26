use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use std::error::Error;
use std::convert::TryInto;

use pow_common::{Md5PuzzleTask, Md5PuzzleSolution};
use pow_common::PowRequest1;
use pow_common::PowRequest2;
use pow_common::PowResponse1;
use pow_common::PowResponse2;

use wow_common::{GetWisdomQuoteRequest, GetWisdomQuoteResponse};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = std::env::args().nth(1).unwrap_or("127.0.0.1:7878".to_string());
    let mut stream = TcpStream::connect(addr).await?;
    let (mut stream_read, mut stream_write) = stream.split();

    let request_1 = PowRequest1 {
        request: GetWisdomQuoteRequest {
            temp: "temp".to_string()
        },
    };
    let serialized_request_1 = bincode::serialize(&request_1)?;
    stream_write.write(&serialized_request_1).await?;
    stream_write.flush().await?;

    let mut buffer = [0; 1024];
    stream_read.read(&mut buffer).await?;

    let response_1: PowResponse1<GetWisdomQuoteResponse, Md5PuzzleTask> = bincode::deserialize(&buffer)?;

    let wow_response = match response_1 {
        PowResponse1::R1(puzzle) => {
            let puzzle_solution = solve_task(puzzle)
                .expect("unable to brute force task");
            let request_2 = PowRequest2 {
                puzzle_solution
            };
            let serialized_request_2 = bincode::serialize(&request_2)?;
            stream_write.write(&serialized_request_2).await?;
            stream_write.flush().await?;

            let mut buffer = [0; 1024];
            stream_read.read(&mut buffer).await?;

            let response_2: PowResponse2<GetWisdomQuoteResponse> = bincode::deserialize(&buffer)?;

            match response_2 {
                PowResponse2::Ok(rs) => {
                    Ok(rs)
                }
                PowResponse2::Err(err) => {
                    Err(err)
                }
            }
        }
        PowResponse1::R2(rs) => {
            println!("(pow off)");
            Ok(rs)
        }
        PowResponse1::Err(err) => {
            Err(err)
        }
    };

    match wow_response {
        Ok(rs) => {
            println!("Quote: '{}'", rs.quote);
        }
        Err(err) => {
            eprintln!("Get Quote Failed: {:?}", err);
        }
    }
    Ok(())
}

fn solve_task(task: Md5PuzzleTask) -> Option<Md5PuzzleSolution> {
    println!("start solving task: {:x?}, {:x?}", task.hint, &task.hash);
    let mut result = None;
    for i in 0..u16::MAX {
        let mask: [u8; 2] = i.to_le_bytes();

        let mut attempt: Vec<u8> = vec![];
        attempt.extend_from_slice(&mask);
        attempt.extend_from_slice(&task.hint);

        let hash: [u8; 16] = md5::compute(&attempt).into();
        if &hash == &task.hash {
            let answer = attempt.try_into().expect("failed to convert vec to array");
            println!("task solved: {:x?}", answer);
            let solution = Md5PuzzleSolution {
                answer
            };
            result = Some(solution)
        }
    }
    result
}