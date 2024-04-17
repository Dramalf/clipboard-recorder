// clipboard_history.rs
use rusqlite::{params, Connection, Result};
use std::time::{SystemTime, UNIX_EPOCH};

// 数据库路径
const DB_PATH: &str = "/Users/Shared/clipboard_history.db";

// 初始化数据库和表
pub fn init_db() -> Result<Connection> {
    let conn = Connection::open(DB_PATH)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS clipboard_history (
                  id          INTEGER PRIMARY KEY,
                  content     TEXT NOT NULL,
                  timestamp   INTEGER NOT NULL
              )",
        (),
    )?;
    Ok(conn)
}

// 存储剪贴板内容到数据库
pub fn save_or_update_clipboard_content(conn: &Connection, content: &str) -> Result<()> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    // 检查内容是否已经存在
    let mut stmt = conn.prepare("SELECT id FROM clipboard_history WHERE content = ?1")?;
    let mut rows = stmt.query(params![content])?;

    if let Some(row) = rows.next()? {
        // 如果存在，更新该条目的时间戳
        let id: i64 = row.get(0)?;
        conn.execute(
            "UPDATE clipboard_history SET timestamp = ?1 WHERE id = ?2",
            params![now, id],
        )?;
    } else {
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM clipboard_history ORDER BY timestamp DESC", [], |row| {
            row.get(0)
        })?;

        if count >= 12 {
            // 删除时间戳最早的记录
            conn.execute(
                "DELETE FROM clipboard_history WHERE id = (SELECT id FROM clipboard_history ORDER BY timestamp ASC LIMIT 1)",
                [],
            )?;
        }

        conn.execute(
            "INSERT INTO clipboard_history (content, timestamp) VALUES (?1, ?2)",
            params![content, now],
        )?;
    }
    print!("save_or_update_clipboard_content: {}", content);
    Ok(())
}

pub fn get_history(conn: &Connection) -> Result<Vec<String>> {
    // 请注意此处新增了conn参数，以传递数据库连接
    let mut stmt =
        conn.prepare("SELECT id, content, timestamp FROM clipboard_history ORDER BY id DESC")?;
    let clipboard_iter = stmt.query_map([], |row| Ok(row.get(1)?))?;

    // 将迭代器中的所有结果收集到一个 Vec 中
    let mut history = Vec::new();
    for item in clipboard_iter {
        history.push(item?);
    }

    // 返回填充的 Vec
    Ok(history)
}
