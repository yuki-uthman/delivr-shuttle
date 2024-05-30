use crate::helpers::setup_app;
use crate::error::Result;

#[tokio::test]
async fn insert_invoice() -> Result<()> {
    let app = setup_app().await?;

    let json = r#"{
        "invoice": {
            "created_by_name": "yuki.uthman",
            "created_date": "2024-05-27",
            "customer_id": "4332607000000089589",
            "customer_name": "1. indon 2",
            "date": "2024-05-27",
            "invoice_id": "4332607000000182746",
            "invoice_number": "INV-000086",
            "line_items": [
                {
                    "item_id": "4332607000000182461",
                    "item_total": 60,
                    "line_item_id": "4332607000000182754",
                    "name": "Cosmetics RM5",
                    "purchase_rate": 5,
                    "quantity": 10,
                    "rate": 6
                }
            ],
            "salesperson_name": "Yuki",
            "status": "draft",
            "total": 60
        }
    }"#;

    let client = reqwest::Client::new();
    let response = client
        .post(format!("http://{}/post", app.address()))
        .header("Content-Type", "application/json")
        .body(json)
        .send()
        .await?;

    assert_eq!(response.status(), 200);

    let res = client
        .get(format!("http://{}/invoices?date=2024-05-27", app.address()))
        .header("Content-Type", "application/json")
        .body(json)
        .send()
        .await?;

    println!("{:#?}", res.json::<serde_json::Value>().await?);
    todo!();

    Ok(())
}
