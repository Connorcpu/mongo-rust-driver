use mongo_driver::uri::Uri;
use mongo_driver::client::ClientPool;

#[test]
fn test_command() {
    let uri      = Uri::new("mongodb://localhost:27017/");
    let pool     = ClientPool::new(uri, None);
    let client   = pool.pop();
    let database = client.get_database("rust_test");

    let command = doc! { "ping" => 1 };

    let result = database.command(command, None).unwrap().next().unwrap().unwrap();
    assert!(result.contains_key("ok"));
}

#[test]
fn test_get_collection_and_name() {
    let uri      = Uri::new("mongodb://localhost:27017/");
    let pool     = ClientPool::new(uri, None);
    let client   = pool.pop();
    let database = client.get_database("rust_test");

    assert_eq!("rust_test", database.get_name().to_mut());

    let collection = database.get_collection("items");
    assert_eq!("items", collection.get_name().to_mut());
}

#[test]
fn test_create_collection() {
    let uri      = Uri::new("mongodb://localhost:27017/");
    let pool     = ClientPool::new(uri, None);
    let client   = pool.pop();
    let database = client.get_database("rust_test");
    database.get_collection("created_collection").drop().unwrap_or(());

    let collection = database.create_collection(
        "created_collection",
        None
    ).unwrap();

    assert_eq!("created_collection", collection.get_name().to_mut());
}