pub use sea_orm_migration::prelude::*;

mod iden;
mod m20250613_000001_schema;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20250613_000001_schema::Migration)]
    }
}
