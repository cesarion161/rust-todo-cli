#[derive(Default)]
pub struct TodoList {
    items: Vec<String>,
}

impl TodoList {
    pub fn add_todo_item(&mut self, item: &str) {
        self.items.push(item.to_string());
        println!("Added item: {}", item)
    }
    //
    pub fn list(&self) -> &Vec<String> {
        if self.items.is_empty() {
            println!("No to-do items found.");
        } else {
            for (index, item) in self.items.iter().enumerate() {
                println!("{}. {}", index + 1, item);
            }
        }
        &self.items
    }
}