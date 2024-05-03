use std::collections::BTreeMap;

struct AllOne {
    words_bt_count: BTreeMap<String, u64>
}

impl AllOne {
    pub fn new() -> Self {
        Self {
            words_bt_count: BTreeMap::new()
        }
    }
    
    pub fn inc(&mut self, key: String) {
        let count = self.words_bt_count.entry(key).or_insert(0);
        *count += 1;
    }
    
    pub fn dec(&mut self, key: String) {
        let count = self.words_bt_count.entry(key.clone()).or_insert(0);

        if *count == 0 || *count == 1 {
            self.words_bt_count.remove(&key);
        } else {
            *count -= 1;
        }
    }
    
    pub fn get_max_key(&self) -> String {        
        match self.words_bt_count.iter().max_by_key(|&(_, count)| count)  {
            Some(result) => result.0.clone(),
            None => "".to_string(),
        }
    }
    
    pub fn get_min_key(&self) -> String {
        match self.words_bt_count.iter().min_by_key(|&(_, count)| count)  {
            Some(result) => result.0.clone(),
            None => "".to_string(),
        }
    }
}


fn main() {
    let mut all_one = AllOne::new();

    all_one.inc("hello".to_string());
    all_one.inc("hello".to_string());

    let max_1 = all_one.get_max_key(); // return "hello"
    let min_1 = all_one.get_min_key(); // return "hello"

    all_one.inc("leet".to_string());
    
    let max_2 = all_one.get_max_key(); // return "hello"
    let min_2 = all_one.get_min_key(); // return "leet"

    all_one.dec("hello".to_string());
    all_one.dec("hello".to_string());

    let max_3 = all_one.get_max_key(); // return "leet"

    println!("{max_1} {min_1} {max_2} {min_2} {max_3}");
}
