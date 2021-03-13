use barrel::{types, Migration};
use barrel::backend::MySql;

pub fn migration() -> String {
    let mut m = Migration::new();
    println!("Applying: {}", file!());

    m.create_table("todos", |t| {
        t.add_column("id", types::primary());
        t.add_column("title", types::varchar(255));
        t.add_column("is_completed", types::boolean().default(0));
    });

    m.make::<MySql>()
}
