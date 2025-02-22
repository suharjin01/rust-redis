fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests{
    use redis::{aio::MultiplexedConnection, AsyncCommands, Client, Commands, RedisError};


    // Dengan menggunakan singkronus
    #[test]
    fn test_connection() {
        let mut client = Client::open("redis://localhost:6379/").unwrap();

        let _: () = client.set("name", "Suharjin").unwrap(); // variabel kosong
        let value: String = client.get("name").unwrap();

        println!("{}", value);
    }


    // Async Client
    
    // membuat fungsi
    async fn get_client() -> Result<MultiplexedConnection, RedisError> {
        let client = Client::open("redis://localhost:6379/")?;
        client.get_multiplexed_async_connection().await
    }

    #[tokio::test]
    async fn test_async_connection() -> Result<(), RedisError> {
        let mut con = get_client().await?;
        let _: () = con.set("name", "Aqil").await?;
        let value: String = con.get("name").await?;

        println!("{}", value);

        Ok(())
    }
}