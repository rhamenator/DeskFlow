use desk_flow::{Client, Task, Workspace};

fn main() {
    let workspace = Workspace {
        clients: vec![Client {
            id: "C1".into(),
            name: "Synthetic Client".into(),
            active: true,
        }],
        tasks: vec![Task {
            client_id: "C1".into(),
            title: "Annual review".into(),
            due_day: 20,
            completed: false,
        }],
        holdings: vec![],
    };
    for task in workspace.due_queue(20) {
        println!("day {}: {}", task.due_day, task.title);
    }
}
