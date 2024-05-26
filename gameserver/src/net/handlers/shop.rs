use common::data::EXCEL_COLLECTION;

use super::*;

pub async fn on_get_shop_list_cs_req(
    session: &PlayerSession,
    body: &GetShopListCsReq
) -> Result<()> {
    let rsp = GetShopListScRsp {
        retcode: 0,
        shop_type: body.shop_type,
        shop_list: EXCEL_COLLECTION.shop_configs.iter().filter(|a| a.shop_type == body.shop_type).map(|shop|
            Shop {
                shop_id: shop.shop_id,
                begin_time: 0,
                end_time: i64::MAX,
                city_exp: 0,
                city_level: 1,
                city_taken_level_reward: 0,
                goods_list: EXCEL_COLLECTION.shop_goods_configs.iter().filter(|g| g.shop_id == shop.shop_id).map(|goods| 
                    Goods {
                        begin_time: 0,
                        end_time: i64::MAX,
                        buy_times: 0,
                        goods_id: goods.goods_id,
                        item_id: goods.item_id,
                    }
                ).collect()
            }
        ).collect()
    };

    session.send(
        CMD_GET_SHOP_LIST_SC_RSP,
         rsp
    ).await
}