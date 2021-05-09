/*ToDo: Make sure it works with serenity shards, data base structures are ordered, and have strong relationships
 */

use rusqlite::{types::ToSql, Connection, Result, Statement, Transaction, Row, MappedRows, params};

use std::time::{
	Timespec,
	SystemTime,
	UNIX_EPOCH
};
use std::io::empty;
use std::ptr::null;
use std::alloc::Global;

//ToDo: may or may not use sqlite to set time stamps its self and only use the time_stamp files when retrieving data.
//The id of different sql struct may also be created and handled by sqlite server side.
//Right now sqlite server time stamp is used.

trait sqlObject {
	fn delete(&self) -> Result<()>;
	fn update(&self) -> Result<()>;
	fn push(&self)  -> Result<()>;
	fn pull(&mut self) -> Result<()>;
}

struct sqlConn {
	connection: Connection,
}

impl sqlConn {

}

///////////////////////////////////////////////////
//Beginning of passing and returning structures.//
/////////////////////////////////////////////////

//ToDO: Not sure if this will be used to final release. Need a way to store users in a many to one for admins and cross community. M:M for User:CrossCommunity
#[derive(Debug)]
struct User {
	id: u64,
	reputation: u16,
}

//Structure for cross community information
//ToDo: Currently a stub, but needs to hold information about cross community groups such as admins, member guilds in a way that can be encoded for sqlite.
#[derive(Debug)]
struct CrossCommunity {
	conn: Connection,
	id: u32,
	governance_id: u16,
	name: String,
	time_stamp: Timespec,
}

impl sqlObject for CrossCommunity {
	fn delete(&self) -> Result<()> {
		let tx :Transaction = (&self.conn).transaction()?;

		tx.execute("DELETE FROM Cross_Community WHERE id=(?1)",
				   params![self.id])?;

		tx.commit()
	}

	fn update(&self) -> Result<()> {
		let tx :Transaction = (&self.conn).transaction()?;

		tx.execute( "UPDATE Cross_Community SET (name, governace_id) SET  WHERE ID = (id) (?1, ?2, ?3)",
					params![self.name, self.governance_id, self.id])?;

		tx.commit()
	}

	fn push(&self) -> Result<()> {
		let tx :Transaction = (&self.conn).transaction()?;

		tx.execute( "insert into Cross_Community (name, governace_id) values (?1, ?2)",
					params![self.name, self.governance_id])?;

		tx.commit()
	}

	fn pull(&mut self) -> Result<()> {
		let mut stmt: Statement =
			(&self.conn).prepare("SELECT * FROM Cross_Community WHERE id = (?1)")?;

		let i_shouldnt_have_to_do_This :u32 = self.id;
//:MappedRows<fn(&Row)-> Result<CrossCommunity>>
		let mut cc_bd_callback :MappedRows<fn(&Row)-> Result<CrossCommunity>>  = stmt.query_map([i_shouldnt_have_to_do_This], |row| {
			Ok(CrossCommunity {
				conn: ,
				id: row.get(0)?,
				governance_id: row.get(1)?,
				name: row.get(2)?,
				time_stamp: row.get(3)?,
			})
		})?;

		(self = cc_bd_callback.get())?;

		Ok(())
	}


}

//id mapping for governance_id values.
impl CrossCommunity  {
	fn id_value(&self) -> &'static str {
		match self.governance_id {
			n @ 0 => "dictator",
			n @ 1 => "simple majority",
			n @ 2 => "major majority",
			n @ 3 => "oligarchy",
			_=> Err(_),
		}
	}
}


/*	|~~~~~~~~~~~~~~~~~~~~cross community~~~~~~~~~~~~~~~~~~~~~~~~|
 *	|  cc_id  | cc_name	|	user_id  |    admins   | time stamp |
 *	|   PK    |	    	|	  FK	 |		FK	   |		    |
  *	|   INT   |	  TEXT 	|	  INT	 |			   |		    |
 *	|___________________________________________________________|
 */

//ToDo: may use this for admins for a cross community or use the struct above with a blob.
#[derive(Debug)]
struct CrossCommunityUser {
	id: u32,
	user_id: u64,
	cc_id: u64,
	perm_id: u16,
	authorized_id: u64,
	time_stamp: Timespec,
}

//id mapping for perms_id values.
impl CrossCommunityUser {
	fn id_value(&self) -> &'static str {
		match self.perm_id {
			n @ 0 => "user",
			n @ 1 => "admin",
			n @ 2 => "moderator",
			n @ 3 => "restricted",
			_=> "Unknown",
		}
	}
}

