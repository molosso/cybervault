use rusqlite::Connection;


pub fn init(){

let db =
Connection::open("vault.db")
.unwrap();



db.execute(

"
CREATE TABLE IF NOT EXISTS vault(

id INTEGER PRIMARY KEY AUTOINCREMENT,

name TEXT,

login TEXT,

secret TEXT

)

",

[]

)

.unwrap();


}