use crate::schema::dogs;
use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;

#[derive(Serialize, AsChangeset, Deserialize, Queryable, Insertable)]
#[table_name = "Dog"]
pub struct Dog {
    pub id: Option<i32>,
    pub breed: Option<String>,
    pub color: Option<String>,
}

impl Dog {
    pub fn create(dog: Dog, conn: &MysqlConnection) -> Result<Dog, Error> {
        let new_dog = Dog { ..dog };

        let ops = diesel::insert_into(dogs::table)
            .values(&new_dog)
            .execute(conn);

        match ops {
            Ok(_) => dogs::table.order(dogs::id.desc()).first(conn),
            Err(e) => Err(e),
        }
    }

    pub fn read(conn: &MysqlConnection) -> Result<Vec<Dog>, Error> {
        dogs::table.order(dogs::id.asc()).load::<Dog>(conn)
    }

    pub fn read_by_id(id: i32, conn: &MysqlConnection) -> Result<Dog, Error> {
        dogs::table.find(id).first(conn)
    }

    pub fn update(id: i32, dog: Dog, conn: &MysqlConnection) -> Result<Dog, Error> {
        let new_dog = Dog { ..dog };

        let ops = diesel::update(dogs::table.find(id))
            .set(&new_dog)
            .execute(conn);

        match ops {
            Ok(_) => dogs::table.find(id).first(conn),
            Err(e) => Err(e),
        }
    }

    pub fn delete(id: i32, conn: &MysqlConnection) -> bool {
        diesel::delete(dogs::table.find(id)).execute(conn).is_ok()
    }
}