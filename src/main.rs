use std::collections::HashMap;
fn main() {
    let mut arguments = std::env::args().skip(1);

    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();

    let data = format!("{}\t{}\n",key,value);
    std::fs::write("qs.db", data).unwrap();
}
struct  DataBase{
    map: HashMap<String,String>,
}
impl DataBase {
    fn new() -> Result<DataBase, std::io::Error> {
        // let data =  match std::fs::read_to_string("qs.db") {
        //     Ok(d) => d,
        //     Err(error)=>{
        //         return  Err(error);
        //     }
        // };

        let data = std::fs::read_to_string("qs.db")?;
        let mut  map = HashMap::new();

        for line in data.lines(){
            let mut pair = line.splitn(2,'\t');

            let key = pair.next().expect("No key");
            let value = pair.next().expect("No value");
            
            map.insert(key.to_owned(), value.to_owned());
        }

        Ok(DataBase{map: map})
    }
}
