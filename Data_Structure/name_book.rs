// Rust Example Vector of Structs

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
struct Person {
    name: String,
    age: u32
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Person {
            name,
            age
        }
    }
}

fn show_elements(s: &Vec<Person>)
{
    for x in s
    {
	println!("Age:{} \n Name: {}\n",x.age,x.name);
    }
}

fn main() {
    let mut people = vec![     
	Person::new("Artorias".to_string(), 43),  
	Person::new("Zorro".to_string(), 25),
        Person::new("Jason".to_string(), 60),
        Person::new("Thundercat".to_string(), 1),
	Person::new("Corsair".to_string(),123)
    ];

    people.push(Person::new("Babaca".to_string(),18));
    people.push(Person::new("Flip".to_string(),34));

// Sort people by age
    people.sort_by(|a, b| b.age.cmp(&a.age));
// List elements
    show_elements(&people);  // with directive Clone on header of this code
    println!("---\nRemove item with name Zorro\n---\n");
// Remove by name
    people.retain(|ref mut i|!i.name.contains("Zorro"));
// Sort people by name
    people.sort();
    show_elements(&people);

}
