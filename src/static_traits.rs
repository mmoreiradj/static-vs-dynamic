use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::get};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dog {
    pub id: String,
    pub name: String,
    pub age: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroomingRecord {
    pub dog_id: String,
    pub date: String,
    pub service_type: String,
    pub price: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingRecord {
    pub dog_id: String,
    pub skill: String,
    pub proficiency_level: u8,
    pub last_trained: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthRecord {
    pub dog_id: String,
    pub weight: f64,
    pub vaccinations: Vec<String>,
    pub last_checkup: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DogHouse {
    pub id: String,
    pub size: String,
    pub material: String,
    pub assigned_dog_id: Option<String>,
}

pub trait DogRepositoryTrait: Send + Sync + Clone + 'static {
    fn add_dog(&mut self, dog: Dog) -> impl std::future::Future<Output = ()> + Send;
    fn get_dogs(&self) -> impl std::future::Future<Output = Vec<Dog>> + Send;
}

pub trait GroomingServiceTrait: Send + Sync + Clone + 'static {
    fn add_grooming_record(&self, record: GroomingRecord) -> impl std::future::Future<Output = ()> + Send;
    fn get_grooming_history(&self, dog_id: &str) -> impl std::future::Future<Output = Vec<GroomingRecord>> + Send;
    fn calculate_total_grooming_cost(&self, dog_id: &str) -> impl std::future::Future<Output = f64> + Send;
}

pub trait TrainingServiceTrait: Send + Sync + Clone + 'static {
    fn add_training_record(&self, record: TrainingRecord) -> impl std::future::Future<Output = ()> + Send;
    fn get_training_history(&self, dog_id: &str) -> impl std::future::Future<Output = Vec<TrainingRecord>> + Send;
    fn get_dog_skills(&self, dog_id: &str) -> impl std::future::Future<Output = Vec<String>> + Send;
}

pub trait HealthServiceTrait: Send + Sync + Clone + 'static {
    fn add_health_record(&self, record: HealthRecord) -> impl std::future::Future<Output = ()> + Send;
    fn get_health_history(&self, dog_id: &str) -> impl std::future::Future<Output = Vec<HealthRecord>> + Send;
    fn get_dog_weight_history(&self, dog_id: &str) -> impl std::future::Future<Output = Vec<(String, f64)>> + Send;
}

pub trait DogHouseServiceTrait: Send + Sync + Clone + 'static {
    fn add_dog_house(&self, house: DogHouse) -> impl std::future::Future<Output = ()> + Send;
    fn assign_dog_to_house(&self, dog_id: &str, house_id: &str) -> impl std::future::Future<Output = ()> + Send;
    fn get_dog_house(&self, dog_id: &str) -> impl std::future::Future<Output = Option<DogHouse>> + Send;
    fn get_available_houses(&self) -> impl std::future::Future<Output = Vec<DogHouse>> + Send;
}

pub trait DogServiceTrait: Send + Sync + Clone + 'static {
    fn add_dog(&self, dog: Dog) -> impl std::future::Future<Output = ()> + Send;
    fn get_dogs(&self) -> impl std::future::Future<Output = Vec<Dog>> + Send;
}

#[derive(Debug, Clone)]
pub struct DogRepository {
    pub dogs: Vec<Dog>,
}

#[derive(Debug, Clone)]
pub struct GroomingService {
    pub records: Vec<GroomingRecord>,
}

#[derive(Debug, Clone)]
pub struct TrainingService {
    pub records: Vec<TrainingRecord>,
}

#[derive(Debug, Clone)]
pub struct HealthService {
    pub records: Vec<HealthRecord>,
}

#[derive(Debug, Clone)]
pub struct DogHouseService {
    pub houses: Vec<DogHouse>,
}

#[derive(Debug, Clone)]
pub struct DogService<R: DogRepositoryTrait> {
    pub dog_repository: Arc<RwLock<R>>,
}

impl<R: DogRepositoryTrait> DogService<R> {
    pub fn new(dog_repository: Arc<RwLock<R>>) -> Self {
        Self { dog_repository }
    }
}

impl DogRepository {
    pub fn new() -> Self {
        Self { dogs: vec![] }
    }
}

