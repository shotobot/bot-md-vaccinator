pub enum Endpoint {
    CalenderList,
    CalerderDetails(u64)
}

pub fn get(endpoint: Endpoint) -> Result<serde_json::Value, String> {
    let url = match endpoint {
        Endpoint::CalenderList => "https://impfzentrum.termin-direkt.de/rest-v2/api/Calendars/".to_string(),
        Endpoint::CalerderDetails(id) => "https://impfzentrum.termin-direkt.de/rest-v2/api/Calendars/$(id)/FirstFreeSlot".replace("$(id)", &id.to_string())
    };
    
    match request(&url) {
        Ok(body) => into_json(&body),
        Err(err) => Err(err)
    }   
}

fn into_json(body: &str) -> Result<serde_json::Value, String> {
    match serde_json::from_str(&body) {
        Ok(value) => Ok(value),
        Err(err) => Err(format!("unable to parse json: '{}'", err))
    }
}

fn request(url: &str) -> Result<String, String> {
    log::info!("request '{}'", url);
    match reqwest::blocking::get(url) {
        Ok(res) => get_body(res),
        Err(err) => Err(format!("unable to make GET request: '{}'", err))
    }
}

fn get_body(res: reqwest::blocking::Response) -> Result<String, String> {
    let text = res.text();
    match text {
        Ok(body) => {
            log::info!("HTTP GET body: {}", body);
            Ok(body.to_string())
        },
        Err(err) => Err(format!("unable to extract body of response: '{}'", err))
    }
}