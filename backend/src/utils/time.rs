use tracing::error;
use tokio::sync::mpsc;
use chrono::prelude::*;
use uuid::{NoContext, Timestamp, Uuid};

use crate::definitions::{GetTimeQuery,TimeandUUID};

#[tracing::instrument(name="get_time_and_uuid")]
pub async fn get_time_and_uuid_task(rx: &mut mpsc::Receiver<GetTimeQuery>){
    loop{
        let Some(query) = rx.recv().await else{
            error!("query not found!");
            continue;
        };
        // UTCで時間取得
        let utc_datetime: DateTime<Utc> = Utc::now();

        // Uuid v7でuuid作成
        let ts = Timestamp::from_unix(
            NoContext,
            utc_datetime.timestamp() as u64,
            utc_datetime.timestamp_subsec_nanos(),
        );
        let id = Uuid::new_v7(ts);

        let time_and_uuid = TimeandUUID{time: utc_datetime,uuid:id};
        if let Err(_) = query.tx.send(Ok(time_and_uuid)){
            error!("Failed to send the result back to the caller.");
        }
    }
}
