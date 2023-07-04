use typedb_client::{
    concept::{Attribute, Concept, Value},
    error::ConnectionError,
    Connection, DatabaseManager, Error, Options, Session,
    SessionType::{Data, Schema},
    TransactionType::{Read, Write},
    Credential, Database
};
use futures::executor::block_on;
use  std::fs;

const TEST_DATABASE: &str = "project-cf";
 
fn new_core_connection() -> typedb_client::Result<Connection> {
    Connection::new_plaintext("localhost:1729")
}

async fn mymain()->std::io::Result<()>{
    // load tql files
    let schema = fs::read_to_string("./src/schema.tql")?;
    // let data=fs::read_to_string("./src/data.tql")?;
    // let queries=fs::read_to_string("./src/queries.tql")?;

    let con=new_core_connection().expect("Line: 74");
    // query_options(con.clone()).await.unwrap();
    let databases = DatabaseManager::new(con);
    if databases.contains(TEST_DATABASE).await.unwrap()==false {
        databases.create(TEST_DATABASE).await;
        println!("Done");
    }
    println!("line:24");

    // define schema
    let session = Session::new(databases.get(TEST_DATABASE).await.unwrap(), Schema).await.unwrap();
    let transaction = session.transaction(Write).await.unwrap();
    transaction.query().define(schema.as_str()).await.unwrap();
    transaction.commit().await.unwrap();
    
    // insert data
    // let transaction = session.transaction(Write).await.unwrap();
    // let _ = transaction.query().insert(data.as_str());
    // transaction.commit().await.unwrap();

    Ok(())
}

#[tokio::main]
async fn main(){
    // let r=mymain();
    // block_on(r);
    mymain().await;
}
