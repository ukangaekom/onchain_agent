mod agent_process;
mod tools;

// use customer_agent::customer_process;
use agent_process::{process};
use tools::{get_price, parse_input, extract_output};

use frankenstein::GetUpdatesParams;
use frankenstein::ReplyParameters;
use frankenstein::SendMessageParams;
use frankenstein::TelegramApi;
use frankenstein::{Api, UpdateContent};

use std::env;
use std::collections::HashMap;


fn main() {
    dotenv_flow::dotenv_flow().ok();
    let token: String = env::var("TELEGRAM_BOT_TOKEN").expect("Telegram bot token must be set");

    
    let api = Api::new(&token);

    let update_params_builder = GetUpdatesParams::builder();
    let mut update_params = update_params_builder.clone().build();


    loop {
        let result = api.get_updates(&update_params);

        println!("result: {result:?}");

        // Matching results from the api call: This result is either Ok() or Error()
        match result {
            Ok(response) => {

                // Looping throught all the object in the response body
                for update in response.result {

                    // Getting the message object from looped parameters
                    if let UpdateContent::Message(message) = update.content {

                        if let Some(text) = message.text.clone(){
                            println!("Text:{}",text);

                            let routing_response = process(&text);

                            let mut agent_response:String = String::from("");

                            let output = extract_output(&routing_response.unwrap());
                            match output {

                                output if output.clone().expect("REASON").contains("get_price") => {

                                    if let Some((command, coin)) = parse_input(&output.unwrap()){

                                        println!("The valid command has been triggered{}, {}!",&command, &coin);
                                        
                                        agent_response = format!("The price of {} is ${}",&coin,get_price(&coin));
                                        // agent_response = format!("Price of {:?} is {:?}", &coin, get_price(&prices)["USD"]);
                                        // agent_response = format!("{}",&prices);
                                        println!("{:?}", &command);
                                    }

                                },


                                _ => {
                                    agent_response = format!("{}", &output.unwrap());
                                }

                                


                            }

                            



                           


                            // Abstracting the sent message instance for future reply
                        let reply_parameters = ReplyParameters::builder()
                            .message_id(message.message_id)
                            .build();
                       

                       

                        let send_message_params = SendMessageParams::builder()
                            .chat_id(message.chat.id)
                            .text(&agent_response)
                            .reply_parameters(reply_parameters)
                            .build();


                        // let send_message_params = SendMessageParams::builder()
                        //     .chat_id(message.chat.id)
                        //     .text(text)
                        //     .reply_parameters(reply_parameters)
                        //     .build();



                        if let Err(err) = api.send_message(&send_message_params) {
                            println!("Failed to send message: {err:?}");
                        }
                            
                            
                        }





                        

                        
                    }
                    update_params = update_params_builder
                        .clone()
                        .offset(update.update_id + 1)
                        .build();
                }
            },
            Err(error) => {
                println!("Failed to get updates: {error:?}");
            }
        }
    }


    

}
