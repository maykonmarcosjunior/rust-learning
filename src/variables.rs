const G_CTE: u32 = 100;

pub fn run() {
    const L_CTE: u32 = 50;
    println!("The value of the global cte is: {G_CTE}\nThe value of the local cte is: {L_CTE}");
    let mut x: isize = 5;
    let y = 1;
    println!("The value of x, y is: {x}, {y}");
    {
        let mut x = x*2;
        let y = y*2;
        x += 1;
        println!("the value of x, y within this scope is: {x}, {y}")
    }
    x = 6;
    let y = 10;
    println!("The value of x, y after reassigning on main scope is: {x}, {y}");
    let str_int: u32 = "42".parse().expect("Not a number!");
    println!("The value brought from a string is {str_int}");

    println!("-----------------------------------------------------------");
    /*{
        let mut positivo_8: u8 = 0;
        let mut inteiro_8: i8 = 127;
        println!("The 8-bits values are: {positivo_8}, {inteiro_8}");
        positivo_8 -= 1;
        inteiro_8 += 1;
        println!("The overflowed 8-bits values are: {positivo_8}, {inteiro_8}");
    }*/

    // addition = 15
    let sum = 5 + 10;
    println!("sum = {sum}");


    // subtraction = 
    let difference = 95.5 - 4.3;
    println!("difference = {difference}");


    // multiplication
    let product = 4 * 30;
    println!("product = {product}");


    // division
    let quotient = 56.7 / 32.2;
    println!("quocient = {quotient}");
    let truncated = -5 / 3; // Results in -1
    println!("truncated = {truncated}");


    // resto
    let remainder = 43 % 5;
    println!("resto = {remainder}");

    let mut booleano = true;
    println!("bool = {booleano}");
    booleano = false;
    println!("bool = {booleano}");

    
    let t = ([1; 2], [3; 4]);
    let (a, b) = t;
    println!("{}", a[0] + t.1[0]);

}
