use anyhow::Error;
use arangors::Connection;

// exact same API except that you don't need to await
fn main() -> Result<(), Error> {
    let conn = Connection::establish_jwt("http://127.0.0.1:8529", "root", "test123")?;
    let database = conn.db("test_db")?;

    let collections = database.accessible_collections()?;
    println!("{:?}", collections);

    let collections = database.accessible_collections()?;
    println!("{:?}", collections);

    let info = database.info()?;
    println!("{:?}", info);

    Ok(())
}
