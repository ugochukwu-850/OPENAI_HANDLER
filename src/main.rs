use std::env;

use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};

use tokio;

use chat::deserialize_into_my_data;

///generates the and prints the requested data from OPENAI
async fn generate_text(prompt: &str) -> Result<String, reqwest::Error> {
    //get OPENAI key as environment variable
    let auth_token = env::var("OPENAI_KEY").expect("Openai key");

    //init a data json like structure of the request data
    let mut data: String = r#"
    {
        "model": "text-davinci-003",
        "prompt": "{}",
        "temperature": 0.9,
        "max_tokens": 1024
    }"#
    .to_string();

    //replace the prompt with the prompt text
    data = format! {"{}", data.replace("{}", prompt)};

    //print setup details for confirmation
    //println!("{}, authentication: {}", data, auth_token);

    //format a bearer token string
    let bearer_auth = format!("Bearer {}", auth_token);

    //init the open ai request url to a var
    let url = "https://api.openai.com/v1/completions".to_string();

    //init a new reqwest client
    let client = reqwest::Client::new();

    //make the reqwest
    let response = client
        //the url to post to
        .post(url)
        //the header details
        .header(ACCEPT, "*/*")
        .header(AUTHORIZATION, &bearer_auth)
        .header(CONTENT_TYPE, "application/json")
        //the data initialized earlier
        .body(data)
        //make the request with send
        .send()
        //await the future data
        .await
        //unwrap the ok value
        .unwrap();
    //match the response status code to ensure the reqwest was successful
    match response.status() {
        reqwest::StatusCode::OK => {
            //on ok value get the test field
            match response.text().await {
                Ok(parsed) => {
                    println!("🔥 Success!");

                    let new = r#"{}"#;
                    let new = new.replace("{}", &parsed);
                    //parse it into myData structure and save as parsed var
                    let parsed = deserialize_into_my_data(&new);
                    //iter through the parsed var::Vec of string and print
                    parsed.iter().for_each(|f| println!("{f}"));
                    //confirmation string
                    return Ok("THE WORK IS DONE".to_string());
                }
                //if there was an error message instead of an Ok value
                Err(_) => println!("Hm, the response didn't match the shape we expected."),
            };
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            //if your unauthorized likely because you did not set up you environment variable
            println!("Status: UNAUTHORIZED - Need to grab a new token");
        }
        reqwest::StatusCode::TOO_MANY_REQUESTS => {
            //and too many request
            println!("Status: 429 - Too many requests");
        }
        other => {
            panic!("Uh oh! Something unexpected happened: [{:#?}]", other);
        }
    };
    Ok("OK NOT FOUND".to_string())
}

///custome function to take input from command line
async fn input(prompt: &str) -> String {
    let mut output = String::new();
    println!("{prompt}\n>>>  ");
    std::io::stdin().read_line(&mut output).expect("erroocred");
    output.trim().to_string()
}

#[tokio::main]
async fn main() {
    let prompt = input("Please input a topic.").await;

    match generate_text(&prompt).await {
        Ok(text) => println!("Response Message: {}", text),
        Err(err) => eprintln!("Error: {}", err),
    }
}
