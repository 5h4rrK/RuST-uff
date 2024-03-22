use std::io;
use todoapp::app::TodoList;

fn input() -> String {
    let mut tname = String::new();
    io::stdin().read_line(&mut tname).expect("Failed to read");
    tname
}
fn main() {
    let mut todo_list = TodoList::new();
    loop {
        println!();
        println!("1. Add Items");
        println!("2. List Items");
        println!("3. Mark Complete");
        println!("4. Completed Tasks ");
        println!("5. Exit ");

        let choose = input();
        let choose: u32 = match choose.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match choose {
            1 => {
                println!("Enter Task Name :: ");
                loop {
                    let tname = input();
                    if !tname.eq(&String::from("\n")) && !tname.eq(&String::from(" ")) {
                        todo_list.add_task(&tname);
                        break;
                    }
                }
            }
            2 => {
                todo_list.list_tasks();
            }

            3 => {
                let tid = input();
                let tid = match tid.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                todo_list.finish_task(tid);
            }

            4 => {
                todo_list.list_finished_task();
            }
            5 => {
                println!("Exiting..");
                break;
            }
            _ => {
                println!("Invalid choice.");
            }
        }
    }
}
