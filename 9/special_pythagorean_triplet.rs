// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which, a^2 + b^2 = c^2
// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
// Find the product abc.

fn main(){

    'a:for a in 1..((1000/3) as u32){
        for b in 1..((1000/2) as u32){

            let c = 1000 - a - b;

            if a.pow(2) + b.pow(2) == c.pow(2){
                println!("Product of a ={} , b={}, c={},\nfor which a + b + c = 1000 & a^2 + b^2 = c^2,\nis {}",a,b,c,(a*b*c));

                break 'a;
            }
        }
    }
}