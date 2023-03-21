use postgres::{Client, NoTls};
use redis;

fn init_pgsql_connection() -> Client {
    let mut client = Client::connect("host=localhost user=postgres password=postgres dbname=db", NoTls).expect("Could not connect to postgres");
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
        std::thread::sleep(std::time::Duration::from_millis(300));
        let mut conn = redis_client.get_connection().expect("Could not get redis connection");
        let vote = redis::cmd("LPOP")
            .arg("votes")
            .query::<String>(&mut conn)
            .expect("Could not get vote from redis");

        let parsed_vote: serde_json::Value = serde_json::from_str(&vote).expect("Could not parse vote as JSON");
        println!("Got vote: {:?}", vote);

        let voter_id = parsed_vote["voter_id"].as_str().expect("Could not get voter_id");
        let vote = parsed_vote["vote"].as_str().expect("Could not get vote");

        psql_client
            .execute(
                "INSERT INTO votes (id, vote) VALUES ($1, $2)",
                &[&voter_id, &vote],
            )
            .expect("Could not insert vote into postgres");
    }
}
