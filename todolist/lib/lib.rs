pub mod app {
    #[derive(Debug)]
    pub struct Item {
        pub id: u64,
        pub taskname: String,
        pub finished: bool,
    }
    #[derive(Debug)]
    pub struct TodoList {
        pub tasks: Vec<Item>,
        pub task_finished: u64,
        pub task_pending: u64,
    }
}

impl app::Item {
    pub fn modify(_id: u64, name: String, finish: bool) -> app::Item {
        app::Item {
            id: _id,
            taskname: name,
            finished: finish,
        }
    }
    pub fn clone(self) -> app::Item {
        self
    }
}

impl app::TodoList {
    pub fn new() -> app::TodoList {
        app::TodoList {
            tasks: Vec::new(),
            task_finished: 0,
            task_pending: 0,
        }
    }

    pub fn add_task(&mut self, taskname: &String) {
        self.task_pending += 1;
        let task_block: app::Item = self.frame_task(taskname.clone(), false);
        self.tasks.push(task_block);
    }

    pub fn frame_task(&mut self, name: String, finish: bool) -> app::Item {
        app::Item {
            id: self.task_finished + self.task_pending + 1,
            taskname: name,
            finished: finish,
        }
    }
    pub fn list_tasks(&self) {
        if self.task_pending == 0 {
            println!("No tasks Pending..\n");
        } else if self.task_pending != 0 {
            println!("Pending Tasks : {}", self.task_pending);
            println!(" {} ", String::from("-").repeat(52));
            for i in self.tasks.iter().enumerate() {
                println!(
                    "| Task {:<3} | {:<30} | {:<6} |",
                    i.0 as u64,
                    i.1.taskname.trim(),
                    i.1.finished
                );
                println!(" {} ", String::from("-").repeat(52));
            }
        }
    }

    pub fn list_finished_task(&self) {
        if self.task_finished == 0 {
            println!("No Finished Tasks !!\n");
        } else {
            println!("Finished Tasks : {}", self.task_finished);
            println!(" {} ", String::from("-").repeat(42));
            for i in self.tasks.iter().enumerate() {
                if i.1.finished {
                    println!("| Task {:<3} |{:<30} |", i.0 as u64, i.1.taskname.trim());
                    println!(" {} ", String::from("-").repeat(42));
                }
            }
        }
    }

    pub fn finish_task(&mut self, id: u64) {
        let mut pos: u64 = 0;
        for i in self.tasks.iter_mut() {
            if pos == id {
                *i = app::Item::clone(app::Item::modify(i.id, i.taskname.clone(), true));
                self.task_finished += 1;
                self.task_pending -= 1;
            }
            pos += 1;
        }
    }
}
