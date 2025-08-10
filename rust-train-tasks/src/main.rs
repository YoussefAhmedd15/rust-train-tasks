#[deny(clippy::all)]
use std::io;
use std::io::stdin;
// Task 1
// fn main() {
//     let s1 = String::from("hello");
//     let s2 = String::from("world");
//     let answer = concatenate_strings(&s1, &s2);
//     println!("{answer}")
// }
// fn concatenate_strings(s1: &str, s2: &str) -> String {
//     let s3: String = format!("{}{}", s1, s2);
//     s3
// }

// Task 2
// struct Task {
//     id: u32,
//     description: String,
//     priority: String,
//     completed: bool,
// }
// fn main() {
//     let mut tasks: [Option<Task>; 5] = [None, None, None, None, None];
//     let mut task_id = 1;

//     loop {
//         println!("\n== To-Do List Manager ==");
//         println!("1. Add Task");
//         println!("2. List Tasks");
//         println!("3. Mark Task as Completed");
//         println!("4. Exit");

//         let mut choice = String::new();
//         io::stdin().read_line(&mut choice).unwrap();
//         let choice = choice.trim();

//         if choice == "1" {
//             println!("Description: ");
//             let mut desc = String::new();
//             io::stdin().read_line(&mut desc).unwrap();

//             println!("Priority: ");
//             let mut priority = String::new();
//             io::stdin().read_line(&mut priority).unwrap();

//             let mut added = false;

//             for i in 0..5 {
//                 if tasks[i].is_none() {
//                     tasks[i] = Some(Task {
//                         id: task_id,
//                         description: desc.trim().to_string(),
//                         priority: priority.trim().to_string(),
//                         completed: false,
//                     });
//                     println!("Task added with id {}", task_id);
//                     task_id += 1;
//                     added = true;
//                     break;
//                 }
//             }
//             if !added {
//                 println!("no more storage left for tasks")
//             }
//         } else if choice == "2" {
//             for i in 0..5 {
//                 if let Some(task) = &tasks[i] {
//                     let status = if task.completed { "Done" } else { "pending" };
//                     println!(
//                         "{}: {} ({}) - {}",
//                         task.id, task.description, task.priority, status
//                     )
//                 }
//             }
//         } else if choice == "3" {
//             let mut inp = String::new();
//             println!("Enter task ID to mark as completed:");
//             io::stdin().read_line(&mut inp).unwrap();
//             let id = inp.trim().parse().unwrap_or(0);

//             let mut found = false;
//             for i in 0..5 {
//                 if let Some(task) = &mut tasks[i] {
//                     if task.id == id {
//                         task.completed = true;
//                         println!("TAsk is completed");
//                         found = true;
//                         break;
//                     }
//                 }
//             }
//             if !found {
//                 println!("task not found")
//             }
//         } else if choice == "4" {
//             println!("Exiting");
//             break;
//         } else {
//             println!("invalid choice")
//         }
//     }
// }

// Task 3

trait Account {
    fn deposit(&mut self, amount: f64) -> Result<(), String>;
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;
    fn balance(&self) -> f64;
}

struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err(String::from("Deposit must be more than 0"));
        }
        self.balance += amount;
        Ok(())
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err(String::from("the amount must be greater than 0"));
        }
        if amount > self.balance {
            return Err(String::from("Not enough money"));
        }
        self.balance -= amount;
        Ok(())
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}
fn main() {
    let mut user1 = BankAccount {
        account_number: 1,
        balance: 1000.0,
        holder_name: String::from("Youssef"),
    };

    let mut user2 = BankAccount {
        account_number: 2,
        balance: 1750.0,
        holder_name: String::from("Mohamed"),
    };

    let result1 = user1.deposit(500.0);
    if result1.is_ok() {
        println!("Deposit succesful")
    } else {
        println!("Deposit failed: {}", result1.unwrap_err())
    }

    let result2 = user2.withdraw(2500.0);
    if result2.is_ok() {
        println!("Withdraw is succesful")
    } else {
        println!("Withdraw failed: {}", result2.unwrap_err())
    }

    println!("{} Balance: {}", user1.holder_name, user1.balance());
    println!("{} Balance: {}", user2.holder_name, user2.balance());
}
