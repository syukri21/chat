use std::sync::Arc;

use credentials::{credential::Credential, credential_services::CredentialServiceInterface};
use fake::{
    faker::{
        internet::en::{SafeEmail, Username},
        name::raw::{FirstName, LastName},
    },
    locales::EN,
    Fake,
};
use log::info;
use shaku::{Component, Interface};
use tokio::spawn;
use usecases::userdetail_usecase::UserDetailUsecase;
use user_details::entity::UserDetail;
use users::user_services::UserServiceInterface;

#[derive(Component)]
#[shaku(interface = Faker)]
pub struct FakerImpl {
    #[shaku(inject)]
    faker_inner: Arc<dyn FakerInner>,
}

#[async_trait::async_trait]
pub trait Faker: Interface {
    async fn generate_random_users(&self, numbers: i32) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
impl Faker for FakerImpl {
    async fn generate_random_users(&self, numbers: i32) -> anyhow::Result<()> {
        for _ in 0..numbers {
            let fi = __self.faker_inner.clone();
            spawn::<_>(async move {
                fi.generate_random_user().await.map_err(|e| {
                    info!("Error generating random user: {}", e);
                    e
                })
            });
        }
        Ok(())
    }
}

#[derive(Component)]
#[shaku(interface = FakerInner)]
pub struct FakerInnerImpl {
    #[shaku(inject)]
    user_detail_service: Arc<dyn UserDetailUsecase>,
    #[shaku(inject)]
    user_service: Arc<dyn UserServiceInterface>,
    #[shaku(inject)]
    credential_services: Arc<dyn CredentialServiceInterface>,
}

#[async_trait::async_trait]
pub trait FakerInner: Interface {
    async fn generate_random_user(&self) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
impl FakerInner for FakerInnerImpl {
    async fn generate_random_user(&self) -> anyhow::Result<()> {
        let user_id = uuid::Uuid::new_v4();
        let user_name = Username().fake();
        let email = SafeEmail().fake::<String>();
        let password = "password1234".to_string();

        let user = users::user::User {
            id: user_id,
            username: user_name,
            email,
            password,
            is_active: true,
            created_at: Some(chrono::Local::now().naive_local()),
            updated_at: Some(chrono::Local::now().naive_local()),
            deleted_at: None,
        };
        let _ = self
            .user_service
            .create_user(&user)
            .await
            .expect("Failed to create dummy user");

        let credential = Credential::new(user.id, "private_key", "public_key");
        let _ = self
            .credential_services
            .create_credential(&credential)
            .await
            .expect("Failed to create dummy credential");

        let mut user_detail = UserDetail::new(user.id);

        user_detail.first_name = FirstName(EN).fake::<String>();
        user_detail.last_name = LastName(EN).fake();
        let _ = self.user_detail_service.update_profile(&user_detail).await;

        info!("Created {} users", user.username);
        Ok(())
    }
}
