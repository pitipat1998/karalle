extern crate csv;

pub fn read_csv(path: &str) -> Vec<i32> {
    let rdr = csv::Reader::from_path(path);
    // let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut v: Vec<i32> = Vec::new();
    for result in rdr.unwrap().records() {
        let record = result.unwrap();
        let number = record.get(0).unwrap().parse::<i32>().unwrap();
        v.push(number)
    }
    v
}