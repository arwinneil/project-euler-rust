//Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

fn main(){

let mut sum_of_squares:u64 = 0;
let mut sum:u64 =0;

	for i in 1..=100 as u64{

		sum_of_squares += i.pow(2);
		sum += i;
	}

	println!("Difference between sum of squares of first 100 numbers and square of the sum : {}", (sum.pow(2)- sum_of_squares));
}