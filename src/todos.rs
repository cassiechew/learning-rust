use redis::{FromRedisValue, Value, RedisResult, from_redis_value, Commands};
use tide::{Request, Response};
use serde::{Deserialize, Serialize};

use crate::db::{get_all, get_one, create, update, delete};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Todo {
  pub id: String,
  pub name: String,
  pub desc: String,
}

// No idea how this works. Probably from my lack of using redis experience hahhaha
impl FromRedisValue for Todo {
  fn from_redis_value(v: &Value) -> RedisResult<Todo> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    let v: String = from_redis_value(v)?;
    println!("{}", v);

    let redis_res = con.get(v);
    let todo_string: String = redis_res.unwrap();

    let todo_obj: Todo = serde_json::from_str(&todo_string).unwrap();

    println!("{}", todo_obj.desc);

    Ok(todo_obj)
  }
}

pub async fn get_all_todos(_req: Request<()>) -> tide::Result {
  let res = get_all()?;
  Ok(format!("{:?}", res).into())
}

pub async fn get_todo(req: Request<()>) -> tide::Result {
  let key = req.param("id")?;
  let res = get_one(key.to_string())?;
  print!("{}", res.name);

  Ok(format!("Here is todo id {} | name {}.", res.id, res.name).into())
}

pub async fn create_todo(mut req: Request<()>) -> tide::Result {
  let todo: Todo = req.body_json().await?;
  let res = create(todo)?;

  let mut response: Response = Response::new(if res {200} else {500});
  if res {response.set_body("success!")} else {response.set_body("failed!")};
  Ok(response)
}

pub async fn update_todo(mut req: Request<()>) -> tide::Result {
  let todo: Todo = req.body_json().await?;
  let res = update(todo)?;
  
  let mut response: Response = Response::new(if res {200} else {500});
  if res {response.set_body("success!")} else {response.set_body("failed!")};
  Ok(response)
}

pub async fn delete_todo(mut req: Request<()>) -> tide::Result {
  let Todo { id, name: _, desc: _ } = req.body_json().await?;
  let _ = delete(id.to_string())?;
  Ok(format!("Hello, I've deleted").into())
}

