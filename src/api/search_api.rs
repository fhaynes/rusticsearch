use std::io::Read;

use iron::prelude::*;
use iron::status;
use router::Router;
use rustc_serialize::json::Json;

use super::persistent;
use super::super::{Globals, query};


pub fn view_count(req: &mut Request) -> IronResult<Response> {
    let ref glob = get_globals!(req);
    let ref index_name = read_path_parameter!(req, "index").unwrap_or("");

    // Lock index array
    let indices = glob.indices.read().unwrap();

    // Get index
    let index = get_index_or_404!(indices, *index_name);

    let count = match json_from_request_body!(req) {
        Some(query_json) => {
            // Parse query
            let query = query::parse_query(query_json.as_object().unwrap().get("query").unwrap());
            debug!("{:#?}", query);

            match query {
                Ok(query) => {
                    let mut count = 0;
                    for (_, doc) in index.docs.iter() {
                        if query.matches(&doc) {
                            count += 1;
                        }
                    }

                    count
                }
                Err(error) => {
                    // TODO: What specifically is bad about the Query?
                    let mut response = Response::with((status::BadRequest,
                                                       "{\"message\": \"Query error\"}"));
                    response.headers.set_raw("Content-Type", vec![b"application/json".to_vec()]);
                    return Ok(response);
                }
            }
        }
        None => index.docs.len()
    };

    return json_response!(status::Ok, format!("{{\"count\": {}}}", count));
}


pub fn view_search(req: &mut Request) -> IronResult<Response> {
    let ref glob = get_globals!(req);
    let ref index_name = read_path_parameter!(req, "index").unwrap_or("");

    // Lock index array
    let indices = glob.indices.read().unwrap();

    // Get index
    let index = get_index_or_404!(indices, *index_name);

    let data = json_from_request_body!(req);
    debug!("{:#?}", query::parse_query(data.unwrap().as_object().unwrap().get("query").unwrap()));

    // TODO: Run query

    // TODO: {"took":5,"timed_out":false,"_shards":{"total":5,"successful":5,"failed":0},"hits":{"total":4,"max_score":1.0,"hits":[{"_index":"wagtail","_type":"searchtests_searchtest_searchtests_searchtestchild","_id":"searchtests_searchtest:5380","_score":1.0,"fields":{"pk":["5380"]}},{"_index":"wagtail","_type":"searchtests_searchtest","_id":"searchtests_searchtest:5379","_score":1.0,"fields":{"pk":["5379"]}}]}}
    return json_response!(status::Ok, "{\"hits\": {\"total\": 0, \"hits\": []}}");
}
