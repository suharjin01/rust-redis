fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests{
    use std::{num::NonZero, time::Duration};

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


    // String
    #[tokio::test]
    async fn test_string() -> Result<(), RedisError> {
        let mut con = get_client().await?;
        let _: () = con.set_ex("name", "Aqil", 2).await?;
        let value: String = con.get("name").await?;
        
        println!("{}", value);

        tokio::time::sleep(Duration::from_secs(5)).await;

        let value: Result<String, RedisError> = con.get("name").await;
        assert_eq!(true, value.is_err());

        Ok(())
    }


    // List
    #[tokio::test]
    async fn test_list() -> Result<(), RedisError> {
        let mut con = get_client().await?;

        let _: () = con.del("names").await?;
        let _: () = con.rpush("names", "Carongkong").await?;
        let _: () = con.rpush("names", "Wicok").await?;
        let _: () = con.rpush("names", "Wacok").await?;
        let _: () = con.rpush("names", "Waracik").await?;

        let len: i32 = con.llen("names").await?;
        assert_eq!(4, len);

        let names: Vec<String> = con.lrange("names", 0, -1).await?;
        assert_eq!(vec!["Carongkong", "Wicok", "Wacok", "Waracik"], names);

        let names: Vec<String> = con.lpop("names", NonZero::new(1)).await?;
        assert_eq!(vec!["Carongkong"], names);
        let names: Vec<String> = con.lpop("names", NonZero::new(1)).await?;
        assert_eq!(vec!["Wicok"], names);
        let names: Vec<String> = con.lpop("names", NonZero::new(1)).await?;
        assert_eq!(vec!["Wacok"], names);
        let names: Vec<String> = con.lpop("names", NonZero::new(1)).await?;
        assert_eq!(vec!["Waracik"], names);

        Ok(())
    }


    // Set
    #[tokio::test]
    async fn test_set() -> Result<(), RedisError> {
        let mut con = get_client().await?;

        let _: () = con.del("names").await?;
        let _: () = con.sadd("names", "Carongkong").await?;
        let _: () = con.sadd("names", "Carongkong").await?;
        let _: () = con.sadd("names", "Wicok").await?;
        let _: () = con.sadd("names", "Wicok").await?;
        let _: () = con.sadd("names", "Wacok").await?;
        let _: () = con.sadd("names", "Wacok").await?;
        let _: () = con.sadd("names", "Waracik").await?;
        let _: () = con.sadd("names", "Waracik").await?;

        let len: i32 = con.scard("names").await?;
        assert_eq!(4, len);

        let names: Vec<String> = con.smembers("names").await?;
        assert_eq!(vec!["Carongkong", "Wicok", "Wacok", "Waracik"], names);

        Ok(())
    }


    // Sorted Set
    #[tokio::test]
    async fn test_sorted_set() -> Result<(), RedisError> {
        let mut con = get_client().await?;

        let _: () = con.del("names").await?;
        let _: () = con.zadd("names", "Carongkong", 100).await?;
        let _: () = con.zadd("names", "Wicok", 10).await?;
        let _: () = con.zadd("names", "Wacok", 1).await?;
        let _: () = con.zadd("names", "Waracik", 50).await?;

        let len: i32= con.zcard("names").await?;
        assert_eq!(4, len);

        let names: Vec<String> = con.zrange("names", 0, -1).await?;
        assert_eq!(vec!["Wacok", "Wicok", "Waracik", "Carongkong"], names);
        
        Ok(())
    }
}