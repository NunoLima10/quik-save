use std::{collections::HashMap};
fn main() {
    let mut arguments = std::env::args().skip(1);

    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();

    let mut data_base =  DataBase::new().expect("data base error");
    data_base.add(key.to_uppercase(),value.clone());
    data_base.add(key,value);
    data_base.flush().unwrap();
}
struct  DataBase{
    map: HashMap<String,String>,
}
impl DataBase {
    fn new() -> Result<DataBase, std::io::Error> {
        let data = std::fs::read_to_string("qs.db")?;
        let mut  map = HashMap::new();

        for line in data.lines(){
            let mut pair = line.splitn(2,'\t');

            let key = pair.next().expect("No key");
            let value = pair.next().expect("No value");
            
            map.insert(key.to_owned(), value.to_owned());
        }  

        Ok(DataBase{map})
    }
    fn add(&mut self, key: String, value: String){
        self.map.insert(key, value);
    }
    fn flush(self) -> std::io::Result<()>{
        let mut  data = String::new();
        for (key, value) in &self.map{
            data.push_str(key);
            data.push('\t');
            data.push_str(value);
            data.push('\n');
        }

        std::fs::write("qs.db", data)
    }
}
