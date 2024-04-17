// clipboard_history.rs
use clipboard::{ClipboardContext, ClipboardProvider};
use rusqlite::Result;
use std::env;
mod db;
mod ui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let conn = db::init_db()?;
    if args.iter().any(|s|s=="save") {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        let clipboard_content = ctx.get_contents()?;

        // 存储剪贴板内容到数据库
        db::save_or_update_clipboard_content(&conn, &clipboard_content)?;
    } else {
        let history = db::get_history(&conn)?;
        for record in history.clone().iter() {
            println!("record===>,{}", record)
        }
        ui::show(history);
    }

    Ok(())
}
