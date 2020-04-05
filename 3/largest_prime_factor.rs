//The prime factors of 13195 are 5, 7, 13 and 29.
//What is the largest prime factor of the number 600851475143 ?
fn  main() {

// Only checks odd numbers

    let mut num:u64 = 600851475143;
    let mut counter = 3; 
    let mut largest_prime_factor = 0;

    while counter * counter < num {
        if num % counter == 0{
            num = num / counter;
            largest_prime_factor = counter;
        }else{
            counter +=2;
        }

        if num > largest_prime_factor{
            largest_prime_factor = num;
        }
    }

    println!("Largest prime factor of 600851475143 is : {}", largest_prime_factor);
}