/*	|~~~~~~~cross community admins~~~~~~~~~|
 *	|  cc_id	|	admin_id  | time_stamp |
 *	|	 PK 	|	   PK	  |	 		   |
 *	|	 INT 	|	   INT	  |	 		   |
 *	|______________________________________|
 */

//Structure for storing sql cross community incidents rows in rust.
#[derive(Debug)]
struct CrossCommunityActions {
	id: u32,
	cc_id: u64,
	user_id: u64,
	actioner_id: u64,
	action_taken_id: u16,
	time_stamp: Timespec,
	reason: String,
}

//id mapping for issues_id values
impl CrossCommunityActions {
	fn id_value(&self) -> &'static str {
		match self.action_taken_id {
			n @ 0u16 => "other",
			n @ 1u16 => "ban",
			n @ 2u16 => "kick",
			n @ 3u16 => "tempBan",
			n @ 4u16 => "mute",
			_=> "Unknown",
		}
	}
}

/*	|~~~~~~~~~~~~~~~~~~~~~cross community actions~~~~~~~~~~~~~~~~~~~~~~~~~~~~|
 *	| action_id |   cc_id	|	user_id  | actioner_id | time_stamp | reason |
 *	|     PK    |	 FK 	|	  FK	 |		FK	   |		    | 		 |
 *	|     INT   |	 INT 	|	  INT	 |		INT	   |		    |	TEXT |
 *	|________________________________________________________________________|
 */


////////////////////////////////////////////////
//Ending of passing and returning structures.//
//////////////////////////////////////////////

//Structure for storing sql cross community incident rows in rust.
#[derive(Debug)]
struct CcIncident {
	id: u32,
	id_about: u64,
	id_reporter: u64,
	time_stamp: Timespec,
	issue_id: u32,
	issue: String,
}

//id mapping for issues_id values
impl CcIncident {
	fn id_value(&self) -> &'static str {
		match self.issue_id {
			n @ 0 => "other",
			n @ 1 => "doxx",
			n @ 2 => "toxic",
			n @ 3 => "threat",
			_=> "Unknown",
		}
	}
}

/*	|~~~~~~~~~~~~~~~~~~cross community incident~~~~~~~~~~~~~~~~~~~~~|
 *	| incident_id | id_about |	id_reporter |  time_stamp |  issue  |
 *	|   	PK    |	  FK 	 |	   FK	    |	  		  |	        |
  *	|   	INT   |	  INT 	 |	   INT	    |	  		  |	TEXT    |
 *	|_______________________________________________________________|
 */

//Structure for storing sql cross community guild belonging rows in rust.
#[derive(Debug)]
struct CrossCommunityGuildJoin {
	id: u32,
	guild_id: u64,
	cc_id: u32,
	time_stamp: Timespec,
}

//Structure for storing sql cross community guild belonging rows in rust.
#[derive(Debug)]
struct Guild {
	id: u64,
	reputation: u16,
}

/*	|~~~~~~~~~~~~~~~~cross community guilds~~~~~~~~~~~~~~~~~~~~~|
 *	|    id   |  guild_id  |  cross_community_id  | time stamp	|
 *	|   ?PK?  |	  PK/FK	   |		PK/FK		  |		    	|
 *	|___________________________________________________________|
 */

/////////////////////////////////////////////
//End of passing and returning structures.//
///////////////////////////////////////////

////////////////////////////////////
//Create data from SQL data base.//
//////////////////////////////////

//ToDo: Create data base connections that work with serenity shards
fn create_connection(cc_database_group: String) -> Result<Connection>
{
	let conn = Connection::open(cc_database_group+".db")?;

	//successful_tx(&conn)?
}

//ToDo: Complete a insert for a guild into a cross community
//STUB
fn push_guild_cc(conn: &mut Connection, new_ccjoin: &CrossCommunityGuildJoin) -> Result<()>
{
	let tx :Transaction = conn.transaction()?;

	tx.execute("insert into Cross_Community_Guild_Join (cross_community_id, guild_id) values (?1, ?2)",
			   params![new_ccjoin.guild_id, new_ccjoin.cc_id])?;

	tx.commit()
}

//ToDo: complete a insert for discord guilds (servers).
fn push_guild(conn: &mut Connection, new_guild: &Guild, cc_id: u32) -> Result<()>
{
	let tx :Transaction = conn.transaction()?;

	tx.execute("insert into Cross_Community_Guild_Join (cross_community_id, guild_id) values (?1, ?2)",
			   params![new_guild.id, cc_id])?;

	tx.commit()
}


