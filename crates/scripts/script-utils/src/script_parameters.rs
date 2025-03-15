use std::collections::HashMap;
use std::str::FromStr;

pub struct ScriptParams {
    data: HashMap<String, String>
}

impl ScriptParams {
    pub fn new(params: Option<Vec<String>>) -> Self {
        let data = get_params_map(params);

        ScriptParams {
            data
        }
    }
    
    pub fn get_parameter<T: FromStr>(&self, key: &str) -> Option<T> {
        self.data.get(key).map(|v| v.parse().ok()).flatten()
    }

    pub fn get_list_parameter<T: FromStr>(&self, key: &str) -> Option<Vec<T>> {
        self.data.get(key).map(|v| {
            v.split(",").filter_map(|v| v.parse().ok()).collect()
        })
    }
}

fn get_params_map(params: Option<Vec<String>>) -> HashMap<String, String> {
    let params = params.unwrap_or(vec![]).into_iter().filter_map(|v| {
        let parts = v.split("=").collect::<Vec<_>>();

        if parts.len() != 2 {
            None
        } else {
            Some((parts[0].to_string(), parts[1].to_string()))
        }
    });

    HashMap::from_iter(params)
}