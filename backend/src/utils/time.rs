use tracing::error;
use tokio::sync::{mpsc,oneshot};
use chrono::prelude::*;
use uuid::{NoContext, Timestamp, Uuid};

use crate::definitions::{GetTimeQuery,TimeandUUID};

#[tracing::instrument(name="time_and_uuid_worker")]
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

#[tracing::instrument(name="get_time_and_uuid_helper")]
pub async fn get_time_and_uuid(query_tx: &mut mpsc::Sender<GetTimeQuery>, error_patience: usize)->Result<TimeandUUID, std::io::Error>{
    let mut error_num = 0;
    loop{
        let (return_tx, return_rx) = oneshot::channel::<Result<TimeandUUID,std::io::Error>>();
        let query = GetTimeQuery{tx: return_tx};
        query_tx.send(query);
        let Ok(time_and_uuid) = return_rx.await else{
            error!("message dropped while mpsc channel!");
            error_num += 1;
            if error_num<error_patience{
                continue;
            }else{
                return Err(std::io::Error::new(std::io::ErrorKind::TimedOut, "message dropped while mpsc channel!"));
            }
        };
        return time_and_uuid;
    }
}
