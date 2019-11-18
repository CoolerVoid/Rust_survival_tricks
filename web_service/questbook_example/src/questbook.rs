use diesel::{self, prelude::*};

mod schema {
    table! {
        questbooks {
            id -> Nullable<Integer>,
            uid -> Text,
            message -> Text,
        }
    }
}

use self::schema::questbooks;
use self::schema::questbooks::dsl::{questbooks as all_questbooks};

#[table_name="questbooks"]
#[derive(Serialize, Queryable, Insertable, Debug, Clone)]
pub struct Questbook {
    pub id: Option<i32>,
    pub uid: String,
    pub message: String,
}

#[derive(FromForm)]
pub struct NewQuest {
    pub uid: String,
    pub message: String,
}


#[derive(FromForm)]
pub struct DeleteQuest {
    pub id: String,
}

#[derive(FromForm)]
pub struct UpdateQuest {
    pub id: String,
    pub uid: String,
    pub message: String,
}


impl Questbook {
    pub fn all(conn: &SqliteConnection) -> Vec<Questbook> {
        all_questbooks.order(questbooks::id.desc()).load::<Questbook>(conn).unwrap()
    }

    pub fn insert(questbook: NewQuest, conn: &SqliteConnection) -> bool {
        let t = Questbook { id: None, uid: questbook.uid, message: questbook.message };
        diesel::insert_into(questbooks::table).values(&t).execute(conn).is_ok()
    }


    pub fn delete(questbook: DeleteQuest, conn: &SqliteConnection) -> bool {
        let my_int_id: i32 = questbook.id.parse().unwrap();
        diesel::delete(questbooks::table).filter(questbooks::id.eq(my_int_id)).execute(conn).is_ok()
    }

    pub fn update(questbook: UpdateQuest, conn: &SqliteConnection) -> bool {
        let int_id: i32 = questbook.id.parse().unwrap();
        diesel::update(questbooks::table).filter(questbooks::id.eq(int_id))
            .set( (questbooks::uid.eq(questbook.uid), questbooks::message.eq(questbook.message)) )
            .execute(conn)
            .is_ok()
    }

}
