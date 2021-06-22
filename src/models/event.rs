use diesel::prelude::*;
use diesel::PgConnection;
use chrono::NaiveDateTime;

use crate::schema::events;
use crate::schema::events::dsl::*;

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub location: String,
    pub max_size: i32,
    pub budget: String,
    pub description: String,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
    pub organizer_id: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "events"]
pub struct EventDTO {
    pub name: String,
    pub location: String,
    pub max_size: i32,
    pub budget: String,
    pub description: String,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
    pub organizer_id: i32,
}

impl Event {
    pub fn create(e: EventDTO, user_id: i32, conn: &PgConnection) -> QueryResult<usize> {
        let event = EventDTO {
            organizer_id: user_id,
            ..e
        };

        diesel::insert_into(events).values(&event).execute(conn)
    }

    pub fn find_by_id(i: i32, conn: &PgConnection) -> Option<Event> {
        let result_event = events.find(i).get_result::<Event>(conn);
        if let Ok(event) = result_event {
            Some(event)
        } else {
            None
        }
    }
}
