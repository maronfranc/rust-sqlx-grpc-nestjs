use sqlx::postgres::PgPool;
use tonic::{Request, Response, Status};

use crate::grpc_protos::user_service_server::{UserService, UserServiceServer};
use crate::grpc_protos::{ByIdRequest, EmptyRequest, FindAllUsersResponse, FindUserResponse};
use crate::model;
mod user_repository;

pub fn new_grpc_service(pool: PgPool) -> UserServiceServer<UserController> {
    let controller = UserController::new_grpc_service(pool);
    UserServiceServer::new(controller)
}
fn into_response(user: &model::User) -> FindUserResponse {
    FindUserResponse {
        id: user.id,
        email: user.email.clone(),
        username: user.username.clone(),
    }
}
pub struct UserController {
    pool: PgPool,
}

impl UserController {
    pub fn new_grpc_service(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[tonic::async_trait]
impl UserService for UserController {
    async fn find_many(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<FindAllUsersResponse>, Status> {
        let users = match user_repository::UserRepository::find_many(&self.pool).await {
            Ok(us) => {
                let users_response = us.iter().map(|t| into_response(t)).collect::<Vec<_>>();
                FindAllUsersResponse {
                    users: users_response,
                }
            }
            Err(_e) => return Err(Status::not_found("Not Found")),
        };

        Ok(Response::new(users))
    }

    async fn find_one(
        &self,
        req: Request<ByIdRequest>,
    ) -> Result<Response<FindUserResponse>, Status> {
        let id_user: i32 = req.into_inner().id;
        let user = match user_repository::UserRepository::find_one(&self.pool, id_user).await {
            Ok(us) => into_response(&us),
            Err(_e) => return Err(Status::not_found("Not Found")),
        };
        Ok(Response::new(user))
    }
}
