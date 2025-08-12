use tracing::error;
use tokio::sync::{mpsc,oneshot};
use chrono::prelude::*;

use crate::definitions::GetTimeQuery;

#[tracing::instrument(name="time_worker")]
pub async fn get_time_worker(rx: &mut mpsc::Receiver<GetTimeQuery>){
    loop{
        let Some(query) = rx.recv().await else{
            error!("query not found!");
            continue;
        };
        // UTCで時間取得
        let utc_datetime: DateTime<Utc> = Utc::now();

        if let Err(_) = query.tx.send(Ok(utc_datetime)){
            error!("Failed to send the result back to the caller.");
        }
    }
}

#[tracing::instrument(name="get_time_helper")]
pub async fn get_time(query_tx: mpsc::Sender<GetTimeQuery>, error_patience: usize)->Result<DateTime<Utc>, std::io::Error>{
    let mut error_num = 0;
    loop{
        let (return_tx, return_rx) = oneshot::channel::<Result<DateTime<Utc>,std::io::Error>>();
        let query = GetTimeQuery{tx: return_tx};
        if let Err(e) = query_tx.send(query).await{
            error!("Failed to send query: {}",e);
            return Err(std::io::Error::new(std::io::ErrorKind::BrokenPipe, "message send failed"));
        };
        let Ok(time) = return_rx.await else{
            error!("message dropped while mpsc channel!");
            error_num += 1;
            if error_num<error_patience{
                continue;
            }else{
                return Err(std::io::Error::new(std::io::ErrorKind::TimedOut, "message dropped while mpsc channel!"));
            }
        };
        return time;
    }
}
