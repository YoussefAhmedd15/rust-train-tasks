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

fn main() {}
