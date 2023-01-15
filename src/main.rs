use salah::prelude::*;
use chrono::{ Utc, Date };

pub struct PrayerParameters {
    pub date: Date<Utc>,
    pub location: Coordinates,
    pub madhab_type: salah::Madhab,
    pub time_calculation_method: salah::Method, 

}


impl Default for PrayerParameters{
    fn default() -> Self {
        Self{
            date: Utc.ymd(2023, 01, 15),
            location: Coordinates::new(9.1099, 7.4042), //Location of Gwarinpa
            madhab_type: Madhab::Shafi,
            time_calculation_method: Method::Egyptian,
        }
    }
}


fn all_prayer_time() {
//let date  = Utc.ymd(2019, 1, 25);
println!("{}", PrayerParameters::default().date);
let params= Configuration::with(PrayerParameters::default().time_calculation_method, PrayerParameters::default().madhab_type);
let prayers= PrayerSchedule::new()
                      .on(PrayerParameters::default().date)
                      .for_location(PrayerParameters::default().location)
                      .with_configuration(params)
                      .calculate();
    println!("Hello, world!");
    println!("{:?}", prayers.unwrap());
}

fn next_pray_time() {
    
    let params= Configuration::with(PrayerParameters::default().time_calculation_method, PrayerParameters::default().madhab_type);
    let nextprayertime = PrayerTimes::new(PrayerParameters::default().date, PrayerParameters::default().location, params);
    println!("{:?}", nextprayertime.time_remaining());
}

fn main() {
    next_pray_time();
    all_prayer_time();
}