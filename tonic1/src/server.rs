
use tonic::{Request, Response, Status};
use tonic::transport::Server;
use records::recorder_server::{Recorder, RecorderServer};
use records::{RecordRequest,RecordResponse};



pub mod records{
    tonic::include_proto!("records");
}

#[derive (Debug,Default)]
pub struct RecorderService {
}

#[tonic::async_trait]
impl Recorder for RecorderService {
    async fn send_message(
        &self,
        request: Request<RecordRequest>
    )->Result<Response<RecordResponse>,Status>{
        println!("Request{:#?}",request);

        let req =request.into_inner();

        let response =RecordResponse{
            successful:true,
            message:format!("user {} is {} years old ",req.user_name,req.user_age).into()
        };
        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() ->Result<(),Box<dyn std::error::Error>>{
    let addr="[::1]:50050".parse()?;
    let recorder:RecorderService=RecorderService::default();
    println!("Recorder listening on {}!",addr);

    Server::builder()
        .add_service(RecorderServer::new(recorder))
        .serve(addr)
        .await?;
    Ok(())
}
