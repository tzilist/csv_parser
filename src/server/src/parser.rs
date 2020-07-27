use actix_web::{error, web::Bytes, HttpResponse, Result as HttpResult};
use atone::Vc;
use serde::{Deserialize, Serialize};

use crate::errors::ServerErrors;

#[derive(Deserialize, Debug, Clone)]
struct CsvRecord {
    id: String,
    email: String,
    name: String,
    is_parent: u8,
}

#[derive(Clone, Debug, Serialize)]
struct ResponseRecord {
    id: String,
    email: String,
    name: String,
    is_parent: bool,
}

/// Parses a CSV body from bytes and converts the CSV into an array of records
/// This is then returned as JSON
pub async fn parse_csv(body: Bytes) -> HttpResult<HttpResponse> {
    // it would be better to create a stream here
    // out of laziness we are going to just collect the bytes for now and pass to CSV reader
    let mut reader = csv::Reader::from_reader(body.as_ref());
    let mut raw_record = csv::ByteRecord::new();
    let headers = reader.byte_headers().map_err(ServerErrors::from)?.clone();

    // use atone here to amortize vec memory allocation across all inserts
    // this potentially could be slower for small vecs, however, for large CSV files,
    // this will prevent pauses while reallocating
    let mut results = Vc::new();

    while reader
        .read_byte_record(&mut raw_record)
        .map_err(ServerErrors::from)?
    {
        let record: CsvRecord = raw_record
            .deserialize(Some(&headers))
            .map_err(ServerErrors::from)?;

        if record.is_parent != 0 && record.is_parent != 1 {
            return Err(error::ErrorBadRequest(format!(
                "Invalid \"is_parent\" found in record, set to {}",
                &record.is_parent,
            )));
        }

        results.push(ResponseRecord {
            id: record.id,
            email: record.email,
            name: record.name,
            // convert from CSV weirdness, assume 0 is "true", 1 is "false"
            is_parent: if record.is_parent == 0 { true } else { false },
        });
    }

    Ok(HttpResponse::Ok().json(results).into())
}

#[cfg(test)]
mod test {
    use super::*;
    use actix_web::{guard, http, test, web, App};

    #[actix_rt::test]
    async fn test_parse() {
        let mut app = test::init_service(
            App::new().route(
                "/api/parse",
                web::post()
                    .guard(guard::Header("content-type", "text/csv"))
                    .to(parse_csv),
            ),
        )
        .await;

        let body = r#"
id,email,name,is_parent
RvVOM5BnJx2ZRQRkOc4pFfqhCap2JboJvZCHuWnP,Sylvester41@hotmail.com,Rowland Lowe,1
"#;
        let req = test::TestRequest::post()
            .header("content-type", "text/csv")
            .uri("/api/parse")
            .set_payload(body.as_bytes())
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    // Commenting this out as there seems to be a panic that is happening somewhere in
    // the CSV crate or serde crate due to a stack overflow
    // Normally, I'd like to test that bad requests handle the response correctly
    //
    // #[actix_rt::test]
    // async fn test_parse_bad_csv() {
    // let mut app = test::init_service(
    // App::new().route(
    // "/api/parse",
    // web::post()
    // .guard(guard::Header("content-type", "text/csv"))
    // .to(parse_csv),
    // ),
    // )
    // .await;

    // let body = r#"
    // id,email,name,is_parent
    // RvVOM5BnJx2ZRQRkOc4pFfqhCap2JboJvZCHuWnP,Sylvester41@hotmail.com,Rowland Lowe,1
    // "#;
    // let req = test::TestRequest::post()
    // .header("content-type", "text/csv")
    // .uri("/api/parse")
    // .set_payload(body.as_bytes())
    // .to_request();

    // let resp = test::call_service(&mut app, req).await;
    // assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    // }

    #[actix_rt::test]
    async fn test_parse_missing_header() {
        let mut app = test::init_service(
            App::new().route(
                "/api/parse",
                web::post()
                    .guard(guard::Header("content-type", "text/csv"))
                    .to(parse_csv),
            ),
        )
        .await;

        let body = r#"
id,email,name,is_parent
RvVOM5BnJx2ZRQRkOc4pFfqhCap2JboJvZCHuWnP,Sylvester41@hotmail.com,Rowland Lowe,1
"#;
        let req = test::TestRequest::post()
            .uri("/api/parse")
            .set_payload(body.as_bytes())
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);
    }
}
