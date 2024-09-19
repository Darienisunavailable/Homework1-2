// Implement a function is_even(n: i32) -> bool
// that returns true if a number is even, false otherwise.
// (Syntax might be wrong)
fn is_even(n: &i32) -> bool {
    if n % 2 == 0 {
        true
    }
    else {
        false
    }
}

fn main() {
    // Create an array of 10 integers
    let array: [i32; 10] = [3, 7, 10, 12, 15, 18, 22, 24, 27, 30];

    // Use a for loop to go through the array
    for x in array.iter() {
        if x % 3 == 0 && x % 5 == 0 {
            println!{"FizzBuzz"}
        }
        else if x % 3 == 0 {
            println!{"Fizz"}
        }
        else if x % 5 == 0 {
            println!{"Buzz"}
        }
        else {
            println!("{}", is_even(x));
        }
    }

    // Use a while loop to go through the array and add everything up
    let mut counter = 0;
    let mut sum = 0;
    while counter != 10 {
        sum += array[counter];
        counter += 1;
    }
    println!("The sum is {}", sum);

    // Use a loop to find and print the largest number in the array
    let mut largest = 0;
    counter = 0;
    loop {

        if array[counter] > largest {
            largest = array[counter]
        }

        counter += 1;
        if counter == 10 {
            println!("The largest number is {}", largest);
            break
        }
    }

}
