use actix_web::{error, web::Bytes, HttpResponse, Result as HttpResult};
use atone::Vc;
use serde::{Deserialize, Serialize};

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
    let headers = reader.byte_headers().unwrap().clone();

    // use atone here to amortize vec memory allocation across all inserts
    // this potentially could be slower for small vecs, however, for large CSV files,
    // this will prevent pauses while reallocating
    let mut results = Vc::new();

    while reader
        .read_byte_record(&mut raw_record)
        .map_err(|e| error::ErrorBadRequest(format!("Failed to parse CSV record, {}", e)))?
    {
        let record: CsvRecord = raw_record
            .deserialize(Some(&headers))
            .map_err(|e| error::ErrorBadRequest(format!("Failed to parse CSV record, {}", e)))?;

        results.push(ResponseRecord {
            id: record.id,
            email: record.email,
            name: record.name,
            is_parent: if record.is_parent == 0 { true } else { false },
        });
    }

    Ok(HttpResponse::Ok().json(results).into())
}
