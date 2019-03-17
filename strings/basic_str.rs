

fn main() 
{
 let mut cow = "Muuu".to_string();
 let dog = "_Haus".to_string();
 let cat = "_Meows ".to_string();

// concatenate string
 cow.push_str(&dog);
 println!("1- cat+gog {} \n",cow);
// concatenate string second way
 let othertest=format!("Cat:{}\nDog:{}\n", cat, dog+&"LoooL".to_string());
 println!("{}\n",othertest);
// third way to cocatenate 
let s = concat!("test", 10, '1', true);
println!("{}\n",&s);

// simple parser, to extract numbers for each lines
let word="cacau = 123\ncoffee = 432\n".to_string();
let mut makestr= String::new();
let mut count = 0;

 for c in word.chars() 
 {
  println!("Character: {}", c);

	if c == '='
  	{
		count=1;
  	}
	if count > 0 
	{       
		if c.is_digit(10) 
		{
			makestr.push(c);
		}
	}
	if c == '\n'
	{
		makestr.push('\n');
		count=0;
	}
 }
// remove first two chars
 let endstring = &makestr[2..];
// replace string
 let pres= endstring.replace("=","");
 println!("{}",word);
 println!("Values: \n{}\n",pres);

 // tokenize example
 let mut iter = "cow in grass".split_whitespace();
 assert_eq!(Some("cow"), iter.next());
 assert_eq!(Some("in"), iter.next());

// split example util in CSV
 let mut listas: Vec<&str> = "bananas,apples,pear".split(",").collect(); // ["bananas", "apples", "pear"]
// Add element on Vec
 listas.push("coco");
 listas.push("kiwi");
// remove element pear
 listas.remove(3);

// remove by element name
 listas.retain(|&i|!i.contains("bananas"));

	for x in listas 
  	{
		println!("{}",x);
  	}
 
}
