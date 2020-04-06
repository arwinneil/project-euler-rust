// Finds all the palindrome made from the product of two 3-digit numbers.

fn main() {

    println!("all the palindrome made from the product of two 3-digit numbers : ");

    for i in 100..1000 {
        for j in 100..1000 {

            let current = i*j;
            let characters:Vec<char> = current.to_string().chars().collect();

            // Products of numbers 3 digits long range contain 5 or 6 digit
            let mut  is_palindrome:bool = true;
            for k in 0..(3+characters.len() % 5){

               if characters[k] != characters[characters.len()-1-k]{
                   is_palindrome = false;
               }
            }

            if is_palindrome{
               println!("{} ({}x{})", current,i,j);
            }
        }
    }
}