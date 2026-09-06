use std::cell::RefCell;
use std::rc::Rc;

struct Database {
    max_connections: u32,
}

struct AppState {
    db: Rc<RefCell<Database>>,
}

fn main() {
    let db = Rc::new(RefCell::new(Database {
        max_connections: 10,
    }));
    let state = AppState { db: Rc::clone(&db) };
    let state2 = AppState { db: Rc::clone(&db) };

    db.borrow_mut().max_connections = 20;
    // Use the state in your application
}
