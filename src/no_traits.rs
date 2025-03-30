use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dog {
    pub id: String,
    pub name: String,
    pub age: u32,
}

#[derive(Debug, Clone)]
pub struct DogRepository {
    pub dogs: Vec<Dog>,
}

impl DogRepository {
    pub fn new() -> Self {
        Self { dogs: vec![] }
    }

    pub async fn add_dog(&mut self, dog: Dog) {
        self.dogs.push(dog);
    }

    pub async fn get_dogs(&self) -> Vec<Dog> {
        self.dogs.clone()
    }
}

#[derive(Debug)]
pub struct DogService {
    pub dog_repository: Arc<RwLock<DogRepository>>,
}

impl DogService {
    pub fn new(dog_repository: Arc<RwLock<DogRepository>>) -> Self {
        Self { dog_repository }
    }

    pub async fn add_dog(&self, dog: Dog) {
        self.dog_repository.write().await.add_dog(dog).await;
    }

    pub async fn get_dogs(&self) -> Vec<Dog> {
        self.dog_repository.read().await.get_dogs().await
    }
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub dog_service: Arc<DogService>,
}

pub async fn add_dog(State(state): State<AppState>, Json(dog): Json<Dog>) -> impl IntoResponse {
    state.dog_service.add_dog(dog).await;
    (StatusCode::CREATED, "Dog created")
}

pub async fn get_dogs(State(state): State<AppState>) -> Json<Vec<Dog>> {
    let dogs = state.dog_service.get_dogs().await;
    Json(dogs)
}

pub async fn router() -> Router {
    let dog_repository = Arc::new(RwLock::new(DogRepository::new()));
    dog_repository
        .write()
        .await
        .add_dog(Dog {
            id: "1".to_string(),
            name: "Max".to_string(),
            age: 5,
        })
        .await;
    dog_repository
        .write()
        .await
        .add_dog(Dog {
            id: "2".to_string(),
            name: "Luna".to_string(),
            age: 3,
        })
        .await;
    dog_repository
        .write()
        .await
        .add_dog(Dog {
            id: "3".to_string(),
            name: "Charlie".to_string(),
            age: 2,
        })
        .await;
    let dog_service = Arc::new(DogService::new(dog_repository));
    let app_state = AppState { dog_service };

    Router::new()
        .route("/dogs", get(get_dogs))
        .route("/dogs", post(add_dog))
        .with_state(app_state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum_test::TestServer;

    #[tokio::test]
    async fn test_get_dogs() {
        let app = router().await;
        let server = TestServer::new(app).unwrap();

        for _ in 0..1000 {
            let response = server.get("/dogs").await;
            assert_eq!(response.status_code(), StatusCode::OK);
        }
    }
}
