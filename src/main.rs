mod todos;
mod db;

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();
    let mut app = tide::Server::new();

    app.at("/todo").get(todos::get_all_todos);
    app.at("/todo/:id").get(todos::get_todo);
    app.at("/todo").post(todos::create_todo);
    app.at("/todo").put(todos::update_todo);
    app.at("/todo").delete(todos::delete_todo);
    
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}


