use elasticsearch::{Elasticsearch, IndexParts, http::{response::Response, transport::Transport}};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), String> {
    let transport: Transport = Transport::single_node("http://localhost:9200/").map_err(|e: elasticsearch::Error| {
        println!("{e}");
        e.to_string()
    })?;
    let client: Elasticsearch = Elasticsearch::new(transport);
    let logs: Vec<serde_json::Value> = vec![
        json!({
            "id": 1,
            "user": "kimchy",
            "post_date": "2009-11-15T00:00:00Z",
            "message": "Trying out Elasticsearch, so far so good?"
        }),
        json!({
            "id": 2,
            "user": "data_explorer",
            "post_date": "2024-03-10T14:20:00Z",
            "message": "Just finished building my first Rust microservice with Actix-web!"
        }),
        json!({
            "id": 3,
            "user": "elastic_fan",
            "post_date": "2024-03-09T09:15:30Z",
            "message": "Exploring the new vector search capabilities in Elasticsearch 8.12"
        }),
        json!({
            "id": 4,
            "user": "dev_ops_guru",
            "post_date": "2024-03-08T16:45:22Z",
            "message": "Kubernetes cluster migration completed successfully with zero downtime"
        }),
        json!({
            "id": 5,
            "user": "code_artist",
            "post_date": "2024-03-07T11:30:15Z",
            "message": "Refactoring legacy JavaScript to TypeScript - type safety is amazing"
        }),
        json!({
            "id": 6,
            "user": "cloud_nomad",
            "post_date": "2024-03-06T18:10:45Z",
            "message": "Terraform modules make multi-cloud deployment so much easier"
        }),
        json!({
            "id": 7,
            "user": "ai_researcher",
            "post_date": "2024-03-05T13:25:10Z",
            "message": "Training a new BERT model for document classification"
        }),
        json!({
            "id": 8,
            "user": "security_watch",
            "post_date": "2024-03-04T10:05:33Z",
            "message": "Implementing zero-trust architecture for our microservices"
        }),
        json!({
            "id": 9,
            "user": "db_optimizer",
            "post_date": "2024-03-03T15:40:28Z",
            "message": "PostgreSQL query performance improved 10x with proper indexing"
        }),
        json!({
            "id": 10,
            "user": "frontend_master",
            "post_date": "2024-03-02T12:15:50Z",
            "message": "React 18 concurrent features are game-changing for UX"
        }),
        json!({
            "id": 11,
            "user": "mobile_dev",
            "post_date": "2024-03-01T19:20:18Z",
            "message": "Flutter 3.0 makes cross-platform development smoother than ever"
        }),
        json!({
            "id": 12,
            "user": "qa_automation",
            "post_date": "2024-02-29T08:55:42Z",
            "message": "Cypress + GitHub Actions = perfect CI/CD testing pipeline"
        }),
        json!({
            "id": 13,
            "user": "data_viz",
            "post_date": "2024-02-28T17:30:25Z",
            "message": "D3.js visualizations with real-time Elasticsearch data streams"
        }),
        json!({
            "id": 14,
            "user": "backend_architect",
            "post_date": "2024-02-27T14:12:37Z",
            "message": "GraphQL Federation is solving our API gateway bottlenecks"
        }),
        json!({
            "id": 15,
            "user": "kimchy",
            "post_date": "2024-02-26T21:05:14Z",
            "message": "Elasticsearch aggregations help us understand user behavior patterns"
        })
    ];
    let mut myi = 0;
    for i in logs {
        let response: Response = client.index(IndexParts::IndexId("tweets", &myi.clone().to_string())).body(i.clone()).send().await
            .map_err(|e: elasticsearch::Error| e.to_string())?;
        let successful: bool = response.status_code().is_success();
        myi += 1;
        if successful {
            println!("{:#?} Доставлен", i);
        } else {
            println!("{:#?} НЕ Доставлен", i);
        }
    }

    Ok(())
}
