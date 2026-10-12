mod database;
mod models;

use models::VaultItem;

use rusqlite::Connection;

use tauri::command;

use rand::{thread_rng, Rng};



#[command]
fn add_item(
    name: String,
    login: String,
    secret: String
) {

    let db =
        Connection::open("vault.db")
        .unwrap();


    db.execute(
        "
        INSERT INTO vault
        (name,login,secret)

        VALUES(?1,?2,?3)
        ",
        [
            &name,
            &login,
            &secret
        ]
    )
    .unwrap();

}



#[command]
fn list_items() -> Vec<VaultItem> {

    let db =
        Connection::open("vault.db")
        .unwrap();


    let mut query =
        db.prepare(
            "SELECT id,name,login,secret FROM vault"
        )
        .unwrap();



    let rows =
        query.query_map([], |row| {

            Ok(
                VaultItem {

                    id: row.get(0)?,

                    name: row.get(1)?,

                    login: row.get(2)?,

                    secret: row.get(3)?

                }
            )

        })
        .unwrap();



    rows.map(|item| item.unwrap()).collect()

}



#[command]
fn remove_item(id:i32) {


    let db =
        Connection::open("vault.db")
        .unwrap();


    db.execute(
        "DELETE FROM vault WHERE id=?1",
        [id]
    )
    .unwrap();

}



#[command]
fn generate() -> String {


    let chars =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%";


    let mut rng =
        thread_rng();



    (0..20)
        .map(|_| {

            let index =
                rng.gen_range(0..chars.len());


            chars
                .chars()
                .nth(index)
                .unwrap()

        })
        .collect()

}



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {


    database::init();



    tauri::Builder::default()

        .invoke_handler(
            tauri::generate_handler![

                add_item,

                list_items,

                remove_item,

                generate

            ]
        )

        .run(
            tauri::generate_context!()
        )

        .expect("error while running CyberVault");


}