use std::io;
use std::thread;
use std::time::Duration;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("i have generated a number from 1 to 100. guess what it is or suffer the consequences.\n");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("the number is {secret_number}.\n");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line...\n");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            thread::sleep(Duration::from_secs(1));
            println!("");
            println!("dawg. that isn't a number. quit being difficult.\n");
            return;
        }
    };

    println!("");

    thread::sleep(Duration::from_secs(1));

    println!("good.");

    thread::sleep(Duration::from_secs(1));

    println!("wait. actually huh");

    thread::sleep(Duration::from_secs(2));

    println!("what...? '{guess}'... are you sure??\n");

    let mut confirmation_1 = String::new();

    io::stdin()
        .read_line(&mut confirmation_1)
        .expect("failed to read line...");

    let confirmation_1 = confirmation_1.trim();

    println!("");

    thread::sleep(Duration::from_secs(1));

    println!("really? are you absolutely positive...?\n");

    let mut confirmation_2 = String::new();

    io::stdin()
        .read_line(&mut confirmation_2)
        .expect("failed to read line...\n");

    let confirmation_2 = confirmation_2.trim();

    println!("");

    thread::sleep(Duration::from_secs(1));

    println!("okay... if you say so...");

    thread::sleep(Duration::from_secs(2));

    println!("locking in '{guess}'. wow. i can't tell if you're confident or just stupid.\n");

    thread::sleep(Duration::from_secs(4));

    println!("oh hold on i gotta find the actual number i generated...");

    thread::sleep(Duration::from_secs(2));

    println!("gimmie a sec... where did i put it??");

    thread::sleep(Duration::from_secs(2));

    println!("oh yeah here it is.\n");

    thread::sleep(Duration::from_secs(1));

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("of course '{guess}' is wrong. way below the number. try again.\n"),
        Ordering::Equal => { 
            println!("yeah. you got it. i was just messing with you. good job.");
            thread::sleep(Duration::from_secs(1));
            println!("well anyways, til next time.\n");
            return;
        }
        Ordering::Greater => println!("of course '{guess}' is wrong. way above the number. try again.\n"),
    } // guess comparison close brace

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line...\n");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                thread::sleep(Duration::from_secs(1));
                println!("");
                println!("dawg. that isn't a number. quit being difficult.\n");
                continue;
            }
        };

        println!("");

        thread::sleep(Duration::from_secs(1));

        println!("mmmh. okay");

        thread::sleep(Duration::from_secs(1));
    
        println!("not gonna ask if you're sure, because clearly you are.");

        thread::sleep(Duration::from_secs(2));

        println!("for whatever reason. '{guess}'? really?\n");
    
        thread::sleep(Duration::from_secs(3));

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("yeah. '{guess}' is wrong. below my number. try again.\n"),
            Ordering::Equal => { 
                println!("just kidding. you got it. i was messing with you. took you long enough.");
                thread::sleep(Duration::from_secs(1));
                println!("well anyways, til next time.\n");
                break;
            }
            Ordering::Greater => println!("yeah. '{guess}' is wrong. above my number. try again.\n"),
        } // guess comparison close brace

    } // loop close brace

    thread::sleep(Duration::from_secs(1));
}
