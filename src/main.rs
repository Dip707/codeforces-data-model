use typedb_client::{
    concept::{Attribute, Concept, Value},
    Connection, DatabaseManager, Session,
    SessionType::{Data, Schema},
    TransactionType::{Read, Write}
};
use futures::StreamExt;
use futures::TryStreamExt;
use  std::fs;
use std::io;

#[derive(Debug)]
enum HandleError {
    Io(io::Error),
    TypeDB(typedb_client::error::Error),
}

const TEST_DATABASE: &str = "codeforces-data-model";
 
fn new_core_connection() -> typedb_client::Result<Connection> {
    Connection::new_plaintext("localhost:1729")
}

fn read_input() -> Result<String, HandleError>{
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).map_err(HandleError::Io)?;
    input_string.pop();
    Ok(input_string)
}

async fn load_data(connection: Connection)->Result<(), HandleError>{
    let data=fs::read_to_string("./src/data.tql").map_err(HandleError::Io)?;    
    let databases = DatabaseManager::new(connection.clone());
    let session = Session::new(databases.get(TEST_DATABASE).await.map_err(HandleError::TypeDB)?, Data).await.map_err(HandleError::TypeDB)?;
    let transaction = session.transaction(Write).await.map_err(HandleError::TypeDB)?;
    let inserted_query = transaction.query().insert(data.as_str());
    let _ = inserted_query.unwrap().try_collect::<Vec<_>>().await;
    transaction.commit().await.map_err(HandleError::TypeDB)?;
    println!("\nData Loaded Successfully\n");
    Ok(())
}

async fn load_schema_and_data(connection: Connection)->Result<(), HandleError>{
    let schema = fs::read_to_string("./src/schema.tql").map_err(HandleError::Io)?;
    let databases = DatabaseManager::new(connection.clone());
    if databases.contains(TEST_DATABASE).await.map_err(HandleError::TypeDB)? {
        println!("\nSchema Already Defined\n");
    }else {
        databases.create(TEST_DATABASE).await.map_err(HandleError::TypeDB)?;
        // define schema
        let session = Session::new(databases.get(TEST_DATABASE).await.map_err(HandleError::TypeDB)?, Schema).await.map_err(HandleError::TypeDB)?;
        let transaction = session.transaction(Write).await.map_err(HandleError::TypeDB)?;
        transaction.query().define(schema.as_str()).await.map_err(HandleError::TypeDB)?;
        transaction.commit().await.map_err(HandleError::TypeDB)?;
        drop(session);
        println!("\nSchema Defined Successfully\n");
        load_data(connection.clone()).await?;
    }

    Ok(())
}

async fn query1(connection: Connection)->Result<(), HandleError>{
    let databases = DatabaseManager::new(connection.clone());
    let session = Session::new(databases.get(TEST_DATABASE).await.map_err(HandleError::TypeDB)?, Data).await.map_err(HandleError::TypeDB)?;
    let transaction = session.transaction(Read).await.map_err(HandleError::TypeDB)?;
    println!("Query chosen : Get names of all coders with rating >= x");
    println!("Choose rating x");
    let x = read_input()?;
    let query = format!("match $cc isa coder, has handle $p, has rating >= {x}; get $p;");

    let mut answer_stream = transaction.query().match_(&query.as_str()).map_err(HandleError::TypeDB)?;
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

async fn query2(connection: Connection)->Result<(), HandleError>{
    let databases = DatabaseManager::new(connection.clone());
    let session = Session::new(databases.get(TEST_DATABASE).await.map_err(HandleError::TypeDB)?, Data).await.map_err(HandleError::TypeDB)?;
    let transaction = session.transaction(Read).await.map_err(HandleError::TypeDB)?;
    println!("Query chosen : Get IDs of problems with a particular tag");
    println!("Choose tag");
    let tag = read_input()?;
    let query = format!("match $tt ($x, $y) isa possesses-tag; $x isa problem, has problem-number $a; $y isa topic, has topic-name \"{tag}\"; get $a;");

    let mut answer_stream = transaction.query().match_(&query.as_str()).map_err(HandleError::TypeDB)?;
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
                return Err(HandleError::TypeDB(err));
            }
        }
    }

    Ok(())
}

async fn query3(connection: Connection)->Result<(), HandleError>{
    let databases = DatabaseManager::new(connection.clone());
    let session = Session::new(databases.get(TEST_DATABASE).await.map_err(HandleError::TypeDB)?, Data).await.map_err(HandleError::TypeDB)?;
    let transaction = session.transaction(Read).await.map_err(HandleError::TypeDB)?;
    println!("Query chosen : Get problem-name of problems with a particular tag with rating >= x");
    println!("Choose rating x");
    let x = read_input()?;
    println!("Choose tag");
    let tag = read_input()?;
    let query = format!("match $p isa problem, has problem-name $m, has rating >= {x}; $y isa topic, has topic-name \"{tag}\"; $tt ($p, $y) isa possesses-tag; get $m;");

    let mut answer_stream = transaction.query().match_(&query.as_str()).map_err(HandleError::TypeDB)?;
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
                return Err(HandleError::TypeDB(err));
            }

        }
    }

    Ok(())
}

async fn run_query(connection: Connection)->Result<(), HandleError>{
    println!("Welcome to the codeforces data model project. Please choose what kind of query you would like to make from the following options");
    println!("Please select your entry using the number of the query");
    println!("Options");
    println!("1) Get names of all coders with rating >= x");
    println!("2) Get IDs of problems with a particular tag");
    println!("3) Get problem-name of problems with a particular tag with rating >= x");
    let qtype =read_input()?;
    match qtype.as_str() {
        "1" => query1(connection).await?,
        "2" => query2(connection).await?,
        "3" => query3(connection).await?,
        _ => println!("Retry, invalid query option chosen")
    };

    Ok(())
}

#[tokio::main]
async fn main()->Result<(), HandleError>{
    let con=new_core_connection().expect(line!().to_string().as_str());
    load_schema_and_data(con.clone()).await?;
    run_query(con.clone()).await?;

    Ok(())
}