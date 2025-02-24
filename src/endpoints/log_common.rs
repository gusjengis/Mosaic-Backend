use crate::log::Log;

pub fn construct_log_body(logs: Vec<Log>) -> String {
    let mut res = "".to_string();
    for log in logs {
        res.push_str(format!("{}\n", log).as_str());
    }

    return res.trim_end().to_string();
}