impl GroomingService {
    pub fn new() -> Self {
        Self { records: vec![] }
    }
}

impl TrainingService {
    pub fn new() -> Self {
        Self { records: vec![] }
    }
}

impl HealthService {
    pub fn new() -> Self {
        Self { records: vec![] }
    }
}

impl DogHouseService {
    pub fn new() -> Self {
        Self { houses: vec![] }
    }
}


impl DogRepositoryTrait for DogRepository {
    fn add_dog(&mut self, dog: Dog) -> impl std::future::Future<Output = ()> + Send {
        async move {
            self.dogs.push(dog);
        }
    }

    fn get_dogs(&self) -> impl std::future::Future<Output = Vec<Dog>> + Send {
        async move {
            let mut dogs = self.dogs.clone();

            for _ in 0..1000 {
                dogs.sort_by(|a, b| a.name.cmp(&b.name));
                dogs.sort_by(|a, b| a.age.cmp(&b.age));
                dogs.sort_by(|a, b| a.id.cmp(&b.id));
            }

            dogs
        }
    }
}

impl GroomingServiceTrait for GroomingService {
    fn add_grooming_record(&self, record: GroomingRecord) -> impl std::future::Future<Output = ()> + Send {
        async move {
            let mut records = self.records.clone();
            records.push(record);

            for _ in 0..500 {
                records.sort_by(|a, b| a.date.cmp(&b.date));
                records.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
            }
        }
    }

    fn get_grooming_history(&self, dog_id: &str) -> impl std::future::Future<Output = Vec<GroomingRecord>> + Send {
        async move {
            let mut records = self.records.clone();

            for _ in 0..300 {
                records = records
                    .into_iter()
                    .filter(|r| r.dog_id == dog_id)
                    .map(|r| GroomingRecord {
                        dog_id: r.dog_id.clone(),
                        date: r.date.clone(),
                        service_type: r.service_type.to_uppercase(),
                        price: r.price * 1.1,
                    })
                    .collect();
            }

            records
        }
    }

    fn calculate_total_grooming_cost(&self, dog_id: &str) -> impl std::future::Future<Output = f64> + Send {
        async move {
            let mut total = 0.0;
            let records = self.get_grooming_history(dog_id).await;

            for _ in 0..200 {
                total = records.iter().map(|r| r.price).sum();
                total *= 1.1;
                total /= 1.1;
            }

            total
        }
    }
}

impl TrainingServiceTrait for TrainingService {
    fn add_training_record(&self, record: TrainingRecord) -> impl std::future::Future<Output = ()> + Send {
        async move {
            let mut records = self.records.clone();
            records.push(record);

            for _ in 0..400 {
                records.sort_by(|a, b| a.last_trained.cmp(&b.last_trained));
                records.sort_by(|a, b| a.proficiency_level.cmp(&b.proficiency_level));
            }
        }
    }

    fn get_training_history(&self, dog_id: &str) -> impl std::future::Future<Output = Vec<TrainingRecord>> + Send {
        async move {
            let mut records = self.records.clone();

            for _ in 0..300 {
                records = records
                    .into_iter()
                    .filter(|r| r.dog_id == dog_id)
                    .map(|r| TrainingRecord {
                        dog_id: r.dog_id.clone(),
                        skill: r.skill.to_uppercase(),
                        proficiency_level: r.proficiency_level,
                        last_trained: r.last_trained.clone(),
                    })
                    .collect();
            }

            records
        }
    }

    fn get_dog_skills(&self, dog_id: &str) -> impl std::future::Future<Output = Vec<String>> + Send {
        async move {
            let mut skills = Vec::new();
            let records = self.get_training_history(dog_id).await;

            for _ in 0..200 {
                skills = records.iter().map(|r| r.skill.clone()).collect();
                skills.sort();
                skills.dedup();
            }

            skills
        }
    }
}


