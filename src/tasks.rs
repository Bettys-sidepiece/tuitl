#[derive(Debug)] 
pub struct Task{
    description: String,
    completed: bool,
    id: Option<i16>,
}
#[derive(Debug)] 
pub struct Tasks{
    tasks: Vec<Task>,
}

impl Tasks {
    /// Creates a new
    pub fn new() -> Tasks {
       Tasks { tasks: Vec::new() }
    }

    // Adds a task to the tasks vector
    pub fn add_task(&mut self, description: String){
        let tasks = Task {
            description,
            completed: false,
            id: None,
        };

        self.tasks.push(tasks);
    }
    
    pub fn completed_task(&mut self, index: usize){
        if index < self.tasks.len(){
            self.tasks[index].completed = true;
        }
    }

    pub fn delete_task(&mut self, index: usize)-> Option<Task>{
        if index < self.tasks.len(){
            Some(self.tasks.remove(index))
        }else{
            None
        }
    }

    pub fn list_tasks(&self, selected_index: Option<usize>) {
        for (i, task) in self.tasks.iter().enumerate() {
            let prefix = if Some(i) == selected_index { ">" } else { " " };
            let status = if task.completed { "[x]" } else { "[ ]" };
            println!("{} {}{}", prefix, status, task.description);
        }
    }

    pub fn edit_task(&mut self, index:usize, new_description:String)->Result<(),String>{
        if index < self.tasks.len(){
            self.tasks[index].description = new_description;
            Ok(())
        }else{
            Err(format!("Task index {} is out of range",index))
        }
    }

    pub fn reorder_task(&mut self, priority: usize, index: usize) -> Result<(), String> {
        // Check if both index and priority are within valid bounds
        if index >= self.tasks.len() || priority >= self.tasks.len() {
            return Err(format!(
                "Invalid index {} or priority {}. Task list has only {} tasks.",
                index,
                priority,
                self.tasks.len()
            ));
        }
        // Remove the task from the current index
        let task = self.tasks.remove(index);

        // Insert the task at the new priority position
        self.tasks.insert(priority, task);

        Ok(())
    }

    pub fn count_tasks(&self) -> usize {
        self.tasks.len()
    }
    
}