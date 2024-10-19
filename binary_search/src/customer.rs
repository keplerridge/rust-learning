use std::cmp::Ordering;

// Here I create the basic form of a Customer
pub struct Customer {
    pub name: String,
    pub amount_spent: u32,
    pub gender: String
}

// This builds the basic functionality of the Customer struct including the constructor
// a basic way to print/display it and lastly a function to update how much money that an
// individual customer has spent
impl Customer {
    pub fn new(name: String, amount_spent: u32, gender: String) -> Customer {
        Customer {name, amount_spent, gender}
    }

    pub fn display(&self) {
        println!("Name: {}, Amount Spent: {}, Gender: {}", self.name, self.amount_spent, self.gender)
    }

    pub fn set_amount_spent(&mut self, new_money: u32) {
        self.amount_spent += new_money 
    }
}

// These next few functions allow me to compare customer objects with just the comparison operators
impl PartialEq for Customer {
    fn eq(&self, other: &Self) -> bool {
        self.amount_spent == other.amount_spent
    }
}

impl PartialOrd for Customer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.amount_spent.cmp(&other.amount_spent))
    }
}

impl Ord for Customer {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for Customer {}