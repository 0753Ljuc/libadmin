use chrono::{DateTime, Duration, Utc};

pub struct Borrow {
    // order id
    pub oid: u32,
    // book id
    pub bid: u32,
    // borrow date
    pub o_date: DateTime<Utc>,
    // the maximum loan time.
    pub max_loan: Duration,
    // the maximum loan count.
    pub max_count: u8,
}