impl HealthServiceTrait for HealthService {
    fn add_health_record(&self, record: HealthRecord) -> impl std::future::Future<Output = ()> + Send {
        async move {
            let mut records = self.records.clone();
            records.push(record);

            for _ in 0..400 {
                records.sort_by(|a, b| a.last_checkup.cmp(&b.last_checkup));
                records.sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap());
            }
        }
    }

    fn get_health_history(&self, dog_id: &str) -> impl std::future::Future<Output = Vec<HealthRecord>> + Send {
        async move {
            let mut records = self.records.clone();

            for _ in 0..300 {
                records = records
                    .into_iter()
                    .filter(|r| r.dog_id == dog_id)
                    .map(|r| HealthRecord {
                        dog_id: r.dog_id.clone(),
                        weight: r.weight * 1.1,
                        vaccinations: r.vaccinations.iter().map(|v| v.to_uppercase()).collect(),
                        last_checkup: r.last_checkup.clone(),
                    })
                    .collect();
            }

            records
        }
    }

    fn get_dog_weight_history(&self, dog_id: &str) -> impl std::future::Future<Output = Vec<(String, f64)>> + Send {
        async move {
            let mut history = Vec::new();
            let records = self.get_health_history(dog_id).await;

            for _ in 0..200 {
                history = records
                    .iter()
                    .map(|r| (r.last_checkup.clone(), r.weight))
                    .collect();
                history.sort_by(|a, b| a.0.cmp(&b.0));
            }

            history
        }
    }
}


impl DogHouseServiceTrait for DogHouseService {
    fn add_dog_house(&self, house: DogHouse) -> impl std::future::Future<Output = ()> + Send {
        async move {
            let mut houses = self.houses.clone();
            houses.push(house);

            for _ in 0..400 {
                houses.sort_by(|a, b| a.id.cmp(&b.id));
                houses.sort_by(|a, b| a.size.cmp(&b.size));
            }
        }
    }

    fn assign_dog_to_house(&self, dog_id: &str, house_id: &str) -> impl std::future::Future<Output = ()> + Send {
        async move {
            let mut houses = self.houses.clone();

            for _ in 0..300 {
                houses = houses
                    .into_iter()
                    .map(|h| {
                        if h.id == house_id {
                            DogHouse {
                                id: h.id,
                                size: h.size,
                                material: h.material,
                                assigned_dog_id: Some(dog_id.to_string()),
                            }
                        } else {
                            h
                        }
                    })
                    .collect();
            }
        }
    }

    fn get_dog_house(&self, dog_id: &str) -> impl std::future::Future<Output = Option<DogHouse>> + Send {
        async move {
            let mut houses = self.houses.clone();

            for _ in 0..200 {
                houses = houses
                    .into_iter()
                    .filter(|h| h.assigned_dog_id.as_deref() == Some(dog_id))
                    .collect();
            }

            houses.first().cloned()
        }
    }

    fn get_available_houses(&self) -> impl std::future::Future<Output = Vec<DogHouse>> + Send {
        async move {
            let mut houses = self.houses.clone();

            for _ in 0..300 {
                houses = houses
                    .into_iter()
                    .filter(|h| h.assigned_dog_id.is_none())
                    .map(|h| DogHouse {
                        id: h.id.clone(),
                        size: h.size.to_uppercase(),
                        material: h.material.clone(),
                        assigned_dog_id: None,
                    })
                    .collect();
            }

            houses
        }
    }
}


impl<R: DogRepositoryTrait> DogServiceTrait for DogService<R> {
    fn add_dog(&self, dog: Dog) -> impl std::future::Future<Output = ()> + Send {
        async move {
            self.dog_repository.write().await.add_dog(dog).await;
        }
    }

