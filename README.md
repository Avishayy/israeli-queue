============
IsraeliQueue
============

This crate implements the Israeli Queue data structure, which is a queue that allows objects to be inserted either at the end of the queue or nearby to an existing item in the queue that enables some connection.

The crate is currently WIP, PRs and suggestions are welcome.

Usage
-----
Insert this into your `Cargo.toml`:
```
israeli-queue = "0.1"
```

Insert those into your code:
```Rust
extern crate israeli-queue;

use israeli-queue::IsraeliQueue;
```

Example:
```Rust
#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    pub fn new(first_name: String, last_name: String) -> Person {
        Person {
            first_name,
            last_name,
        }
    }
}

fn person_comparator(a: &Person, b: &Person) -> bool {
    a.last_name == b.last_name
}

#[test]
fn test_persons() {
    let people = vec![
        Person::new(String::from("Yosi"), String::from("Cohen")),
        Person::new(String::from("Kim"), String::from("Tisbad")),
        Person::new(String::from("Ron"), String::from("Rick")),
        Person::new(String::from("Annie"), String::from("Cohen")),
        Person::new(String::from("Beelzebub"), String::from("Ligma")),
        Person::new(String::from("Apolexy"), String::from("Ligma")),
        Person::new(String::from("Mo"), String::from("Cohen")),
        Person::new(String::from("Ra"), String::from("Picky")),
    ];

    let mut people_queue = IsraeliQueue::new();

    for person in people {
        people_queue.queue(person, Some(&person_comparator));
    }

    for person in people_queue.into_iter() {
        println!("{:?}", person);
    }
}
```

Output will be:
```
Person { first_name: "Yosi", last_name: "Cohen" }
Person { first_name: "Mo", last_name: "Cohen" }
Person { first_name: "Annie", last_name: "Cohen" }
Person { first_name: "Kim", last_name: "Tisbad" }
Person { first_name: "Ron", last_name: "Rick" }
Person { first_name: "Beelzebub", last_name: "Ligma" }
Person { first_name: "Apolexy", last_name: "Ligma" }
Person { first_name: "Ra", last_name: "Picky" }
```
