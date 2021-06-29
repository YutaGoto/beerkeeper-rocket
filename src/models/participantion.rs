use diesel::prelude::*;
use diesel::PgConnection;

use crate::schema::participations;
use crate::schema::participations::dsl::*;
use crate::models::event::Event;
use crate::models::user::User;

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct Participation {
    pub id: i32,
    pub event: i32,
    pub user: i32
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "participations"]
pub struct ParticipationInsert {
    pub user_id: i32,
    pub event_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct ParticipationUser {
    pub user: User
}

#[derive(Serialize, Deserialize)]
pub struct ParticipationEvent {
    pub event: Event
}

impl Participation {
    pub fn attach_event(self, event: Event) -> ParticipationEvent {
        ParticipationEvent {
            event: event,
        }
    }

    pub fn attach_user(self, user: User) -> ParticipationUser {
        ParticipationUser {
            user: user,
        }
    }

    pub fn create(u_id: i32, e_id: i32, conn: &PgConnection) -> QueryResult<usize> {
        let participation = ParticipationInsert {
            event_id: e_id,
            user_id: u_id
        };

        diesel::insert_into(participations).values(&participation).execute(conn)
    }
}
