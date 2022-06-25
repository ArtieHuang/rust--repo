use std::process::id;
use reqwest::Client;
use serde::{Deserialize, Serialize};
//use reqwest structure

#[derive(Debug,Deserialize,Serialize)]

struct Todo{  //预制结构体，解构json
    id:u64,
    text:String,
    completed:bool,
}

#[tokio::main]
async fn main() ->Result<(),Box<dyn std::error::Error>> {

    let client:Client =reqwest::Client::new();

    let create_posts=Todo {
        id:14,
        text:"afsfa13123".to_string(),
        completed: true
    };

    //结构体方式发送post
    let posts_post:Todo =client
        .post("http://localhost:3030/todos")
        .json(&create_posts)
        .send()
        .await?
        .json()
        .await?;


    println!("{:#?}",posts_post);


    //get三种方式
    //json array
    let post_get_array:Vec<Todo>=client
        .get("http://localhost:3030/todos")
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}",post_get_array);
    println!("{}",post_get_array.len());



    // single json
    let post_get_one :Todo=client
        .get("http://localhost:3030/todos?offset=3&limit=5")
        .send()
        .await?
        .json()
        .await?;

    println!("{:?}",post_get_one);

    Ok(())
}




//get request
//同步请求
// let mut resp:Response=reqwest::blocking::get("http://jsonplaceholder.typicode.com/post/1")?;
// println!("{:?}",resp.headers());
// let post_one:String =resp.text()?;
// println!("{:?}",post_one);
// Ok(())

