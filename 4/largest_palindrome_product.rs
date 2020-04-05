// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
// Find the largest palindrome made from the product of two 3-digit numbers.

fn main() {

    let mut largest_palindrome:u32 = 0;

    for i in (100..1000).rev() {
        for j in (100..1000).rev() {

            let current = i*j;
            let characters:Vec<char> = current.to_string().chars().collect();

            // Products of numbers 3 digits long range contain 5 or 6 digit
            let mut  is_palindrome:bool = true;
            for k in 0..(3+characters.len() % 5){

               if characters[k] != characters[characters.len()-1-k]{
                   is_palindrome = false;
               }
            }

            if is_palindrome && current> largest_palindrome{
                largest_palindrome = current;
            }
        }
    }

    println!("Largest palindrome from product of two 3-digit numbers is {}", largest_palindrome);
}