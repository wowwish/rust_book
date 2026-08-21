fn main() {
    /*
    The loop keyword tells Rust to execute a block of code again and again until you
    interrupt it manually or using the break keyword. You can also re-iterate from
    the start of the code block of the loop using the continue keyword.
    */
    // loop {
    //     println!("again!");
    // }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {result}");

    /*
    If you have nested loops, break and continue apply only to the loop context in which
    they are used. You can optionally specify a loop label on a loop that you can
    then use with break or continue to specify that these keywords apply to the
    labelled loop instead of the loop inside which the keywords are used.
    */
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; // exiting labelled loop
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // While loop
    let mut n = 3;
    while n != 0 {
        println!("{n}!");
        n -= 1;
    }
    println!("LIFTOFF!!!");

    /*
    Looping through a collection using a while loop is prone to index out of range
    errors and is slow because of additional runtime checking of loop termination condition
    on each iteration. A for loop is an optimal way to iterate over a collection instead.
    */
    let a = [10, 20 , 30, 40, 50];
    // let mut index = 0;

    // while index < 5 {
    //     println!("The value is: {}", a[index]);
    //     index += 1;
    // }

    for element in a {
        println!("The value is: {element}");
    }

    // The for loop can also be used to repeat execution of code a set number of times like so
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
