use cursive::{
    align::HAlign,
    event::Key,
    traits::*,
    view::{Nameable, Resizable},
    views::{Button, Dialog, EditView, LinearLayout, NamedView, TextView},
    Cursive,
};

use std::{
    process::Command,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
};

use cursive_tabs::{Align, TabPanel, TabView};

use crate::tasks::Task;
use crate::todo::TodoList;

pub fn tui_run() {
    let mut todo_list = Arc::new(Mutex::new(TodoList::new()));

    //Creating a new Cursive instance
    let mut siv = cursive::default();

    //Creating a new TabPanel instance
    let tab_panel = TabPanel::new()
        .with_tab(TextView::new(" Program Under-construction").with_name("+"))
        .with_bar_alignment(Align::Start)
        .with_bar_placement(cursive_tabs::Placement::VerticalLeft)
        .with_active_tab("+")
        .unwrap_or_else(|_| {panic!("Could not set the first tab as active tab! This is probably an issue with the implementation in the lib. Please report!")});

    //Create an atomic Referernce Counter to hold the number of tabs
    let tab_counter = Arc::new(AtomicUsize::new(1));
    // Create a vector to track the available tab numbers
    let available_tab_ids = Arc::new(std::sync::Mutex::new(Vec::new()));

    //Define TUI structure
    siv.add_layer(
        Dialog::new()
            .title("Tuitl")
            .title_position(HAlign::Right)
            .padding_lrtb(1, 1, 1, 1)
            .content(
                LinearLayout::vertical()
                    .child(tab_panel.with_name("Tabs").fixed_size((100, 50)))
                    .child(
                        LinearLayout::horizontal()
                            .child(Button::new("Prev", |siv| {
                                let mut tabs: cursive::views::ViewRef<TabPanel> =
                                    siv.find_name("Tabs").expect("id not found");
                                tabs.prev();
                            }))
                            .child(Button::new("Next", |siv| {
                                let mut tabs: cursive::views::ViewRef<TabPanel> =
                                    siv.find_name("Tabs").expect("id not found");
                                tabs.next();
                            }))
                            .child(Button::new("Switch", |siv| {
                                let mut tabs: cursive::views::ViewRef<TabPanel> =
                                    siv.find_name("Tabs").expect("id not found");
                                tabs.swap_tabs("1", "2");
                            })),
                    ),
            ),
    );

    siv.add_global_callback('q', |s| s.quit());

    // Handle adding new tabs
    let tab_counter_add = Arc::clone(&tab_counter);
    let available_add = Arc::clone(&available_tab_ids);
    siv.add_global_callback('a', move |s| {
        let mut counter = tab_counter_add.fetch_add(1, Ordering::SeqCst);
        create_new_tab(
            s,
            &mut counter,
            Arc::clone(&available_add),
            Arc::clone(&todo_list),
        );
    });

    // Handle deleting tabs
    let tab_counter_del = Arc::clone(&tab_counter);
    let available_del = Arc::clone(&available_tab_ids);
    siv.add_global_callback('d', move |s| {
        let mut counter = tab_counter_del.fetch_add(1, Ordering::SeqCst);
        delete_tab(s, &mut counter, Arc::clone(&available_del));
    });

    siv.run();
}

// Function to create a new tab when on the '+' tab
fn create_new_tab(
    siv: &mut Cursive,
    tab_counter: &mut usize,
    available_tab_ids: Arc<Mutex<Vec<usize>>>,
    todo_lists: Arc<Mutex<TodoList>>,
) {
    let tabs: cursive::views::ViewRef<TabPanel> =
        siv.find_name("Tabs").expect("TabPanel not found");

    //check if the current active tab is the '+'
    if tabs.active_tab() == Some("+") {
        let mut available_tabs = available_tab_ids.lock().unwrap_or_else(|_| {
            panic!("unable to acquire lock on active");
        });

        let new_tab_name = if let Some(recycled_id) = available_tabs.pop() {
            recycled_id.to_string()
        } else {
            tab_counter.to_string()
        };

        let todo_lists_c = Arc::clone(&todo_lists); // Clone the Arc for use in the closure

        //show dialog to get tab connect
        siv.add_layer(
            Dialog::new()
                .title("New Todolist")
                .title_position(HAlign::Left)
                .padding_lrtb(1, 1, 1, 1)
                .content(EditView::new().with_name("tab_content").fixed_width(20))
                .button("Ok", move |s| {
                    let content = s
                        .call_on_name("tab_content", |view: &mut EditView| view.get_content())
                        .unwrap_or_else(|| {
                            panic!("Failed to call on_name, add tab content");
                        });

                    if content.is_empty() {
                        s.add_layer(Dialog::info("List name is required!"));
                    } else {
                        // Remove the dialog
                        s.pop_layer();

                        let mut todo_lists = todo_lists_c.lock().unwrap();
                        todo_lists.add_list(content.to_string());

                        // Add the new tab with the content
                        s.call_on_name("Tabs", |tab: &mut TabPanel| {
                            tab.add_tab(NamedView::new(
                                &new_tab_name,
                                TextView::new(content.as_str()).with_name(&new_tab_name),
                            ));
                            tab.set_active_tab(&new_tab_name)
                                .expect("Failed to switch to new tab");
                        })
                        .unwrap_or_else(|| {
                            panic!("Failed to create new tab");
                        });
                        println!("Created new tab: {}", new_tab_name);
                    }
                }),
        );

        // Increment the tab counter
        *tab_counter += 1;
    }
}

