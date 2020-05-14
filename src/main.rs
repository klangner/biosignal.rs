use std::path::Path;
use wfdb_rust;


fn main() {
    let path = Path::new("data/mit-bih-arrhythmia-database-1.0.0/234.hea");
    let (header, signals) = wfdb_rust::parse_wfdb(&path);
    println!("{:?}", header);
    for signal in signals {
        println!("{:?}", &signal[0..10]);
    }
}