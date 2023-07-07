use typedb_client::{
    concept::{Attribute, Concept, Value},
    Connection, DatabaseManager, Session,
    SessionType::{Data, Schema},
    TransactionType::{Read, Write}
};
use futures::{StreamExt};
use  std::fs;
use text_io::read;

const TEST_DATABASE: &str = "codeforces-data-model";
 
fn new_core_connection() -> typedb_client::Result<Connection> {
    Connection::new_plaintext("localhost:1729")
}

async fn load_data(connection: Connection)->std::io::Result<()>{
    let data=fs::read_to_string("./src/data.tql")?;    
    let databases = DatabaseManager::new(connection.clone());
    let session = Session::new(databases.get(TEST_DATABASE).await.unwrap(), Data).await.unwrap();
    let transaction = session.transaction(Write).await.unwrap();
    let _ = transaction.query().insert(data.as_str());
    transaction.commit().await.unwrap();
    println!("\nData Loaded Successfully\n");
    Ok(())
}

async fn load_schema(connection: Connection)->std::io::Result<()>{
    let schema = fs::read_to_string("./src/schema.tql")?;
    let databases = DatabaseManager::new(connection.clone());
    if databases.contains(TEST_DATABASE).await.unwrap()==false {
        let _ = databases.create(TEST_DATABASE).await;
        // define schema
        let session = Session::new(databases.get(TEST_DATABASE).await.unwrap(), Schema).await.unwrap();
        let transaction = session.transaction(Write).await.unwrap();
        transaction.query().define(schema.as_str()).await.unwrap();
        transaction.commit().await.unwrap();
        session.force_close().unwrap();
        load_data(connection.clone()).await?;
        println!("\nSchema Defined Successfully\n");
    }else {
        println!("\nSchema Already Defined\n");
    }

    Ok(())
}

async fn query1(connection: Connection)->std::io::Result<()>{    
    let databases = DatabaseManager::new(connection.clone());
    let session = Session::new(databases.get(TEST_DATABASE).await.unwrap(), Data).await.unwrap();
    let transaction = session.transaction(Read).await.unwrap();
    println!("Query chosen : Get names of all coders with rating >= x");
    println!("Choose rating x");
    let x: String=read!("{}\n");
    let query = format!("match $cc isa coder, has handle $p, has rating >= {x}; get $p;");

    println!("{}", query);

    let mut answer_stream = transaction.query().match_(&query.as_str()).unwrap();
    while let Some(result) = answer_stream.next().await{
        match result {
            Ok(concept_map) => {
                for (_, concept) in concept_map {
                    if let Concept::Attribute(Attribute { value: Value::String(value), .. }) = concept {
                        println!("{}",value);
                    }
                }
            }
            Err(err) => {
                panic!("An error occurred fetching answers of a Match query: {err}")
            }
        }
    }

    Ok(())
}

async fn query2(connection: Connection)->std::io::Result<()>{    
    let databases = DatabaseManager::new(connection.clone());
    let session = Session::new(databases.get(TEST_DATABASE).await.unwrap(), Data).await.unwrap();
    let transaction = session.transaction(Read).await.unwrap();
    println!("Query chosen : Get IDs of problems with a particular tag");
    println!("Choose tag");
    let tag: String=read!("{}\n");
    let query = format!("match $tt ($x, $y) isa possesses-tag; $x isa problem, has problem-number $a; $y isa topic, has topic-name \"{tag}\"; get $a;");

    println!("{}", query);

    let mut answer_stream = transaction.query().match_(&query.as_str()).unwrap();
    while let Some(result) = answer_stream.next().await{
        match result {
            Ok(concept_map) => {
                for (_, concept) in concept_map {
                    if let Concept::Attribute(Attribute { value: Value::String(value), .. }) = concept {
                        println!("{}",value);
                    }
                }
            }
            Err(err) => {
                panic!("An error occurred fetching answers of a Match query: {err}")
            }
        }
    }

    Ok(())
}

async fn query3(connection: Connection)->std::io::Result<()>{    
    let databases = DatabaseManager::new(connection.clone());
    let session = Session::new(databases.get(TEST_DATABASE).await.unwrap(), Data).await.unwrap();
    let transaction = session.transaction(Read).await.unwrap();
    println!("Query chosen : Get problem-name of problems with a particular tag with rating >= x");
    println!("Choose rating x");
    let x: String=read!("{}\n");
    println!("Choose tag");
    let tag: String=read!();
    let query = format!("match $p isa problem, has problem-name $m, has rating >= {x}; $y isa topic, has topic-name \"{tag}\"; $tt ($p, $y) isa possesses-tag; get $m;");
    println!("{}", query);

    let mut answer_stream = transaction.query().match_(&query.as_str()).unwrap();
    while let Some(result) = answer_stream.next().await{
        match result {
            Ok(concept_map) => {
                for (_, concept) in concept_map {
                    if let Concept::Attribute(Attribute { value: Value::String(value), .. }) = concept {
                        println!("{}",value);
                    }
                }
            }
            Err(err) => {
                panic!("An error occurred fetching answers of a Match query: {err}")
            }
        }
    }

    Ok(())
}

async fn run_query(connection: Connection){
    println!("Welcome to the codeforces data model project. Please choose what kind of query you would like to make from the following options");
    println!("Please select your entry using the number of the query");
    println!("Options");
    println!("1) Get names of all coders with rating >= x");
    println!("2) Get IDs of problems with a particular tag");
    println!("3) Get problem-name of problems with a particular tag with rating >= x");
    let qtype: i32=read!();
    match qtype {
        1 => query1(connection).await.unwrap(),
        2 => query2(connection).await.unwrap(),
        3 => query3(connection).await.unwrap(),
        _ => println!("Retry, invalid query option chosen")
    };
}

#[tokio::main]
async fn main()->std::io::Result<()>{
    let con=new_core_connection().expect(line!().to_string().as_str());
    load_schema(con.clone()).await?;
    run_query(con.clone()).await;

    Ok(())
}