use super::*;

pub async fn propstates(_args: &[&str], session: &PlayerSession) -> Result<()> {
    session.context.scene_mgr.borrow().reset_prop_states();

    send_text(session, "done, log in again to see effect").await
}
