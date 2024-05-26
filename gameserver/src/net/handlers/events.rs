use common::data::EXCEL_COLLECTION;

use super::*;

const ALLOWED_EVENTS: [u32; 5] = [
    50014, //match three (origami bird clash)
    150014, //same as above
    50016, //star challenge (stellar shimmer)
    60011, //divergent universe
    10057, //seven day rewards
];

// const ALLOWED_EVENT_TYPES: [i64; 8] = [
//     //older events
//     20, //expeditions
//     2, //7 day checkins (they dont work but they show up) (update: only march one works fully)
//     221, //clock park (something new; clockie event)
//     220, //evolve build panel (legends of galactic baseballer)
//     //even older ones
//     210, //fever time challenge (tides of war)
//     43, //drink maker
//     47, //cosmodysey (wont start without server assistance)
//     37, //dreamchaser bulletin
// ];

// const EVENT_BLACKLIST: [u32; 2] = [
//     10056, //not that newest 7 day check in (only one not working at all)
//     30007, //medicine assignments
// ];

pub async fn on_get_activity_schedule_config_cs_req(
    session: &PlayerSession,
    _body: &GetActivityScheduleConfigCsReq
) -> Result<()> {
    session.send(
        CMD_GET_ACTIVITY_SCHEDULE_CONFIG_SC_RSP, 
        GetActivityScheduleConfigScRsp {
            retcode: 0,
            activity_schedule_list: EXCEL_COLLECTION.activity_panel_configs
                .iter()
                //.filter(| v| ALLOWED_EVENT_TYPES.contains(&v.activity_panel_type) && !EVENT_BLACKLIST.contains(&v.panel_id))
                .filter(| v| ALLOWED_EVENTS.contains(&v.panel_id))
                .map(|v| ActivityScheduleInfo {
                    begin_time: 0,
                    end_time: i64::MAX,
                    dnallopkkfi: v.panel_id,
                    knhkanmiebi: v.activity_module_id
                }
            ).collect()
        }
    ).await
}

// pub async fn on_monopoly_get_region_progress_cs_req(
//     session: &PlayerSession,
//     _body: &MonopolyGetRegionProgressCsReq
// ) -> Result<()> {
//     session.send(
//         CMD_MONOPOLY_GET_REGION_PROGRESS_SC_RSP,
//         MonopolyGetRegionProgressScRsp {
//             baokagnfnab: 1000,
//             nkbehfhlpef: 1000
//         }
//     ).await
// }