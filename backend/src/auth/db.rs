use crate::core::{forms::RegisterForm, roles::UserRole};
use crate::entities::roles::{Column as RolesColumn, Entity as Role};
use crate::entities::user_roles::{
    ActiveModel as URActiveModel, Column as URColumn, Entity as UserRoles,
};
use crate::entities::users::{
    ActiveModel as UserActiveModel, Column as UserColumn, Entity as User, Model as UserModel,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
    TransactionTrait,
};
use std::{str::FromStr, sync::Arc};

use super::AuthError;

#[derive(Clone, Debug)]
pub struct DbProvider {
    db: Arc<DatabaseConnection>,
}

impl DbProvider {
    pub fn new(db: &Arc<DatabaseConnection>) -> Self {
        Self { db: db.clone() }
    }
}

impl super::Authenticator for DbProvider {
    async fn fetch_roles(&self, user_model: &UserModel) -> Result<Vec<UserRole>, AuthError> {
        let roles = UserRoles::find()
            .filter(URColumn::UserId.eq(user_model.id))
            .find_also_related(Role)
            .all(self.db.as_ref())
            .await
            .map_err(|e| AuthError::DbQueryError(e.to_string()))?;

        let role_titles = roles
            .into_iter()
            .filter_map(|(_, role)| role.and_then(|r| UserRole::from_str(&r.title).ok()))
            .collect();

        Ok(role_titles)
    }

    async fn get_user_by_identifier(&self, identifier: &str) -> Result<UserModel, AuthError> {
        User::find()
            .filter(UserColumn::Email.eq(identifier))
            .one(self.db.as_ref())
            .await
            .map_err(|e| AuthError::DbQueryError(e.to_string()))?
            .ok_or(AuthError::InvalidCredentials)
    }

    async fn read_user(&self, email: &str, password: &str) -> Result<UserModel, AuthError> {
        let user = User::find()
            .filter(UserColumn::Email.eq(email))
            .one(self.db.as_ref())
            .await
            .map_err(|e| AuthError::DbQueryError(e.to_string()))?;

        match user {
            Some(user) => match bcrypt::verify(password, &user.pw_hash) {
                Ok(true) => Ok(user),
                _ => Err(AuthError::InvalidCredentials),
            },
            None => Err(AuthError::InvalidCredentials),
        }
    }
}

impl super::UserCreator for DbProvider {
    async fn assign_role(&self, user: UserModel, role: UserRole) -> Result<(), AuthError> {
        let txn = self
            .db
            .begin()
            .await
            .map_err(|e| AuthError::DbError(e.to_string()))?;

        let role = Role::find()
            .filter(RolesColumn::Title.eq(role.to_string()))
            .one(&txn)
            .await
            .map_err(|e| AuthError::DbQueryError(e.to_string()))?
            .ok_or(AuthError::ResourceNotFound)?;

        let user_role = URActiveModel {
            user_id: Set(user.id),
            role_id: Set(role.id),
            ..Default::default()
        };

        user_role
            .insert(&txn)
            .await
            .map_err(|e| AuthError::DbError(e.to_string()))?;

        txn.commit()
            .await
            .map_err(|e| AuthError::DbError(e.to_string()))
    }

    async fn create_user(&self, form: RegisterForm) -> Result<UserModel, AuthError> {
        let txn = self
            .db
            .begin()
            .await
            .map_err(|e| AuthError::DbError(e.to_string()))?;

        let exists = User::find()
            .filter(UserColumn::Email.eq(&form.email))
            .one(&txn)
            .await
            .map_err(|e| AuthError::DbError(e.to_string()))?;

        if exists.is_some() {
            return Err(AuthError::AccountAlreadyExists);
        }

        let hash = bcrypt::hash(&form.password, bcrypt::DEFAULT_COST)
            .map_err(|e| AuthError::EncryptionError(e.to_string()))?;

        let user = UserActiveModel {
            email: Set(form.email.clone()),
            username: Set(None),
            pw_hash: Set(hash),
            ..Default::default()
        }
        .insert(&txn)
        .await
        .map_err(|e| AuthError::DbError(e.to_string()))?;

        txn.commit()
            .await
            .map_err(|e| AuthError::DbError(e.to_string()))?;

        Ok(user)
    }
}
