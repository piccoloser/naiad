#![allow(unused)]

use super::AuthError;
use crate::auth::JwtHandler;
use crate::entities::sessions::{
    ActiveModel as Session, Column as SessionColumn, Entity as SessionEntity,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct SessionManager {
    db: Arc<DatabaseConnection>,
}

impl SessionManager {
    pub fn new(db: &Arc<DatabaseConnection>) -> Self {
        Self { db: db.clone() }
    }

    pub async fn add(&self, token: &str) -> Result<(), AuthError> {
        let session = Session {
            token: ActiveValue::Set(token.to_string()),
            ..Default::default()
        };

        session.insert(&*self.db).await.map(|_| ()).map_err(|e| {
            tracing::error!("Error adding session: {:?}", e);
            AuthError::SessionStoreError(e.to_string())
        })
    }

    pub async fn clear(&self) -> Result<(), AuthError> {
        SessionEntity::delete_many()
            .exec(&*self.db)
            .await
            .map(|_| ())
            .map_err(|e| {
                tracing::error!("Error clearing sessions: {e:?}");
                AuthError::SessionStoreError(e.to_string())
            })
    }

    pub async fn invalidate(&self, token: &str) -> Result<(), AuthError> {
        SessionEntity::delete_many()
            .filter(SessionColumn::Token.eq(token))
            .exec(&*self.db)
            .await
            .map(|_| ())
            .map_err(|e| {
                tracing::error!("Error invalidating session: {e:?}");
                AuthError::SessionStoreError(e.to_string())
            })
    }

    pub async fn validate(&self, jwt_handler: &JwtHandler, token: &str) -> Result<bool, AuthError> {
        match SessionEntity::find()
            .filter(SessionColumn::Token.eq(token))
            .one(&*self.db)
            .await
        {
            Ok(Some(_)) => {
                if jwt_handler.check(token).await {
                    Ok(true)
                } else {
                    self.invalidate(token).await?;
                    Ok(false)
                }
            }
            Ok(None) => Ok(false),
            Err(e) => Err(AuthError::SessionStoreError(e.to_string())),
        }
    }
}
