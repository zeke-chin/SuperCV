use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(HostClipboard::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(HostClipboard::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(HostClipboard::Type).integer().not_null())
                    .col(ColumnDef::new(HostClipboard::Path).string().not_null())
                    .col(ColumnDef::new(HostClipboard::Content).text().not_null())
                    .col(ColumnDef::new(HostClipboard::Timestamp).integer().not_null())
                    .col(ColumnDef::new(HostClipboard::Hash).string().not_null())
                    .index(
                        Index::create()
                            .name("idx_host_clipboard_hash_unique")
                            .col(HostClipboard::Hash)
                            .unique()
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(HostClipboard::Table).to_owned())
            .await
    }
}

// 移除了 #[derive(Iden)]
enum HostClipboard {
    Table,
    Id,
    Type,
    Path,
    Content,
    Timestamp,
    Hash,
}

impl Iden for HostClipboard {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Table => "host_clipboard",
                Self::Id => "id",
                Self::Type => "type",
                Self::Path => "path",
                Self::Content => "content",
                Self::Timestamp => "timestamp",
                Self::Hash => "hash",
            }
        )
        .unwrap();
    }
}