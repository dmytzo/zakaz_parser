extern crate chrono;

use zakaz::Store;

use std::{thread, time};
use chrono::Local;

fn main() {
    let stores = vec![
        Store{
            name: "Metro",
            url: "https://stores-api.zakaz.ua/stores/48215632/delivery_schedule/plan/?coords=49.971629,36.1759104"
        }, 
        Store{
            name: "Tavria",
            url: "https://stores-api.zakaz.ua/stores/48221130/delivery_schedule/plan/?coords=49.971629,36.1759104"
        }
    ];

    let timer = time::Duration::from_secs(30);

    loop {
        println!("{}\n", Local::now().format("%Y-%m-%d %H:%M:%S"));
        for store in &stores {
            store.find_open_positions();
        }
        thread::sleep(timer);
    }
    
}