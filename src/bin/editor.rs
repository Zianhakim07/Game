use chrono::Utc;
use game::database::Database;

fn main() {
    let db = Database::open();
    let mut profile = db.from_username("test").unwrap();
    profile.data.points = 5500;
    profile.data.items.clear();
    profile.data.next = Utc::now();
    db.save_profile(&profile)
}
