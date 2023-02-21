////////// DO NOT CHANGE BELOW HERE /////////
use std::collections::HashMap;

fn print_hashmap(hashmap: &HashMap<&str, &str>) {
    println!("{hashmap:#?}");
}
////////// DO NOT CHANGE ABOVE HERE /////////

// TODO: create `hashmap!()` macro.
macro_rules! hashmap {
    ($($lit:literal => $e:expr,)+) => {
        {
            let mut hm = HashMap::new();
            $(hm.insert($lit, $e);)+
            hm
        }
    };
}

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    let value = "my_string";
    let my_hashmap = hashmap!(
        "hash" => "map",
        "Key" => value,
    );

    print_hashmap(&my_hashmap);
}
