mod israeliqueue;
pub use israeliqueue::IsraeliQueue;

#[cfg(test)]
mod tests {
    pub use IsraeliQueue;

    #[test]
    fn init() {
        let iq: IsraeliQueue<i32> = IsraeliQueue::new();
        assert!(iq.is_empty());
    }

    fn test_comparator(a: &i32, b: &i32) -> bool {
        a == b
    }

    #[test]
    fn queue_dequeue() {
        let mut iq: IsraeliQueue<i32> = IsraeliQueue::new();
        iq.queue(15, Some(&test_comparator));
        iq.queue(1, Some(&test_comparator));
        iq.queue(13, Some(&test_comparator));
        iq.queue(15, Some(&test_comparator));
        assert!(!iq.is_empty());
        assert_eq!(15, *iq.peek().unwrap());
        assert_eq!(15, iq.dequeue().unwrap());
        assert_eq!(15, iq.dequeue().unwrap());
        assert_eq!(1, iq.dequeue().unwrap());
        assert_eq!(13, iq.dequeue().unwrap());
    }

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
}
