use serde::{Deserialize, Serialize};
use serde_json;


///strucyture of data returned by openai
#[derive(Serialize, Deserialize, Debug)]
struct MyData {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<Choice>,
    usage: Usage,
}

///struct of the CHOICE fields in my data
#[derive(Serialize, Deserialize, Debug)]
struct Choice {
    text: String,
    index: usize,
    logprobs: Option<serde_json::Value>,
    finish_reason: String,
}


///struct of the Usage field in my data
#[derive(Debug, Serialize, Deserialize)]
struct Usage {
    prompt_tokens: u64,
    completion_tokens: u64,
    total_tokens: u64,
}

///Deserianlizes the data into json like format and returns the text field
pub fn deserialize_into_my_data(json_str: &str) -> Vec<String> {
    // Using the serder_json lib convert the returned string from openai to MyData type
    let data: MyData = serde_json::from_str(json_str).unwrap();

    // Get the text field from the data and split on newlines for appealing look on cli
    let return_type = data.choices[0].text.split("\n");
    //convert to a vector of strings -- although not really neccessary but I choose that
    let new_type: Vec<String> = return_type.map(|f| f.to_string()).collect();
    //return the Vec of strings
    new_type
}

#[test]
fn test() {
    let json_str = r#"{"id":"cmpHq5d16vdnEhGXJFzukI","object":"text_completion","created":16854493,"model":"text-davinci-003","choices":[{"text":"\n\nOnce upon a time there lived an old man named David and his only daughter, Sarah. They lived in a small, humble cottage in the countryside.\n\nOne day, Sarah decided to venture into the nearby forest in search of adventure. When she reached the edge of the forest, a surprise awaited her: a beautiful pond filled with exotic fish. Excited, Sarah immediately began to explore, but soon noticed something unusual: at the bottom of the pond was a crystal-clear ball.\n\nShe reached in and grabbed it, and as soon as she touched it, the ball suddenly glowed brightly. When the light faded, she saw that a genie had appeared in a puff of smoke!\n\nThe Genie spoke with a commanding voice, \"I am here to grant you three wishes! Choose wisely- for there is no turning back once you have made your decision.\"\n\nSarah thought hard about what she wanted most in the world. After some time, she finally decided on her three wishes: financial security for her and her father, a mansion and a loyal puppy.\n\nThe Genie granted each of her wishes with a loud \"Poof!\", and Sarah returned to her home, astonished at how quickly her life had changed.\n\nMuch to her surprise, her father was already home and a lavish mansion had taken the place of their humble cottage! The Genie had also granted her wish for a loyal puppy, and she soon found herself playing with a fluffy golden retriever.\n\nThe old man, David, thanked the Genie for all his blessings and said, \"You have changed our lives for the better. Thank you!\"\n\nThe Genie smiled and said, \"My pleasure. Now, I must return to my own realm.\" Then, in a puff of smoke, he disappeared.\n\nSarah and David lived happily ever after in their newfound mansion, enriched by the genie's blessings.","index":0,"logprobs":null,"finish_reason":"stop"}],"usage":{"prompt_tokens":9,"completion_tokens":388,"total_tokens":397}}"#;

    let new = deserialize_into_my_data(json_str);

    new.iter().for_each(|f| println!("{f}"));
}


