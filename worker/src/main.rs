use postgres::{Client, NoTls};
use redis;

fn init_pgsql_connection() -> Client {
    let mut client = Client::connect("host=db user=postgres password=postgres", NoTls).expect("Could not connect to postgres");
    client
        .batch_execute("CREATE TABLE IF NOT EXISTS votes (
            id VARCHAR(255) NOT NULL UNIQUE,
            vote VARCHAR(255) NOT NULL
        )")
        .expect("Could not create table");
    client
}

fn main() {
    println!("Hello, world!");
    let mut psql_client = init_pgsql_connection();
    let redis_client = redis::Client::open("redis://redis/").expect("Could not connect to redis");

    loop {
        std::thread::sleep(std::time::Duration::from_millis(500));
        let mut conn = redis_client.get_connection().expect("Could not get redis connection");
        let result = redis::cmd("LPOP")
            .arg("votes")
            .query::<Option<String>>(&mut conn)
            .expect("Could not get vote from redis");
        if result.is_none() {
            println!("No vote found, skipping");
            continue;
        }
        println!("Found vote");
        let vote = result.unwrap();

        let parsed_vote: serde_json::Value = serde_json::from_str(&vote).expect("Could not parse vote as JSON");
        println!("Got vote: {:?}", vote);

        let voter_id = parsed_vote["voter_id"].as_str().expect("Could not get voter_id");
        let choice = parsed_vote["vote"].as_str().expect("Could not get vote");
        println!("Got vote {} from {}", choice, voter_id);

        let rows = psql_client
            .execute(
                "INSERT INTO votes (id, vote) VALUES ($1, $2)",
                &[&voter_id, &choice],
            )
            .expect("Could not insert vote into postgres");
        println!("Modified {} rows", rows);
        println!("Inserted vote: {:?}", vote);
    }
}
