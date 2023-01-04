use std::io;

fn main() {


    let nth: u32;
    loop{
        let mut input = "".to_string();
        println!("Introduce the index to which you wish to calculate the Fibonnacci sequence:");
        io::stdin().read_line(&mut input).expect("Falied to read");
        nth = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break;
    }
    let mut fib_pair: (u128,u128) = (0,1);
    for _i in 1..=nth{
        fib_pair = calculate_next_fib(fib_pair);
    }

    println!("The value of the Fibonnacci sequence in the {nth} occurrence is : {}",fib_pair.1);

}
//Gran problema de tamaños ya que u128 no es suficiente para indices grandes
fn calculate_next_fib(sequence: (u128,u128)) -> (u128,u128){
    (sequence.1,sequence.0+sequence.1)
}
