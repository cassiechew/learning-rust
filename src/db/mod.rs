use self::db::{get_all_todos, get_one_todo, create_todo, update_todo, delete_todo};

use crate::todos::{Todo};

mod db;

pub fn get_all() -> redis::RedisResult<Vec<Todo>> {
  get_all_todos()
}

pub fn get_one(key: String) -> redis::RedisResult<Todo> {
  get_one_todo(key)
}

pub fn create(todo: Todo) -> redis::RedisResult<bool> {
  create_todo(todo)
}

pub fn update(todo: Todo) -> redis::RedisResult<bool> {
  update_todo(todo)
}

pub fn delete(id: String) -> redis::RedisResult<()> {
  delete_todo(id)
}
