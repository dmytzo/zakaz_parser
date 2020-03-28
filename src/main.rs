use zakaz::Store;

use std::{thread, time, env, convert::Infallible};

use carapax::{
    handler, Api, Config, Dispatcher, ExecuteError,
    longpoll::LongPoll, 
    methods::SendMessage,
    webhook::run_server,
    types::{Command, Message},
};
use dotenv::dotenv;

#[handler(command = "/start")]
async fn start_handler(api: &Api, command: Command) -> Result<(), ExecuteError> {
    let chat_id = command.get_message().get_chat_id();

    let stores = vec![
        Store{
            name: "Metro".to_string(),
            url: "https://stores-api.zakaz.ua/stores/48215632/delivery_schedule/plan/?coords=49.971629,36.1759104".to_string()
        }, 
        Store{
            name: "Tavria".to_string(),
            url: "https://stores-api.zakaz.ua/stores/48221130/delivery_schedule/plan/?coords=49.971629,36.1759104".to_string()
        }
    ];

    let timer = time::Duration::from_secs(60);
    loop {
        for store in &stores {
            let results = store.find_open_positions().await;
            if results.len() > 0 {
                let method = SendMessage::new(chat_id, format!("{}: {:?}", store.name, results));
                api.execute(method).await?;
            }
            println!("{:?}", results);
        }
        thread::sleep(timer);
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("TOKEN").expect("TOKEN is not set");
    let config = Config::new(token);
    let api = Api::new(config).expect("Failed to create API");
    let mut dispatcher = Dispatcher::new(api.clone());
    dispatcher.add_handler(start_handler);
    LongPoll::new(api, dispatcher).run().await
}
