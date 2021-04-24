/*ToDo: Make sure it works with serenity shards
 */

use rusqlite::{
	types::ToSql,
	Connection,
	Result,
};

use time::{
	Timespec,
	SystemTime,
	UNIX_EPOCH
};

enum ActionTaken {
	Ban,
	Kick,
}

#[derive(Debug)]
struct CcActions
{
	action: ActionTaken,
	id: u32,
	use_name: String,
	actioner_id: u32,
	actioner_user_name: String,
	time_stamp: Timespec,
}

//ToDo: Create data base connections that work with serenity shards
fn create_conection(cc_database_group: String) -> Result<()>
{
	let conn = Connection::open(path: cc_database_group+".db");

	successful_tx(&conn)?;

	let res = rooled_back_tx(&conn);
	assert!(res.is_err());

	Ok(())
}

//ToDo: Set the cross community group for a given guild
fn set_guild_cc() -> Result<()>
{
	Ok(())
}

//ToDo: Get the cross community group for a given guild
fn get_guild_cc() -> Result<()>
{

	Ok(())
}


//ToDo:: Return a arrray of bans for a cross community group
fn pull_cc_ban_list(conn: &mut Connection,new_act: CcActions) -> Result<()>
{
	let tx = conn.transaction()?;

	tx.execute("insert into (table) (action, user_id, user_name, actioner_id, actioner_user_name, time_stamp) values (?1, ?2, ?3, ?4, ?5, ?6)",
			   params![new_act.actionTaken, new_act.id, new_act.userName,
			new_act.actioner_id, new_act.actioner_userName, new_act,time_stamp])?;

	tx.commit()
}
//ToDo: complete a insert for kicked and banned memebers of CC tables.
fn push_new_action(conn: &mut Connection,new_act: CcActions) -> Result<()>
{
	let tx = conn.transaction()?;

	tx.execute("insert into actions (action, user_id, user_name, actioner_id, actioner_user_name, time_stamp) values (?1, ?2, ?3, ?4, ?5, ?6)",
		params![new_act.actionTaken, new_act.id, new_act.userName,
			new_act.actioner_id, new_act.actioner_userName, new_act,time_stamp])?;

	tx.commit()
}

//ToDo: complete a delete for kicked and banned memebers of CC tables.
fn delete_existing_action_by_id(conn: &mut Connection, new_act: CcActions) -> Result<()>
{

	let tx = conn.transaction()?;

	tx.execute("DELETE FROM actions WHERE user_id=(?1)",
			   params![new_act.id])?;

	tx.commit()
}
