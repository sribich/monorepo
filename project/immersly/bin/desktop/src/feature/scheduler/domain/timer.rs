use chrono::DateTime;
use chrono::Days;
use chrono::Local;
use chrono::NaiveTime;
use chrono::TimeDelta;
use chrono::Timelike;

#[derive(Clone, Debug)]
pub struct Timer {
    // system_timezone
    // client_timezone
    now: DateTime<Local>,
    schedule_limit: DateTime<Local>,
    tomorrow: DateTime<Local>,
}

impl Timer {
    pub fn new() -> Self {
        let now = chrono::Local::now();
        let is_already_next_day = now.hour() < 4;

        let schedule_limit = now
            .checked_sub_days(Days::new(u64::from(is_already_next_day)))
            .unwrap()
            .with_time(NaiveTime::from_hms_opt(22, 0, 0).unwrap())
            .unwrap();
        let tomorrow = now
            .checked_add_days(Days::new(u64::from(!is_already_next_day)))
            .unwrap()
            .with_time(NaiveTime::from_hms_opt(4, 0, 0).unwrap())
            .unwrap();

        Self {
            now,
            schedule_limit,
            tomorrow,
        }
    }

    pub fn schedule(&self, days: u32) -> u64 {
        let days = days - 1;

        (self
            .tomorrow
            .checked_add_signed(TimeDelta::days(i64::from(days)))
            .unwrap()
            .timestamp()
            + rand::random_range(0..60)) as u64
    }

    pub fn schedule_intraday(&self, mins: u32) -> u64 {
        if self.can_schedule_intraday(mins) {
            return self
                .now
                .checked_add_signed(TimeDelta::minutes(i64::from(mins)))
                .unwrap()
                .timestamp() as u64;
        }

        // The card needs to be scheduled tomorrow
        (self.tomorrow.timestamp() + rand::random_range(0..60)) as u64
    }

    pub fn is_interval_valid(&self, seconds: u64) -> bool {
        let next = self
            .now
            .checked_add_signed(TimeDelta::seconds(seconds as i64))
            .unwrap();

        next < self.tomorrow
    }

    pub fn can_schedule_intraday(&self, mins: u32) -> bool {
        let schedule_time = self
            .now
            .checked_add_signed(TimeDelta::minutes(i64::from(mins)))
            .unwrap();

        schedule_time < self.schedule_limit && mins <= 240
    }
}