//ToDo: complete a insert for actions against members of CC tables.
fn push_new_cc_action(conn: &mut Connection,new_act: &CrossCommunityActions) -> Result<()>
{
	let tx :Transaction = conn.transaction()?;

	tx.execute( "insert into Cross_Community_Actions (cross_community_id, action_id, user_id, actioner_id, action_taken_id, reason) values (?1, ?2, ?3, ?4, ?5, ?6)",
				params![new_act.cc_id, new_act.action_id, new_act.user_id, new_act.actioner_id, new_act.action_taken_id, new_act.reason])?;

	tx.commit()
}

//ToDo: complete a insert for adding a user to a cross community user.
fn push_new_cc_user(conn: &mut Connection,new_user: &CrossCommunityUser) -> Result<()>
{
	let tx :Transaction = conn.transaction()?;

	tx.execute( "insert into Cross_Community_Users (user_id, cross_community_users, permissions_id, authorized_id) values (?1, ?2, ?3, ?4)",
				params![new_user.id, new_user.cc_id, new_user.perm_id, new_user.authorized_id])?;

	tx.commit()
}

//ToDo: Not sure what I was doing here.
fn push_new_user(conn: &mut Connection,new_user: &User) -> Result<()>
{
	let tx :Transaction = conn.transaction()?;

	tx.execute( "insert into Users (id) values (?1)",
				params![new_user.id])?;

	tx.commit()
}

/////////////////////////////////////
//Pulling data from SQL data base.//
///////////////////////////////////

//ToDo: Get the cross community group's users. May be close to done. IDK rust lmao.
//STUB
fn pull_cc_users(conn: &mut Connection, cc_id: u32) -> Option<Vec<User>>
{
	let mut stmt: Statement =
		conn.prepare("SELECT * FROM Cross_Community_Users WHERE cross_community_id = (?1)")?;

	let cc_guild_bd_callback = stmt.query_map( [cc_id], |row| {
		Ok(CrossCommunityUser{
			id: row.get(0)?,
			user_id: row.get(1)?,
			cc_id: row.get(2)?,
			perm_id: row.get(3)?,
			authorized_id: row.get(4)?,
			time_stamp: row.get(5)?,
		})
	})?;

	return (cc_guild_bd_callback.iter().collect() : Vec<CrossCommunityActions>)?;
}

//ToDo: Return a array of bans for a cross community group
//Stub
fn pull_user_cc_ban_list(conn: &mut Connection, cc_id: u32, action_taken_id: u16) -> Option<Vec<CrossCommunityActions>>
{
	let mut stmt: Statement =
		conn.prepare("SELECT * FROM Cross_Community_Actions WHERE cross_community_id = (?1) AND action_id = (?2)")?;

	let cc_actions_bd_callback =
		stmt.query_map(params![cc_id, action_taken_id], |row| {
		Ok(CrossCommunityActions {
			id: row.get(0)?,
			cc_id: row.get(1)?,
			user_id: row.get(2)?,
			actioner_id: row.get(3)?,
			action_taken_id: row.get(4)?,
			time_stamp: row.get(5)?,
			reason: row.get(6)?,
		})
	})?;

	return (cc_actions_bd_callback.iter().collect() : Vec<CrossCommunityActions>)?;
}

//ToDo: Return a array of bans for a cross community groups by guild membership
//Stub
fn pull_guild_cc(conn: &mut Connection, guild_id: u32) -> Result<Vec<CrossCommunityGuildJoin>>
{
	let mut stmt: Statement =
		conn.prepare("SELECT * FROM Cross_Community_Guild_Join WHERE cross_community_id = (?1)")?;

	let cc_actions_bd_callback = stmt.query_map( [guild_id], |row| {
		Ok(CrossCommunityGuildJoin {
			id: row.get(0)?,
			guild_id: row.get(1)?,
			cc_id: row.get(2)?,
			time_stamp: row.get(3)?,
		})
	})?;

	Ok((cc_actions_bd_callback.to_iter().collect() : Vec<CrossCommunityGuildJoin>))
}

//////////////////////////////////////
//Deleting data from SQL data base.//
////////////////////////////////////

//ToDo: complete a delete for kicked and banned members of CC tables. Make sure you are using the correct API call.
fn delete_existing_action_by_id(conn: &mut Connection, new_act: CrossCommunityActions) -> Result<()>
{
	let tx :Transaction = conn.transaction()?;

	tx.execute("DELETE FROM actions WHERE user_id=(?1)",
			   params![new_act.id])?;

	tx.commit()
}

/*
 *
 *
 */
