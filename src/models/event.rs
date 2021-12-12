use diesel::prelude::*;
use diesel::PgConnection;

use crate::models::participantion::Participation;
use crate::models::user::User;
use crate::schema::events::dsl::*;
use crate::schema::participations::event_id;
use crate::schema::{events, participations, users};

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
    pub organizer: i32,
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

#[derive(Serialize, Deserialize)]
pub struct EventInfo {
    pub id: i32,
    pub name: String,
    pub location: Option<String>,
    pub max_size: i32,
    pub budget: Option<String>,
    pub description: Option<String>,
    pub start_at: Option<String>,
    pub end_at: Option<String>,
    pub organizer: User,
    pub participations: Vec<Participation>,
}

impl Event {
    pub fn attach(self, organizer: User, participations: Vec<Participation>) -> EventInfo {
        EventInfo {
            id: self.id,
            name: self.name,
            location: self.location,
            max_size: self.max_size,
            budget: self.budget,
            description: self.description,
            start_at: self.start_at,
            end_at: self.end_at,
            organizer,
            participations,
        }
    }

    pub fn create(ev: EventDTO, user_id: i32, conn: &PgConnection) -> Option<Event> {
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

        let result_event = diesel::insert_into(events).values(&event).get_result(conn);

        if let Ok(event) = result_event {
            Some(event)
        } else {
            None
        }
    }

    pub fn find_by_id(i: i32, conn: &PgConnection) -> Option<EventInfo> {
        let result_event = events.find(i).get_result::<Event>(conn);

        if let Ok(event) = result_event {
            Some(populate(conn, event))
        } else {
            None
        }
    }
}

fn populate(conn: &PgConnection, event: Event) -> EventInfo {
    let organizer = users::table
        .find(event.organizer)
        .get_result::<User>(conn)
        .expect("Error loading author");

    let participation_list = participations::table
        .filter(event_id.eq(event.id))
        .load::<Participation>(conn)
        .expect("Error");

    event.attach(organizer, participation_list)
}
