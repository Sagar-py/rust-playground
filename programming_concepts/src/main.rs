/* Variables and Mutability
fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x in the outer scope is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The length of spaces is: {}", spaces);
}
 */

/* Data Types
fn main() {
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let remainder = 43 % 5;
    let t = true;
    let f: bool = false;
    let c = 'z';
    let z: char = 'ℤ';
    let face = '😻';
    println!("The value of sum is: {}", sum);
    println!("The value of difference is: {}", difference);
    println!("The value of product is: {}", product);
    println!("The value of quotient is: {}", quotient);
    println!("The value of truncated is: {}", truncated);
    println!("The value of remainder is: {}", remainder);
    println!("The value of t is: {}", t);
    println!("The value of f is: {}", f);
    println!("The value of c is: {}", c);
    println!("The value of z is: {}", z);
    println!("The value of face is: {}", face);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    println!("The value of x is: {}", x);
    println!("The value of z is: {}", z);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("The value of five_hundred is: {}", five_hundred);
    println!("The value of six_point_four is: {}", six_point_four);
    println!("The value of one is: {}", one);

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("The value of first is: {}", first);
    println!("The value of second is: {}", second);

}
 */

/* Functions */
fn five() -> i32 {
    5
}
fn main() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);
    let x = five();
    println!("The value of x is: {}", x);
}