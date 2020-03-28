use serde_derive::Deserialize;

#[derive(Copy, Clone)]
pub struct Store<'a> {
    pub name: &'a str,
    pub url: &'a str,
}

impl <'a>Store<'_> {
    async fn fetch_data(&self) -> Result<Vec<DayInfo>, reqwest::Error> {
        let resp = reqwest::get(self.url).await?;
        print!("{}: {}\n", self.name, resp.status());
    
        let data = resp.json::<Vec<DayInfo>>().await;
        return data
    }
        
    fn process_data(self, data: Vec<DayInfo>) -> Vec<OpenPosition> {
        let mut results = Vec::new();
        for day_info in data {
            for hour_info in day_info.items {
                if hour_info.is_open {
                    let open_position = OpenPosition{
                        time_range: hour_info.time_range, 
                        date: hour_info.date
                    };
                    results.push(open_position);
                }
            }
        }
        results
    }
    
    pub async fn find_open_positions(self) -> Vec<OpenPosition> {
        let data = self.fetch_data().await;
        return self.process_data(data.unwrap());
    }
}

#[derive(Debug)]
pub struct OpenPosition {
    time_range: String,
    date: String
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
