
unsafe extern "C" 
{
   unsafe fn add_numbers(a: i32, b: i32) -> i32;
}   

fn main() 
{
    let sum: i32 = unsafe { add_numbers(5, 10) };
    println!("The sum of 5 and 10 is: {}", sum);
}
