use diesel::prelude::*;
use diesel::PgConnection;

use crate::schema::events;
use crate::schema::events::dsl::*;

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub location: Option<String>,
    pub max_size: i32,
    pub budget: Option<String>,
    pub description: Option<String>,
    pub start_at: Option<String>,
    pub end_at: Option<String>,
    pub organizer_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct EventDTO {
    pub name: String,
    pub location: Option<String>,
    pub max_size: i32,
    pub budget: Option<String>,
    pub description: Option<String>,
    pub start_at: Option<String>,
    pub end_at: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "events"]
pub struct EventInsert {
    pub name: String,
    pub location: Option<String>,
    pub max_size: i32,
    pub budget: Option<String>,
    pub description: Option<String>,
    pub start_at: Option<String>,
    pub end_at: Option<String>,
    pub organizer_id: i32,
}

impl Event {
    pub fn create(ev: EventDTO, user_id: i32, conn: &PgConnection) -> QueryResult<usize> {
        let event = EventInsert {
            name: ev.name,
            location: ev.location,
            max_size: ev.max_size,
            budget: ev.budget,
            description: ev.description,
            start_at: ev.start_at,
            end_at: ev.end_at,
            organizer_id: user_id,
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
