use sea_orm::*;
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::{Expr, OnConflict};

use crate::core::pasteboard::PasteboardContent;
use crate::db::entities::host_clipboard::{self, Entity as ClipboardEntries};
use crate::db::entities::prelude::HostClipboard;
use crate::utils::config::CONFIG;

pub async fn add_clipboard_entry(
    db: &DatabaseConnection,
    item: PasteboardContent,
) -> Result<host_clipboard::Model, DbErr> {
    let timestamp = item.date_time.timestamp();

    // 使用 Sea-ORM 的查询构建器进行插入或更新操作
    let _ = ClipboardEntries::insert(host_clipboard::ActiveModel {
        r#type: Set(item.r#type.to_i32()),
        path: Set(item.path),
        content: Set(item.text_content),
        timestamp: Set(timestamp),
        hash: Set(item.hash.clone()),
        ..Default::default()
    })
    .on_conflict(
        OnConflict::column(host_clipboard::Column::Hash)
            .update_column(host_clipboard::Column::Timestamp)
            .to_owned(),
    )
    .exec(db)
    .await?;

    // 查询刚插入或更新的记录
    ClipboardEntries::find()
        .filter(host_clipboard::Column::Hash.eq(item.hash))
        .one(db)
        .await?
        .ok_or(DbErr::Custom(
            "Failed to retrieve inserted or updated entry".to_string(),
        ))
}

pub async fn get_clipboards_by_type_list(
    db: &DatabaseConnection,
    text: Option<&str>,
    num: Option<u64>,
    type_list: Option<Vec<i32>>,
) -> Result<Vec<host_clipboard::Model>, DbErr> {
    let (text_ts, img_ts, file_ts) = {
        let config = CONFIG.read().unwrap(); // 获取读锁
        let (text_ts, img_ts, file_ts) = config.get_expired_ts();
        (text_ts, img_ts, file_ts) // 将值返回给外部变量
    };

    let mut query = HostClipboard::find();

    // 根据不同的类型指定不同的时间戳
    query = query.filter(
        Condition::any()
            .add(
                Expr::col(host_clipboard::Column::Type)
                    .eq(0)
                    .and(host_clipboard::Column::Timestamp.gt(text_ts)),
            )
            .add(
                Expr::col(host_clipboard::Column::Type)
                    .eq(1)
                    .and(host_clipboard::Column::Timestamp.gt(img_ts)),
            )
            .add(
                Expr::col(host_clipboard::Column::Type)
                    .eq(2)
                    .and(host_clipboard::Column::Timestamp.gt(file_ts)),
            ),
    );

    if let Some(text) = text {
        query = query.filter(
            Expr::cust("LOWER(content)").like(format!("%{}%", text)), // 直接使用原始文本进行模糊匹配
        );
    }

    if let Some(num) = num {
        query = query.limit(num);
    }

    // 如果提供了type_list，则添加类型过滤
    if let Some(type_list) = type_list {
        query = query.filter(host_clipboard::Column::Type.is_in(type_list));
    }

    // 按时间戳降序排序并限制结果数量
    query = query.order_by_desc(host_clipboard::Column::Timestamp);

    query.all(db).await
}

pub async fn _get_clipboard_entries_by_gt_timestamp(
    db: &DatabaseConnection,
    timestamp: i64,
) -> Result<Vec<host_clipboard::Model>, DbErr> {
    let query = HostClipboard::find()
        .filter(host_clipboard::Column::Timestamp.gt(timestamp))
        .order_by_desc(host_clipboard::Column::Timestamp);

    query.all(db).await
}

pub async fn _get_clipboard_entries_by_id_list(
    db: &DatabaseConnection,
    id_list: Option<Vec<i32>>,
) -> Result<Vec<host_clipboard::Model>, DbErr> {
    match id_list {
        Some(ids) if !ids.is_empty() => {
            HostClipboard::find()
                .filter(host_clipboard::Column::Id.is_in(ids))
                .order_by_desc(host_clipboard::Column::Timestamp)
                .all(db)
                .await
        }
        _ => Ok(vec![]),
    }
}

// pub async fn update_clipboard_entry(
//     db: &DatabaseConnection,
//     id: i32,
//     content: String,
// ) -> Result<host_clipboard::Model, DbErr> {
//     let entry = ClipboardEntries::find_by_id(id).one(db).await?;
//     if let Some(entry) = entry {
//         let mut entry: clipboard::ActiveModel = entry.into();
//         entry.content = Set(content);
//         entry.update(db).await
//     } else {
//         Err(DbErr::Custom("Entry not found".to_string()))
//     }
// }

pub async fn _delete_clipboard_entry(
    db: &DatabaseConnection,
    id: i32,
) -> Result<DeleteResult, DbErr> {
    ClipboardEntries::delete_by_id(id).exec(db).await
}