fn add_task_to_list(list: Arc<Mutex<TodoList>>, siv: &mut Cursive) {
    let tabs: cursive::views::ViewRef<TabPanel> =
        siv.find_name("Tabs").expect("TabPanel not found");

    if tabs.active_tab() != Some("+") {
        let active_tab = tabs.active_tab().map(|tab| tab.to_string());
        let list_c = list.clone();
        let active_tab_c = active_tab.unwrap().clone();

        siv.add_layer(
            Dialog::new()
                .title(format!("{}: New Task", active_tab_c))
                .title_position(HAlign::Left)
                .content(
                    LinearLayout::vertical().child(
                        EditView::new()
                            .with_name("task_description")
                            .fixed_size((100, 50)),
                    ),
                )
                .button("Save", move |s| {
                    let task_content = s
                        .call_on_name("task_description", |view: &mut EditView| view.get_content())
                        .unwrap_or_else(|| {
                            panic!("Failed to call on_name, getting task description");
                        });

                    if task_content.is_empty() {
                        s.add_layer(Dialog::info(
                            "Please provide a description or click the <Cancel> button",
                        ));
                    } else {
                        s.pop_layer();

                        let mut list = list_c.lock().unwrap_or_else(|_| {
                            panic!("lock failed");
                        });

                        list.add_task(active_tab_c.as_str(), task_content.to_string())
                            .expect("Error adding task");
                    }
                })
                .h_align(HAlign::Left)
                .button("Cancel", |s| {
                    s.pop_layer();
                })
                .h_align(HAlign::Right),
        );
    }
}

fn edit_list_tasks(siv: &mut Cursive, list: Arc<Mutex<TodoList>>, task_name: String){

}

fn mark_list_tasks(siv: &mut Cursive, list: Arc<Mutex<TodoList>>, task_name: String){
    
}

fn delete_tab(
    siv: &mut Cursive,
    tab_counter: &mut usize,
    available_tab_ids: Arc<Mutex<Vec<usize>>>,
) {
    let mut tabs: cursive::views::ViewRef<TabPanel> =
        siv.find_name("Tabs").expect("TabPanel not found");

    // Ensure the TabPanel is mutable to allow modifications
    if let Some(active_tab) = tabs.active_tab() {
        let active_tab_name = active_tab.to_string();

        if active_tab_name != "+" {
            tabs.remove_tab(&active_tab_name)
                .expect("Failed to remove tab");

            // Add the deleted tab's id to the available pool for reuse
            let mut available_tabs = available_tab_ids.lock().unwrap();
            available_tabs.push(active_tab_name.parse::<usize>().unwrap());

            *tab_counter -= 1;

            // Set the active tab to "+" if there are still other tabs
            if *tab_counter > 0 {
                let _ = tabs.set_active_tab("+");
            }

            println!("{active_tab_name} deleted");
        }
    } else {
        println!("No active tab found");
    }
}

fn open_terminal() {
    if cfg!(target_os = "windows") {
        // For Windows, use PowerShell to open a new window
        Command::new("powershell")
            .args(&["-NoExit", "-Command", "cargo run"])
            .spawn()
            .expect("Failed to open PowerShell");
    } else if cfg!(target_os = "linux") {
        // For Linux, use gnome-terminal or xterm to open a new window
        Command::new("gnome-terminal")
            .arg("--")
            .arg("cargo")
            .arg("run")
            .spawn()
            .expect("Failed to open gnome-terminal");
    } else if cfg!(target_os = "macos") {
        // For macOS, use open -a Terminal to open a new terminal window
        Command::new("open")
            .arg("-a")
            .arg("Terminal")
            .arg("cargo run")
            .spawn()
            .expect("Failed to open Terminal");
    } else {
        panic!("Unsupported platform!");
    }
}
