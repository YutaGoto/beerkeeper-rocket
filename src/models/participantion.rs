use diesel::prelude::*;
use diesel::PgConnection;

use crate::models::event::Event;
use crate::models::user::User;
use crate::schema::participations;
use crate::schema::participations::dsl::*;

#[derive(Identifiable, Queryable, Serialize, Deserialize, Associations)]
#[belongs_to(User)]
pub struct Participation {
    pub id: i32,
    pub event: i32,
    pub user_id: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "participations"]
pub struct ParticipationInsert {
    pub user_id: i32,
    pub event_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct ParticipationUser {
    pub user: User,
}

#[derive(Serialize, Deserialize)]
pub struct ParticipationEvent {
    pub event: Event,
}

impl Participation {
    pub fn attach_event(self, event: Event) -> ParticipationEvent {
        ParticipationEvent { event }
    }

    pub fn attach_user(self, user: User) -> ParticipationUser {
        ParticipationUser { user }
    }

    pub fn create(u_id: i32, e_id: i32, conn: &PgConnection) -> QueryResult<usize> {
        let participation = ParticipationInsert {
            event_id: e_id,
            user_id: u_id,
        };

        diesel::insert_into(participations)
            .values(&participation)
            .execute(conn)
    }

    pub fn delete(p_id: i32, conn: &PgConnection) -> QueryResult<usize> {
        diesel::delete(participations.filter(id.eq(p_id))).execute(conn)
    }

    pub fn find_by_user_and_event(
        u_id: i32,
        e_id: i32,
        conn: &PgConnection,
    ) -> Option<Participation> {
        let result_participation = participations
            .filter(user_id.eq(u_id))
            .filter(event_id.eq(e_id))
            .get_result::<Participation>(conn);

        if let Ok(participantion) = result_participation {
            Some(participantion)
        } else {
            None
        }
    }
}
