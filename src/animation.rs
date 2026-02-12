use crate::types::states;
use crate::input;

pub fn run(cur_state: &states) -> String {
	//call input.rs's update state, passing through the current state
	let new_state = input::update_state(cur_state);
	/* 
	 * 
	 * 
	 * 
	 * 
	 * eventually amend this burh
	 */
	return new_state.anim.to_string();
}

