use chrono::Utc;

#[derive(Debug, Deserialize, Serialize)]
pub struct TimeStamp {
    created: i64,
    updated: i64,
}

impl Default for TimeStamp {
    fn default() -> Self {
        let utc = Utc::now().timestamp_millis();
        Self {
            created: utc,
            updated: utc,
        }
    }
}

impl TimeStamp {
    pub fn new(f: fn() -> i64) -> Self {
        let time = f();
        Self {
            created: time,
            updated: time,
        }
    }
    pub fn get_created(&self) -> i64 { self.created }
    pub fn get_updated(&self) -> i64 { self.updated }
    pub fn update(&mut self) {
        self.updated = Utc::now().timestamp_millis();
    }
}