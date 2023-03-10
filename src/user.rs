use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct User {
    pub(crate) username: String,

    // todo: MAJOR FUCKING SECURITY ISSUE
    pub(crate) password: String,
    pub(crate) permissions: Vec<Permission>,
}

impl User {
    pub(crate) fn new(username: String, password: String, permissions: Vec<Permission>) -> Self {
        Self {
            username,
            password,
            permissions,
        }
    }
}

impl Default for User {
    fn default() -> Self {
        Self {
            // todo: REMOVE THIS
            username: "admin".to_string(),
            password: "password".to_string(),
            permissions: vec![
                Permission::User(UserPermission::Query(UserPermissionTarget::Own)),
                Permission::User(UserPermission::Update(UserPermissionTarget::Own)),
            ],
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum Permission {
    All,
    Head(HeadPermission),
    Plugin(PluginPermission),
    User(UserPermission),
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum HeadPermission {
    Query,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum PluginPermission {
    Query,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum UserPermission {
    Create,

    Delete(UserPermissionTarget),
    Update(UserPermissionTarget),
    Query(UserPermissionTarget),
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum UserPermissionTarget {
    Own,
    Other,
}
