// Object {
//     "invoice": Object {
//         "created_by_name": String("yuki.uthman"),
//         "created_date": String("2024-05-27"),
//         "customer_id": String("4332607000000089589"),
//         "customer_name": String("1. indon 2"),
//         "date": String("2024-05-27"),
//         "invoice_id": String("4332607000000182746"),
//         "invoice_number": String("INV-000086"),
//         "line_items": Array [
//             Object {
//                 "item_custom_fields": Array [],
//                 "item_id": String("4332607000000182461"),
//                 "item_order": Number(1),
//                 "item_total": Number(60),
//                 "line_item_id": String("4332607000000182754"),
//                 "name": String("Cosmetics RM5 Tracked"),
//                 "purchase_rate": Number(5),
//                 "quantity": Number(10),
//                 "rate": Number(6),
//             },
//         ],
//         "salesperson_name": String("Yuki"),
//         "status": String("draft"),
//         "total": Number(60),
//     },
// }

#[tokio::main]
async fn main() {
    // send the json to a webhook 127.0.0.1:8000/post
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

    let res = client
        .post("http://127.0.0.1:8000/post")
        .header("Content-Type", "application/json")
        .body(json)
        .send()
        .await
        .unwrap();

    println!("{:#?}", res);
}
