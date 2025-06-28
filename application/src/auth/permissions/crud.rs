// TODO: Optimize the role and permission checks with dedicated database queries

use crate::error::ApplicationError;
use database::{
    records::{
        role::{RoleRecord, RoleRecordMutation},
        permission::{PermissionRecord, PermissionRecordMutation},
        user_role::{UserRoleRecord, UserRoleRecordMutation},
        role_permission::{RolePermissionRecord, RolePermissionRecordMutation},
    },
    repositories::{
        role::RoleRepository,
        permission::PermissionRepository,
        user_role::UserRoleRepository,
        role_permission::RolePermissionRepository,
        Repository,
    },
};

pub async fn create_role(name: String, description: Option<String>) -> Result<RoleRecord, ApplicationError> {
    let role_repo = RoleRepository::get().await;
    
    if let Some(_) = role_repo.find_by_name(&name).await? {
        return Err(ApplicationError::InvalidInput(format!("Role with name '{}' already exists", name)));
    }
    
    let role_mutation = RoleRecordMutation {
        name,
        description,
    };
    
    let role = role_repo.create(role_mutation).await?;
    Ok(role)
}

pub async fn create_permission(name: String, resource: Option<String>, action: String) -> Result<PermissionRecord, ApplicationError> {
    let permission_repo = PermissionRepository::get().await;
    
    if let Some(_) = permission_repo.find_by_name(&name).await? {
        return Err(ApplicationError::InvalidInput(format!("Permission with name '{}' already exists", name)));
    }
    
    if let Some(ref resource_name) = resource {
        if let Some(_) = permission_repo.find_by_resource_and_action(resource_name, &action).await? {
            return Err(ApplicationError::InvalidInput(
                format!("Permission with resource '{}' and action '{}' already exists", resource_name, action)
            ));
        }
    }
    
    let permission_mutation = PermissionRecordMutation {
        name,
        resource,
        action,
    };
    
    let permission = permission_repo.create(permission_mutation).await?;
    Ok(permission)
}

pub async fn associate_permissions_to_role(role_id: i32, permission_ids: Vec<i32>) -> Result<Vec<RolePermissionRecord>, ApplicationError> {
    let role_repo = RoleRepository::get().await;
    let role_permission_repo = RolePermissionRepository::get().await;
    
    let role = role_repo.find_by_id(role_id).await?;
    if role.is_none() {
        return Err(ApplicationError::InvalidInput(format!("Role with id {} does not exist", role_id)));
    }
    
    let mut created_associations = Vec::new();
    
    for permission_id in permission_ids {
        if let Some(_) = role_permission_repo.find_by_role_and_permission(role_id, permission_id).await? {
            continue;
        }
        
        let association_mutation = RolePermissionRecordMutation {
            role_id,
            permission_id,
        };
        
        let association = role_permission_repo.create(association_mutation).await?;
        created_associations.push(association);
    }
    
    Ok(created_associations)
}

pub async fn remove_permissions_from_role(role_id: i32, permission_ids: Vec<i32>) -> Result<(), ApplicationError> {
    let role_repo = RoleRepository::get().await;
    let role_permission_repo = RolePermissionRepository::get().await;
    
    let role = role_repo.find_by_id(role_id).await?;
    if role.is_none() {
        return Err(ApplicationError::InvalidInput(format!("Role with id {} does not exist", role_id)));
    }
    
    for permission_id in permission_ids {
        role_permission_repo.remove_permission_from_role(role_id, permission_id).await?;
    }
    
    Ok(())
}

pub async fn associate_roles_to_user(user_id: i32, role_ids: Vec<i32>) -> Result<Vec<UserRoleRecord>, ApplicationError> {
    let user_role_repo = UserRoleRepository::get().await;
    
    let mut created_associations = Vec::new();
    
    for role_id in role_ids {
        if let Some(_) = user_role_repo.find_by_user_and_role(user_id, role_id).await? {
            continue;
        }
        
        let association_mutation = UserRoleRecordMutation {
            user_id,
            role_id,
        };
        
        let association = user_role_repo.create(association_mutation).await?;
        created_associations.push(association);
    }
    
    Ok(created_associations)
}

pub async fn remove_roles_from_user(user_id: i32, role_ids: Vec<i32>) -> Result<(), ApplicationError> {
    let user_role_repo = UserRoleRepository::get().await;
    
    for role_id in role_ids {
        user_role_repo.remove_role_from_user(user_id, role_id).await?;
    }
    
    Ok(())
}

pub async fn get_user_roles(user_id: i32) -> Result<Vec<RoleRecord>, ApplicationError> {
    let user_role_repo = UserRoleRepository::get().await;
    let roles = user_role_repo.find_roles_by_user_id(user_id).await?;
    Ok(roles)
}

pub async fn get_user_permissions(user_id: i32) -> Result<Vec<PermissionRecord>, ApplicationError> {
    let role_permission_repo = RolePermissionRepository::get().await;
    let permissions = role_permission_repo.find_user_permissions(user_id).await?;
    Ok(permissions)
}

pub async fn get_role_permissions(role_id: i32) -> Result<Vec<PermissionRecord>, ApplicationError> {
    let role_permission_repo = RolePermissionRepository::get().await;
    let permissions = role_permission_repo.find_permissions_by_role_id(role_id).await?;
    Ok(permissions)
}

pub async fn user_has_permission(user_id: i32, permission_name: &str) -> Result<bool, ApplicationError> {
    let permissions = get_user_permissions(user_id).await?;
    let has_permission = permissions.iter().any(|p| p.name == permission_name);
    Ok(has_permission)
}

pub async fn user_has_resource_permission(user_id: i32, resource: &str, action: &str) -> Result<bool, ApplicationError> {
    let permissions = get_user_permissions(user_id).await?;
    let has_permission = permissions.iter().any(|p| {
        p.resource.as_deref() == Some(resource) && p.action == action
    });
    Ok(has_permission)
}
