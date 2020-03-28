use serde_derive::Deserialize;

#[derive(Copy, Clone)]
pub struct Store<'a> {
    pub name: &'a str,
    pub url: &'a str,
}

impl <'a>Store<'_> {
    fn fetch_data(&self) -> Result<Vec<DayInfo>, reqwest::Error> {
        let resp = reqwest::blocking::get(self.url)?;
        print!("{}: {}\n", self.name, resp.status());
    
        let data = resp.json::<Vec<DayInfo>>();
        return data
    }
        
    fn process_data(self, data: Vec<DayInfo>) {
        let mut no_position = true;
        for day_info in data {
            for hour_info in day_info.items {
                if hour_info.is_open {
                    if no_position {
                        no_position = false
                    }
                    print!("Open position: {} {}\n", hour_info.time_range, hour_info.date);
                }
            }
        }
        if no_position {
            print!("No open positions\n\n");
        }
    }
    
    pub fn find_open_positions(self) {
        let data = self.fetch_data();
        self.process_data(data.unwrap());
    }
}

#[derive(Deserialize, Debug)]
pub struct DayInfo {
    date: String,
    items: Vec<HourInfo>
}

#[derive(Deserialize, Debug)]
pub struct HourInfo {
    id: String,
    end_ordering_time: f64,
    time_range: String,
    price: f64,
    currency: String,
    is_open: bool,
    date: String
}
