mod balances;

fn main() {
	println!("Hello, world Transfer!");

	let mut balances = balances::Pallet::new();

	balances.set_balance(&"alice".to_string(), 100);

	println!("alice balance: {}", balances.balance(&"alice".to_string()));
	println!("bob balance: {}", balances.balance(&"bob".to_string()));

	//perform a transfer

	match balances.transfer("alice".to_string(), "bob".to_string(), 50){
		Ok(_) => println!("transfer successful"),
		Err(e) => println!("transfer failed: {}", e),
	}

	println!("alice balance: {}", balances.balance(&"alice".to_string()));
	println!("bob balance: {}", balances.balance(&"bob".to_string()));
}
