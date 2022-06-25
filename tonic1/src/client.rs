use tonic::Request;
use records::recorder_client::RecorderClient;
use records::RecordRequest;

pub mod records{
    tonic::include_proto!("records");
}


#[tokio::main]
async fn main() ->Result<(),Box<dyn std::error::Error>>{
    let mut client =RecorderClient::connect("http://[::1]:50050").await?;
    let request=Request::new(
        RecordRequest{
            user_name: "huang".to_string(),
            user_age:30,
        }
    );
    let response =client.send_message(request).await?;
    println!("{:#?}", response);

    Ok(())

}


