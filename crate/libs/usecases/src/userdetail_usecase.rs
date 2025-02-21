use std::{fs, path::Path, sync::Arc};
use uuid::Uuid;

use anyhow::Context;
use async_trait::async_trait;
use commons::generic_errors::GenericError;
use infer::Infer;
use shaku::{Component, Interface};
use user_details::{entity::UserDetail, user_detail_service::UserDetailService};
use users::{user::User, user_services::UserServiceInterface};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserInfo {
    pub username: String,
    pub email: String,
    pub user_details: Option<UserDetail>,
}

impl UserInfo {
    pub fn new(username: String, email: String, user_details: Option<UserDetail>) -> Self {
        Self {
            username,
            email,
            user_details,
        }
    }

    pub fn get_profile_picture(&self) -> String {
        self.user_details
            .as_ref()
            .and_then(|details| details.profile_picture.clone())
            .unwrap_or_else(|| self.get_default_profile_picture())
    }

    fn get_default_profile_picture(&self) -> String {
        format!(
            "https://ui-avatars.com/api/?name={}&background=random&rounded=true",
            self.get_full_name()
        )
    }

    pub fn get_full_name(&self) -> String {
        self.user_details
            .as_ref()
            .and_then(|details| {
                return Some(format!("{} {}", details.first_name, details.last_name));
            })
            .unwrap_or_else(|| self.username.clone())
    }
}

#[derive(Component)]
#[shaku(interface = UserDetailUsecase)]
pub struct UserDetailUsecaseImpl {
    #[shaku(inject)]
    user_detail_service: Arc<dyn UserDetailService>,
    #[shaku(inject)]
    user_service: Arc<dyn UserServiceInterface>,
}

#[async_trait]
pub trait UserDetailUsecase: Interface {
    async fn update_profile(&self, user_detail: &UserDetail) -> anyhow::Result<()>;
    async fn get_user_info(&self, user_id: &str) -> anyhow::Result<UserInfo>;
    async fn upload_profile_picture(&self, user_id: &str, image: &[u8]) -> anyhow::Result<String>;
}

#[async_trait]
impl UserDetailUsecase for UserDetailUsecaseImpl {
    async fn update_profile(&self, user_detail: &UserDetail) -> anyhow::Result<()> {
        self.user_detail_service
            .upsert_user_detail(user_detail)
            .await
            .map_err(GenericError::unknown)
    }

    async fn get_user_info(&self, user_id: &str) -> anyhow::Result<UserInfo> {
        let user: User = self
            .user_service
            .get_user_by_uuid(user_id.parse()?)
            .await
            .map_err(GenericError::unknown)?;

        let user_detail = self
            .user_detail_service
            .get_user_detail_by_user_id(user_id)
            .await
            .ok();

        Ok(UserInfo {
            username: user.username,
            email: user.email,
            user_details: user_detail,
        })
    }

    async fn upload_profile_picture(&self, user_id: &str, image: &[u8]) -> anyhow::Result<String> {
        // Validate the file type
        let infer = Infer::new();
        if !infer.is_image(image) {
            return Err(anyhow::anyhow!("Uploaded file is not a valid image"));
        }

        // Create the uploads directory if it doesn't exist
        let upload_dir = Path::new("crate/application/web/assets/uploads");
        if !upload_dir.exists() && fs::create_dir_all(upload_dir).is_err() {
            return Err(anyhow::anyhow!("Failed to create uploads directory"));
        }

        // Generate a unique filename using UUID and the user ID
        let file_name = format!("{}_{}.png", user_id, Uuid::new_v4());
        let file_path = upload_dir.join(file_name.clone());

        // Write the image bytes to the file
        fs::write(&file_path, image).context("Failed to write image to file")?;

        let file_name = format!("/assets/uploads/{}", file_name);
        self.user_detail_service
            .update_profile_picture(user_id, file_name.as_str())
            .await
            .map_err(GenericError::unknown)?;

        // Return the file path or URL
        Ok(file_name)
    }
}
