extern crate csv;
pub fn read_csv(path: &str) -> Vec<u128> {
    let rdr = csv::Reader::from_path(path);
    // let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut v: Vec<u128> = Vec::new();
    for result in rdr.unwrap().records() {
        let record = result.unwrap();
        let number = record.get(0).unwrap().parse::<u128>().unwrap();
        v.push(number)
    }
    v
}