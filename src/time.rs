use chrono::{DateTime, TimeZone, Utc};

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub struct TimeStamp {
    #[serde(default)]
    created: i64,
    #[serde(default)]
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

    pub fn timezone_offset<Tz: TimeZone>(timestamp: i64, tz: &Tz) -> DateTime<Tz> {
        let ndt = chrono::NaiveDateTime::from_timestamp(timestamp / 1000, 0);
        let datetime: DateTime<Utc> = DateTime::from_utc(ndt, Utc);
        datetime.with_timezone(tz)
    }

    pub fn get_created(&self) -> i64 {
        self.created
    }
    pub fn get_updated(&self) -> i64 {
        self.updated
    }
    pub fn update(&mut self) {
        self.updated = Utc::now().timestamp_millis();
    }
}
