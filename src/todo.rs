use crate::tasks::Tasks;

pub struct TodoList{
    list: Vec<(String,Tasks)>,
}

impl TodoList {
    pub fn new()->TodoList{
        TodoList{
            list: Vec::new(),
        }
    }
    
    pub fn count_tasks_in_list(&self, index:usize)->Option<usize>{
        if index < self.list.len() {
            // Get the Tasks instance at the specified index
            let (_, tasks) = &self.list[index];
            Some(tasks.count_tasks()) // Return the task count
        } else {
            None // Return None if the index is out of bounds
        }
    }

    pub fn add_list(&mut self, list_name: String){
        self.list.push((list_name,Tasks::new()));
    }

    pub fn add_task(&mut self, list_name: &str, description:String ) -> Result<(),String>{
        if let Some((_, todolist)) = self.list.iter_mut().find(|(name, _)| name == list_name){
            todolist.add_task(description);
            Ok(())
        } else {
            Err(format!("Task list '{}' does not exist",list_name))
        }
    }

    pub fn remove_task(&mut self, list_name: &str, index: usize) -> Result<(), String> {
        if let Some((_, todolist)) = self.list.iter_mut().find(|(name, _)| name == list_name){
            if todolist.delete_task(index).is_some() {
                Ok(())
            } else {
                Err(format!("Task at index {} does not exist in '{:?}'", index, todolist))
            }
        } else {
            Err(format!("Task list '{}' does not exist", list_name))
        }
    }

    pub fn remove_list(&mut self, list_name: &str) -> Result<(), String> {
        if let Some(index) = self.list.iter().position(|(name, _)| name == list_name) {
            self.list.remove(index);
            Ok(())
        }else{
            Err(format!("Task list '{}' does not exist",list_name))
        }
    }

    pub fn delete_all_list(& mut self){
        self.list.clear();
        assert!(self.list.is_empty());
    }
}