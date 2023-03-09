pub(crate) struct User {
    pub(crate) username: String,
    pub(crate) permissions: Vec<Permission>,
}

impl User {
    pub(crate) fn new(username: String, permissions: Vec<Permission>) -> Self {
        Self {
            username,
            permissions,
        }
    }
}

impl Default for User {
    fn default() -> Self {
        Self {
            username: "User".to_string(),
            permissions: vec![
                Permission::User(UserPermission::Query(UserPermissionTarget::This)),
                Permission::User(UserPermission::Update(UserPermissionTarget::This)),
            ],
        }
    }
}

pub(crate) enum Permission {
    All,
    Head(HeadPermission),
    Plugin(PluginPermission),
    User(UserPermission),
}

pub(crate) enum HeadPermission {
    Query,
}

pub(crate) enum PluginPermission {
    Query,
}

pub(crate) enum UserPermission {
    Create,

    Delete(UserPermissionTarget),
    Update(UserPermissionTarget),
    Query(UserPermissionTarget),
}

pub(crate) enum UserPermissionTarget {
    This,
    Other,
}
