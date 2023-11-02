mod authorization;
mod create;
// mod login;

use std::sync::Arc;

use self::authorization::IdentityAuthorizationUsecase;
use self::authorization::IdentityAuthorizationUsecaseImpl;
use self::create::IdentityCreateUsecase;
use self::create::IdentityCreateUsecaseImpl;
use super::repository::IdentityRepository;
use super::service::IdentityService;
use super::service::IdentityServiceImpl;
use super::Result;

#[derive(Clone)]
pub struct IdentitySecret {
  pub password_key: String,
  pub jwt_key: String,
  pub jwt_expiration_sec: i64,
}

pub struct IdentityUsecase {
  pub create: Box<dyn IdentityCreateUsecase>,
  pub authorization: Box<dyn IdentityAuthorizationUsecase>,
  // pub login: Box<dyn IdentityLoginUsecase>,
}

impl IdentityUsecase {
  pub fn create(
    identity_secret: IdentitySecret,
    identity_repository: Arc<dyn IdentityRepository>,
  ) -> Self {
    let identity_service: Arc<dyn IdentityService> = Arc::new(IdentityServiceImpl {
      identity_repository: Arc::clone(&identity_repository),
      identity_secret: identity_secret.clone(),
    });

    let identity_create_usecase: Box<dyn IdentityCreateUsecase> =
      Box::new(IdentityCreateUsecaseImpl {
        identity_repository: Arc::clone(&identity_repository),
        identity_service: Arc::clone(&identity_service),
        identity_secret: identity_secret.clone(),
      });

    let identity_authorization_usecase: Box<dyn IdentityAuthorizationUsecase> =
      Box::new(IdentityAuthorizationUsecaseImpl {
        identity_secret: identity_secret.clone(),
      });

    // let identity_login_usecase: Box<dyn IdentityLoginUsecase> =
    //   Box::new(IdentityLoginUsecaseImpl {
    //     identity_repository: Arc::clone(&identity_repository),
    //     identity_service: Arc::clone(&identity_service),
    //     identity_secret: identity_secret.clone(),
    //   });

    Self {
      create: identity_create_usecase,
      authorization: identity_authorization_usecase,
      // login: identity_login_usecase,
    }
  }
}
