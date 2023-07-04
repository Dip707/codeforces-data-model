use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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
    let data=fs::read_to_string("./src/data.tql")?;

    let con=new_core_connection().expect("Line: 74");
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

    if let Ok(lines) = read_lines("./src/queries.tql") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let ch = ip.chars().next().unwrap();
                if ch == 'm'{
                    let answer_stream = transaction.query().match_(ip.as_str())?;
                }
            }
        }
    }

    // insert data
    let transaction = session.transaction(Write).await.unwrap();
    let _ = transaction.query().insert(data.as_str());
    transaction.commit().await.unwrap();

    Ok(())
}

#[tokio::main]
async fn main(){
    // let r=mymain();
    // block_on(r);
    mymain().await;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
