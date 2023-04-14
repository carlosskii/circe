use cce_dbmodel::{dbnode, Database};


#[dbnode]
struct Person {
  name: String,
  age: u32
}


#[test]
fn test_database_custom() {
  let mut db = Database::new();

  let person = Person::new("John".to_string(), 42);
  db.add_node(person);

  let person = db.get_node(0).unwrap();
  assert_eq!(person.name, "John");
  assert_eq!(person.age, 42);
}