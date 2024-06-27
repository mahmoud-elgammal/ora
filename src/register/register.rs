use regex::Regex;

enum Error {
    NotFound,
}

pub struct Event {
    hash: String,
    value: String,
    action: String,
}

pub struct Register {
    events: Vec<Event>,
}

impl Register {
    fn new() -> Register {
        Register { events: Vec::new() }
    }

    fn insert(&mut self, hash: String, value: String, action: String) {
        self.events.push(Event {
            hash,
            value,
            action,
        });
    }

    fn read(&self, hash: String) -> String {
        for event in &self.events {
            if event.hash == hash {
                return event.value.clone();
            }
        }

        String::new()
    }

    // query will be like $<index>
    fn index(&self, query: String) -> Result<String, Error> {
        let re = Regex::new(r"^\\$(?P<index>\\d+)$").unwrap();
        if let Some(cap) = re.captures(&query.to_string()) {
            if let Some(index) = cap.name("index") {
                if let Ok(index_num) = index.as_str().parse::<usize>() {
                    println!("Index: {}", index_num);
                }
            }
        }

        Err(Error::NotFound)
    }
}
