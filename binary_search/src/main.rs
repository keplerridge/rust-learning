mod customer;
use customer::Customer;

mod search;
use search::binary_search;

mod sort;
use sort::quicksort;

use std::string::String;

fn main() {
    // Here I just had ChatGPT create a list for me to use, maybe later on I will
    // create a programatic way to do this but this works for now
    let mut customers = vec![
        Customer::new(String::from("Alice"), 563, String::from("Female")),
        Customer::new(String::from("Bob"), 745, String::from("Male")),
        Customer::new(String::from("Carol"), 234, String::from("Female")),
        Customer::new(String::from("David"), 910, String::from("Male")),
        Customer::new(String::from("Eve"), 500, String::from("Female")),
        Customer::new(String::from("Frank"), 350, String::from("Male")),
        Customer::new(String::from("Grace"), 123, String::from("Female")),
        Customer::new(String::from("Henry"), 888, String::from("Male")),
        Customer::new(String::from("Irene"), 675, String::from("Female")),
        Customer::new(String::from("Jack"), 422, String::from("Male")),
        Customer::new(String::from("Karen"), 794, String::from("Female")),
        Customer::new(String::from("Leo"), 295, String::from("Male")),
        Customer::new(String::from("Mia"), 612, String::from("Female")),
        Customer::new(String::from("Noah"), 485, String::from("Male")),
        Customer::new(String::from("Olivia"), 739, String::from("Female")),
        Customer::new(String::from("Paul"), 267, String::from("Male")),
        Customer::new(String::from("Quinn"), 820, String::from("Female")),
        Customer::new(String::from("Ryan"), 156, String::from("Male")),
        Customer::new(String::from("Sophia"), 432, String::from("Female")),
        Customer::new(String::from("Tom"), 678, String::from("Male")),
        Customer::new(String::from("Uma"), 324, String::from("Female")),
        Customer::new(String::from("Victor"), 890, String::from("Male")),
        Customer::new(String::from("Wendy"), 238, String::from("Female")),
        Customer::new(String::from("Xander"), 572, String::from("Male")),
        Customer::new(String::from("Yara"), 483, String::from("Female")),
        Customer::new(String::from("Zach"), 634, String::from("Male")),
        Customer::new(String::from("Ava"), 290, String::from("Female")),
        Customer::new(String::from("Ben"), 760, String::from("Male")),
        Customer::new(String::from("Clara"), 525, String::from("Female")),
        Customer::new(String::from("Daniel"), 410, String::from("Male")),
        Customer::new(String::from("Ella"), 908, String::from("Female")),
        Customer::new(String::from("Felix"), 123, String::from("Male")),
        Customer::new(String::from("Gina"), 342, String::from("Female")),
        Customer::new(String::from("Hank"), 234, String::from("Male")),
        Customer::new(String::from("Isla"), 780, String::from("Female")),
        Customer::new(String::from("James"), 623, String::from("Male")),
        Customer::new(String::from("Kate"), 197, String::from("Female")),
        Customer::new(String::from("Liam"), 854, String::from("Male")),
        Customer::new(String::from("Maya"), 470, String::from("Female")),
        Customer::new(String::from("Nate"), 317, String::from("Male")),
        Customer::new(String::from("Opal"), 563, String::from("Female")),
        Customer::new(String::from("Pete"), 699, String::from("Male")),
        Customer::new(String::from("Quinn"), 234, String::from("Female")),
        Customer::new(String::from("Rick"), 765, String::from("Male")),
        Customer::new(String::from("Sarah"), 401, String::from("Female")),
        Customer::new(String::from("Tom"), 290, String::from("Male")),
        Customer::new(String::from("Ursula"), 525, String::from("Female")),
        Customer::new(String::from("Vince"), 845, String::from("Male")),
        Customer::new(String::from("Willow"), 420, String::from("Female")),
        Customer::new(String::from("Xavier"), 310, String::from("Male")),
        Customer::new(String::from("Yasmine"), 665, String::from("Female")),
        Customer::new(String::from("Zachary"), 129, String::from("Male")),
        Customer::new(String::from("Alice"), 750, String::from("Female")),
        Customer::new(String::from("Bob"), 612, String::from("Male")),
        Customer::new(String::from("Carol"), 899, String::from("Female")),
        Customer::new(String::from("David"), 377, String::from("Male")),
        Customer::new(String::from("Eve"), 530, String::from("Female")),
        Customer::new(String::from("Frank"), 275, String::from("Male")),
        Customer::new(String::from("Grace"), 810, String::from("Female")),
        Customer::new(String::from("Henry"), 560, String::from("Male")),
        Customer::new(String::from("Irene"), 413, String::from("Female")),
        Customer::new(String::from("Jack"), 702, String::from("Male")),
        Customer::new(String::from("Karen"), 982, String::from("Female")),
        Customer::new(String::from("Leo"), 215, String::from("Male")),
        Customer::new(String::from("Mia"), 874, String::from("Female")),
        Customer::new(String::from("Noah"), 678, String::from("Male")),
        Customer::new(String::from("Olivia"), 489, String::from("Female")),
        Customer::new(String::from("Paul"), 725, String::from("Male")),
        Customer::new(String::from("Quinn"), 540, String::from("Female")),
        Customer::new(String::from("Ryan"), 361, String::from("Male")),
        Customer::new(String::from("Sophia"), 899, String::from("Female")),
        Customer::new(String::from("Tom"), 128, String::from("Male")),
        Customer::new(String::from("Uma"), 635, String::from("Female")),
        Customer::new(String::from("Victor"), 297, String::from("Male")),
        Customer::new(String::from("Wendy"), 789, String::from("Female")),
        Customer::new(String::from("Xander"), 410, String::from("Male")),
        Customer::new(String::from("Yara"), 632, String::from("Female")),
        Customer::new(String::from("Zach"), 258, String::from("Male")),
        Customer::new(String::from("Ava"), 785, String::from("Female")),
        Customer::new(String::from("Ben"), 350, String::from("Male")),
        Customer::new(String::from("Clara"), 645, String::from("Female")),
        Customer::new(String::from("Daniel"), 275, String::from("Male")),
        Customer::new(String::from("Ella"), 870, String::from("Female")),
        Customer::new(String::from("Felix"), 123, String::from("Male")),
        Customer::new(String::from("Gina"), 524, String::from("Female")),
        Customer::new(String::from("Hank"), 711, String::from("Male")),
        Customer::new(String::from("Isla"), 423, String::from("Female")),
        Customer::new(String::from("James"), 195, String::from("Male")),
        Customer::new(String::from("Kate"), 892, String::from("Female")),
        Customer::new(String::from("Liam"), 336, String::from("Male")),
        Customer::new(String::from("Maya"), 649, String::from("Female")),
        Customer::new(String::from("Nate"), 201, String::from("Male")),
        Customer::new(String::from("Opal"), 764, String::from("Female")),
        Customer::new(String::from("Pete"), 437, String::from("Male")),
        Customer::new(String::from("Quinn"), 580, String::from("Female")),
        Customer::new(String::from("Rick"), 823, String::from("Male")),
        Customer::new(String::from("Sarah"), 172, String::from("Female")),
        Customer::new(String::from("Tom"), 398, String::from("Male")),
        Customer::new(String::from("Ursula"), 579, String::from("Female")),
        Customer::new(String::from("Vince"), 890, String::from("Male"))
    ];

    // Display customers list before sorting
    for cx in &customers {
        cx.display();
    }

    println!();
    println!("Now sorting customers:");
    // Call the quicksort function on the list of customers
    quicksort(&mut customers);

    // Display freshly sorted list of customers
    for cx in &customers {
        cx.display();
    }

    println!();

    // Search for someone who spent 874. I recognize that it would be better to search based
    // on name but I am sorting based on amount spent so that is what I am searching on.
    
    let search_amount = 874;
    let target_customer = Customer::new(String::from(""), search_amount, String::from(""));
    
    println!("We are searching for a customer that spent {}", search_amount);
    match binary_search(&customers, &target_customer) {
        Some(index) => {
            println!("Found customer who spent {}: ", search_amount);
            customers[index].display();
        }
        None => {
            println!("There was no customer that spent {}", search_amount);
        }
    }

    // Here I search for a customer I know is not in the list to show what happens
    let search_amount = 1100;
    let target_customer = Customer::new(String::from(""), search_amount, String::from(""));
    
    println!();
    println!("We are searching for a customer that spent {}", search_amount);
    match binary_search(&customers, &target_customer) {
        Some(index) => {
            println!("Found customer who spent {}: ", search_amount);
            customers[index].display();
        }
        None => {
            println!("There was no customer that spent {}", search_amount);
        }
    }
}