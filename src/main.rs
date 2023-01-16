use islam::time;
use time::date::Date;
use islam::pray::PrayerTimes;
use islam::pray::PrayerSchedule;
use islam::pray::Location;
use islam::pray::Config;
use islam::pray::Madhab;
use islam::pray::Method;


use chrono::{ Utc,  TimeZone, Local };
pub struct PrayerParameters {
    pub date: Date,
    pub timezone: i32,
    pub location: Location,
    pub madhab_type: Madhab,
    pub time_calculation_method: Method, 

}


impl Default for PrayerParameters{
    fn default() -> Self {
        Self{
            date: Date::from_calendar_date(2023, ::time::Month::January, 16).unwrap(),
            timezone: 7,
            location: Location::new(9.1099, 7.4042, Self::default().timezone), //Location of Gwarinpa
            madhab_type: Madhab::Shafi,
            time_calculation_method: Method::Egyptian,
        }
    }
}


fn all_prayer_time() {
//let date  = Utc.ymd(2019, 1, 25);
        // https://www.mapcoordinates.net/en
    let config = Config::new().with(PrayerParameters::default().time_calculation_method, PrayerParameters::default().madhab_type);
    let prayer_times = PrayerSchedule::new(PrayerParameters::default().location)?
        .on(PrayerParameters::default().date)
        .with_config(config)
        .calculate()?;

    let fajr = prayer_times.fajr;
    println!("fajr: {}:{}:{}", fajr.hour(), fajr.minute(), fajr.second());

    let sherook = prayer_times.sherook;
    println!(
        "sherook: {}:{}:{}",
        sherook.hour(),
        sherook.minute(),
        sherook.second()
    );

    let dohr = prayer_times.dohr;
    println!("dohr: {}:{}:{}", dohr.hour(), dohr.minute(), dohr.second());

    let asr = prayer_times.asr;
    println!("asr: {}:{}:{}", asr.hour(), asr.minute(), asr.second());

    let maghreb = prayer_times.maghreb;
    println!(
        "maghreb: {}:{}:{}",
        maghreb.hour(),
        maghreb.minute(),
        maghreb.second()
    );

    let ishaa = prayer_times.ishaa;
    println!(
        "ishaa: {}:{}:{}",
        ishaa.hour(),
        ishaa.minute(),
        ishaa.second()
    );
}


fn next_pray_time() {
    
    let params= Config::with(PrayerParameters::default().time_calculation_method, PrayerParameters::default().madhab_type);
    let nextprayertime = PrayerTimes::new(PrayerParameters::default().date, PrayerParameters::default().location, params);
    println!("{:?}", nextprayertime);
}

fn main() {
    next_pray_time();
    all_prayer_time();
}