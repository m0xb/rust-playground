fn main() {
	println!("Structs!");

	// a detour for printing and formatting
	let s = 1./10. + 1.0/10000.0;
	println!("What is \"s\"? It is: {:.s$}", s, s=5);

	#[derive(Debug)]
	struct MyStructure(i32);

	let x = MyStructure(6565);
	println!("My struct is {:?}", x);
	println!("The first element of the struct is {:?}", x.0);
}
