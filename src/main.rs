use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

fn main() {
    let manager = ConnectionManager::<PgConnection>::new("postgres://localhost/YOUR_DB_NAME");
    let _pool = Pool::builder()
        .min_idle(Some(4))
        .max_size(16)
        .build(manager)
        .unwrap();
}
