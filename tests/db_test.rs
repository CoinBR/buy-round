// use rust_test::setup;
// use rust_test::teardown;
// use rust_test::test;
//
//
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     let client = Client::default();
//
//     // Assume `streams` is a Vec<String> containing the names of all streams to delete.
//     let streams: Vec<String> = vec!["stream1".to_string(), "stream2".to_string()];
//
//     for stream in streams {
//         hard_delete_stream(&client, &stream).await?;
//     }
//
//     Ok(())
// }
//
//
// fn cleanup_database() {
//     // Delete all records from the relevant tables
//     // ...
// }
//
// fn seed_database() {
//     // Insert initial data into the relevant tables
//     // ...
// }
//
//
//
//
//
// #[setup]
// fn setup() {
//     cleanup_database();
//     seed_database();
// }
//
// pub fn run_tests() {
//     setup();
//     // Run your tests here
//     // ...
//     teardown();
// }
