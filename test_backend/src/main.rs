fn main() {
    use time::OffsetDateTime;
    let utc_time = OffsetDateTime::now_utc();
    let utc_time2 = OffsetDateTime::now_local();
    println!("Current UTC time: {}", utc_time);
    println!("Current time: {:?}", utc_time2);
}
