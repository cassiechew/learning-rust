extern crate redis;
use redis::{Commands, RedisResult};

use crate::todos::Todo;

pub(crate) fn get_all_todos() -> redis::RedisResult<Vec<Todo>> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  con.keys("*")
}

pub(crate) fn get_one_todo(key: String) -> redis::RedisResult<Todo> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;
  
  let redis_res = con.get(key);
  let todo_string: String = redis_res.unwrap();

  let todo_obj: Todo = serde_json::from_str(&todo_string).unwrap();

  Ok(todo_obj)
}

pub(crate) fn create_todo(todo: Todo) -> RedisResult<bool> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  let todo_value = serde_json::to_string(&todo).unwrap();
  let _redis_res: RedisResult<String> = con.set(todo.id, todo_value);

  Ok(true)
}

pub(crate) fn update_todo(todo: Todo) -> RedisResult<bool> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  let todo_value = serde_json::to_string(&todo).unwrap();
  let _redis_res: RedisResult<String> = con.set(todo.id, todo_value);

  Ok(true)
}

pub(crate) fn delete_todo(id: String) -> RedisResult<()> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  con.del(id)
}
