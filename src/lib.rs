#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Client {
    pub id: String,
    pub name: String,
    pub active: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub client_id: String,
    pub title: String,
    pub due_day: u32,
    pub completed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProductHolding {
    pub client_id: String,
    pub product: String,
    pub review_day: u32,
}

#[derive(Debug, Default)]
pub struct Workspace {
    pub clients: Vec<Client>,
    pub tasks: Vec<Task>,
    pub holdings: Vec<ProductHolding>,
}

impl Workspace {
    pub fn due_queue(&self, through_day: u32) -> Vec<&Task> {
        let mut tasks: Vec<_> = self
            .tasks
            .iter()
            .filter(|task| !task.completed && task.due_day <= through_day)
            .filter(|task| {
                self.clients
                    .iter()
                    .any(|client| client.id == task.client_id && client.active)
            })
            .collect();
        tasks.sort_by_key(|task| (task.due_day, task.client_id.as_str(), task.title.as_str()));
        tasks
    }

    pub fn reviews_due(&self, through_day: u32) -> Vec<&ProductHolding> {
        let mut holdings: Vec<_> = self
            .holdings
            .iter()
            .filter(|holding| holding.review_day <= through_day)
            .collect();
        holdings.sort_by_key(|holding| (holding.review_day, holding.client_id.as_str()));
        holdings
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diary_excludes_completed_and_inactive_client_work() {
        let workspace = Workspace {
            clients: vec![
                Client {
                    id: "C1".into(),
                    name: "Synthetic".into(),
                    active: true,
                },
                Client {
                    id: "C2".into(),
                    name: "Dormant".into(),
                    active: false,
                },
            ],
            tasks: vec![
                Task {
                    client_id: "C1".into(),
                    title: "Review".into(),
                    due_day: 10,
                    completed: false,
                },
                Task {
                    client_id: "C2".into(),
                    title: "Skip".into(),
                    due_day: 5,
                    completed: false,
                },
            ],
            holdings: vec![],
        };
        assert_eq!(workspace.due_queue(10)[0].title, "Review");
        assert_eq!(workspace.due_queue(10).len(), 1);
    }
}
