use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

/// A simple Spin HTTP component.
#[http_component]
fn handle_serverless_apis(req: Request) -> Result<Response> {
    println!("{:?}", req.headers());
    Ok(http::Response::builder()
        .status(200)
        .header("foo", "bar")
        .body(Some("Hello, Fermyon".into()))?)
}

// TODO - build out actual fermyon handler endpoints

// Right now these are just some random funtions that
// get and set a document in an Aerospike database

#[macro_use]
extern crate aerospike;

use std::env;
use std::sync::Arc;
use std::thread;
use std::time::Instant;

use aerospike::{operations, Record};
use aerospike::{Bins, Client, ClientPolicy, ReadPolicy, WritePolicy};

fn build_db_client() -> Arc<Client> {
    let cpolicy = ClientPolicy::default();
    let hosts = env::var("AEROSPIKE_HOSTS").unwrap_or(String::from("127.0.0.1:3000"));
    let client = Client::new(&cpolicy, &hosts).expect("Failed to connect to cluster");
    let client = Arc::new(client);

    client
}

enum FileType {
    JPG,
    PNG,
    SVG,
}

fn read_img(
    db_client: Arc<Client>,
    org_name: &str,
    file_name: &str,
    file_type: FileType,
) -> Result<Record, aerospike::Error> {
    // let client = client.clone();
    let rpolicy = ReadPolicy::default();
    let wpolicy = WritePolicy::default();

    let full_file_path = get_full_img_path(org_name, file_name, file_type);

    let key = as_key!("test", "test", 1);
    db_client.touch(&wpolicy, &key).unwrap();
    let rec = db_client.get(&rpolicy, &key, Bins::All);
    // println!("Record: {}", &rec);

    rec
}

fn insert_img(
    db_client: Arc<Client>,
    org_name: &str,
    file_name: &str,
    file_type: FileType,
) -> Result<Record, aerospike::Error> {
    let rpolicy = ReadPolicy::default();
    let wpolicy = WritePolicy::default();

    let full_file_path = get_full_img_path(org_name, file_name, file_type);

    let bins = [as_bin!("int", 123), as_bin!("str", "Hello, World!")];

    let key = as_key!("test", "test", 1);
    db_client.put(&wpolicy, &key, &bins).unwrap();
    let rec = db_client.get(&rpolicy, &key, Bins::All);
    // println!("Record: {}", rec.unwrap());

    rec
}

fn get_full_img_path(org_name: &str, file_name: &str, file_type: FileType) -> String {
    let file_type_string = match file_type {
        FileType::JPG => "jpg",
        FileType::PNG => "jpg",
        FileType::SVG => "jpg",
    };

    format!("{org_name}__{file_name}.{file_type_string}")
}

// let rec = client.get(&rpolicy, &key, Bins::None);
// println!("Record Header: {}", rec.unwrap());
// }

//     let mut threads = vec![];
//     let now = Instant::now();
//     for i in 0..2 {
//         let t = thread::spawn(move || {
//             let bins = [
//                 as_bin!("int", 123),
//                 as_bin!("str", "Hello, World!"),
//             ];

//             client.put(&wpolicy, &key, &bins).unwrap();
//             let rec = client.get(&rpolicy, &key, Bins::All);
//             println!("Record: {}", rec.unwrap());

//             let exists = client.exists(&wpolicy, &key).unwrap();
//             println!("exists: {}", exists);

//             let bin = as_bin!("int", 999);
//             let ops = &vec![operations::put(&bin), operations::get()];
//             let op_rec = client.operate(&wpolicy, &key, ops);
//             println!("operate: {}", op_rec.unwrap());

//             let existed = client.delete(&wpolicy, &key).unwrap();
//             println!("existed (sould be true): {}", existed);

//             let existed = client.delete(&wpolicy, &key).unwrap();
//             println!("existed (should be false): {}", existed);
//         });

//         threads.push(t);
//     }

//     for t in threads {
//         t.join().unwrap();
//     }

//     println!("total time: {:?}", now.elapsed());
// }