    fn get_dogs(&self) -> impl std::future::Future<Output = Vec<Dog>> + Send {
        async move {
            let dogs = self.dog_repository.read().await.get_dogs().await;

            let mut processed_dogs = dogs;
            for _ in 0..500 {
                processed_dogs = processed_dogs
                    .into_iter()
                    .filter(|dog| dog.age > 1)
                    .map(|dog| Dog {
                        id: format!("{}_processed", dog.id),
                        name: dog.name.to_uppercase(),
                        age: dog.age,
                    })
                    .collect();
            }

            processed_dogs
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppState<
    D: DogServiceTrait,
    G: GroomingServiceTrait,
    T: TrainingServiceTrait,
    H: HealthServiceTrait,
    DH: DogHouseServiceTrait,
> {
    pub dog_service: Arc<D>,
    pub grooming_service: Arc<G>,
    pub training_service: Arc<T>,
    pub health_service: Arc<H>,
    pub dog_house_service: Arc<DH>,
}

pub async fn do_stuff<
    D: DogServiceTrait,
    G: GroomingServiceTrait,
    T: TrainingServiceTrait,
    H: HealthServiceTrait,
    DH: DogHouseServiceTrait,
>(
    State(state): State<AppState<D, G, T, H, DH>>,
) -> impl IntoResponse {
    let dogs = state.dog_service.get_dogs().await;

    let mut results = Vec::new();

    for dog in dogs {
        let grooming_history = state.grooming_service.get_grooming_history(&dog.id).await;
        let total_grooming_cost = state
            .grooming_service
            .calculate_total_grooming_cost(&dog.id)
            .await;

        let training_history = state.training_service.get_training_history(&dog.id).await;
        let skills = state.training_service.get_dog_skills(&dog.id).await;

        let health_history = state.health_service.get_health_history(&dog.id).await;
        let weight_history = state.health_service.get_dog_weight_history(&dog.id).await;

        let dog_house = state.dog_house_service.get_dog_house(&dog.id).await;

        let dog_info = serde_json::json!({
            "dog": dog,
            "grooming": {
                "history": grooming_history,
                "total_cost": total_grooming_cost
            },
            "training": {
                "history": training_history,
                "skills": skills
            },
            "health": {
                "history": health_history,
                "weight_history": weight_history
            },
            "housing": dog_house
        });

        results.push(dog_info);
    }

    let available_houses = state.dog_house_service.get_available_houses().await;

    let response = serde_json::json!({
        "dogs_info": results,
        "available_houses": available_houses
    });

    (StatusCode::OK, Json(response))
}

pub async fn state() -> AppState<
    DogService<DogRepository>,
    GroomingService,
    TrainingService,
    HealthService,
    DogHouseService,
> {
    let repository = DogRepository::new();
    let dog_repository = Arc::new(RwLock::new(repository));
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
    let grooming_service = Arc::new(GroomingService::new());
    let training_service = Arc::new(TrainingService::new());
    let health_service = Arc::new(HealthService::new());
    let dog_house_service = Arc::new(DogHouseService::new());

    AppState {
        dog_service,
        grooming_service,
        training_service,
        health_service,
        dog_house_service,
    }
}

pub async fn router() -> Router {
    let app_state = state().await;

    Router::new()
        .route("/stuff", get(do_stuff))
        .with_state(app_state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum_test::TestServer;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_do_stuff_with_mock() {
        #[derive(Debug, Clone)]
        struct MockDogService {
            dogs: Vec<Dog>,
        }

        
        impl DogServiceTrait for MockDogService {
            fn add_dog(&self, _dog: Dog) -> impl std::future::Future<Output = ()> + Send {
                async move {
                    unreachable!()
                }
            }

            fn get_dogs(&self) -> impl std::future::Future<Output = Vec<Dog>> + Send {
                async move {
                    self.dogs.clone()
                }
            }
        }

        #[derive(Debug, Clone)]
        struct MockGroomingService {}

        
        impl GroomingServiceTrait for MockGroomingService {
            fn add_grooming_record(&self, _record: GroomingRecord) -> impl std::future::Future<Output = ()> + Send {
                async move {
                    // Mock implementation
                }
            }

            fn get_grooming_history(&self, _dog_id: &str) -> impl std::future::Future<Output = Vec<GroomingRecord>> + Send {
                async move {
                    vec![]
                }
            }

            fn calculate_total_grooming_cost(&self, _dog_id: &str) -> impl std::future::Future<Output = f64> + Send {
                async move {
                    150.0
                }
            }
        }

        #[derive(Debug, Clone)]
        struct MockTrainingService {}

        
        impl TrainingServiceTrait for MockTrainingService {
            fn add_training_record(&self, _record: TrainingRecord) -> impl std::future::Future<Output = ()> + Send {
                async move {
                    // Mock implementation
                }
            }

            fn get_training_history(&self, _dog_id: &str) -> impl std::future::Future<Output = Vec<TrainingRecord>> + Send {
                async move {
                    vec![]
                }
            }

            fn get_dog_skills(&self, _dog_id: &str) -> impl std::future::Future<Output = Vec<String>> + Send {
                async move {
                    vec!["Sit".to_string(), "Stay".to_string()]
                }
            }
        }

        #[derive(Debug, Clone)]
        struct MockHealthService {}

        
        impl HealthServiceTrait for MockHealthService {
            fn add_health_record(&self, _record: HealthRecord) -> impl std::future::Future<Output = ()> + Send {
                async move {
                    // Mock implementation
                }
            }

            fn get_health_history(&self, _dog_id: &str) -> impl std::future::Future<Output = Vec<HealthRecord>> + Send {
                async move {
                    vec![]
                }
            }

            fn get_dog_weight_history(&self, _dog_id: &str) -> impl std::future::Future<Output = Vec<(String, f64)>> + Send {
                async move {
                    vec![
                        ("2024-01-01".to_string(), 10.5),
                        ("2024-02-01".to_string(), 11.2),
                    ]
                }
            }
        }

        #[derive(Debug, Clone)]
        struct MockDogHouseService {}

        
        impl DogHouseServiceTrait for MockDogHouseService {
            fn add_dog_house(&self, _house: DogHouse) -> impl std::future::Future<Output = ()> + Send {
                async move {
                    // Mock implementation
                }
            }

            fn assign_dog_to_house(&self, _dog_id: &str, _house_id: &str) -> impl std::future::Future<Output = ()> + Send {
                async move {
                    // Mock implementation
                }
            }

            fn get_dog_house(&self, _dog_id: &str) -> impl std::future::Future<Output = Option<DogHouse>> + Send {
                async move {
                    Some(DogHouse {
                        id: "house1".to_string(),
                        size: "MEDIUM".to_string(),
                        material: "Wood".to_string(),
                        assigned_dog_id: Some("1".to_string()),
                    })
                }
            }

            fn get_available_houses(&self) -> impl std::future::Future<Output = Vec<DogHouse>> + Send {
                async move {
                    vec![DogHouse {
                        id: "house2".to_string(),
                        size: "LARGE".to_string(),
                        material: "Metal".to_string(),
                        assigned_dog_id: None,
                    }]
                }
            }
        }

        let mock_dog_service = Arc::new(MockDogService {
            dogs: vec![Dog {
                id: "1".to_string(),
                name: "TestDog".to_string(),
                age: 3,
            }],
        });

        let app_state = AppState {
            dog_service: mock_dog_service,
            grooming_service: Arc::new(MockGroomingService {}),
            training_service: Arc::new(MockTrainingService {}),
            health_service: Arc::new(MockHealthService {}),
            dog_house_service: Arc::new(MockDogHouseService {}),
        };

        let app = Router::new()
            .route("/stuff", get(do_stuff))
            .with_state(app_state);

        let server = TestServer::new(app).unwrap();
        let response = server.get("/stuff").await;

        assert_eq!(response.status_code(), StatusCode::OK);

        let json_response = response.json::<serde_json::Value>();

        // Verify dogs_info
        let dogs_info = json_response["dogs_info"].as_array().unwrap();
        assert_eq!(dogs_info.len(), 1);

        // Verify dog data
        assert_eq!(dogs_info[0]["dog"]["id"], "1");
        assert_eq!(dogs_info[0]["dog"]["name"], "TestDog");
        assert_eq!(dogs_info[0]["dog"]["age"], 3);

        // Verify grooming data
        assert_eq!(dogs_info[0]["grooming"]["total_cost"], 150.0);

        // Verify training data
        let skills = dogs_info[0]["training"]["skills"].as_array().unwrap();
        assert_eq!(skills.len(), 2);
        assert!(skills.contains(&serde_json::json!("Sit")));

        // Verify available houses
        let available_houses = json_response["available_houses"].as_array().unwrap();
        assert_eq!(available_houses.len(), 1);
        assert_eq!(available_houses[0]["size"], "LARGE");
    }
}
