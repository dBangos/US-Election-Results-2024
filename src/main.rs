use std::{fs::File, io::Write};

struct ResultLine {
    state: String,
    county: String,
    trump_percentage: String,
    harris_percentage: String,
}

impl Default for ResultLine {
    fn default() -> Self {
        Self {
            state: "none".to_string(),
            county: "none".to_string(),
            trump_percentage: "none".to_string(),
            harris_percentage: "none".to_string(),
        }
    }
}
#[tokio::main]
async fn main() {
    get_data().await;
    println!("Done!");
}
async fn get_data() {
    let mut output_sting = String::new();
    let state_names = [
        "Alabama",
        "Alaska",
        "Arizona",
        "Arkansas",
        "California",
        "Colorado",
        "Connecticut",
        "Delaware",
        "District-of-columbia",
        "Florida",
        "Georgia",
        "Hawaii",
        "Idaho",
        "Illinois",
        "Indiana",
        "Iowa",
        "Kansas",
        "Kentucky",
        "Louisiana",
        "Maine",
        "Maryland",
        "Massachusetts",
        "Michigan",
        "Minnesota",
        "Mississippi",
        "Missouri",
        "Montana",
        "Nebraska",
        "Nevada",
        "New-Hampshire",
        "New-Jersey",
        "New-Mexico",
        "New-York",
        "North-Carolina",
        "North-Dakota",
        "Ohio",
        "Oklahoma",
        "Oregon",
        "Pennsylvania",
        "Rhode-Island",
        "South-Carolina",
        "South-Dakota",
        "Tennessee",
        "Texas",
        "Utah",
        "Vermont",
        "Virginia",
        "Washington",
        "West-Virginia",
        "Wisconsin",
        "Wyoming",
    ];
    output_sting += &("State,County,Trump Percent, Harris Percent\n");
    for (index, state) in state_names.into_iter().enumerate() {
        println!("{}/{}", index + 1, state_names.len());
        let url = format!(
            "https://www.nbcnews.com/politics/2024-elections/{}-president-results",
            state.to_lowercase()
        );
        let client = reqwest::Client::new();
        let mut headers = reqwest::header::HeaderMap::new();

        headers.insert("authorization", "<authorization>".parse().unwrap());
        headers.insert("user-agent", "CUSTOM_NAME/1.0".parse().unwrap());
        let response = client.get(url).headers(headers).send().await.unwrap();
        let html_content = response.text().await.unwrap();
        let html_split = html_content.split(r#"[{"areas""#).collect::<Vec<&str>>();
        let html_split = html_split[1]
            .split(r#"electionDate"#)
            .collect::<Vec<&str>>();
        let html_split = html_split[0].split(',').filter(|x| {
            !(x.contains("color")
                || x.contains("Color")
                || x.contains("headshot")
                || x.contains("Winner")
                || x.contains("lead")
                || x.contains("Incumbent")
                || x.contains("code")
                || x.contains("call")
                || x.contains("first")
                || x.contains("last")
                || x.contains("party")
                || x.contains("format")
                || x.contains("electoral"))
        });
        let mut trump_next = false;
        let mut harris_next = false;
        let mut current_line: ResultLine = ResultLine::default();
        for item in html_split {
            if trump_next && item.contains("percent") {
                trump_next = false;
                current_line.trump_percentage =
                    item.split(':').collect::<Vec<&str>>()[1].to_string();
            } else if harris_next && item.contains("percent") {
                harris_next = false;
                current_line.harris_percentage =
                    item.split(':').collect::<Vec<&str>>()[1].to_string();
            } else if item.contains("{") {
                if current_line.state != "none"
                    && current_line.county != "none"
                    && current_line.trump_percentage != "none"
                    && current_line.harris_percentage != "none"
                {
                    output_sting += &(current_line.state
                        + ","
                        + &current_line.county
                        + ","
                        + &current_line.trump_percentage
                        + ","
                        + &current_line.harris_percentage
                        + "\n");
                }
                current_line = ResultLine::default();
                current_line.county = item.split('"').collect::<Vec<&str>>()[3].to_string();
                current_line.state = state.to_string();
            } else if item.contains("Trump") {
                trump_next = true;
            } else if item.contains("Harris") {
                harris_next = true;
            }
        }
        //adding the last county
        if current_line.state != "none"
            && current_line.county != "none"
            && current_line.trump_percentage != "none"
            && current_line.harris_percentage != "none"
        {
            output_sting += &(current_line.state
                + ","
                + &current_line.county
                + ","
                + &current_line.trump_percentage
                + ","
                + &current_line.harris_percentage
                + "\n");
        }
    }
    let mut out_file = File::create("data.csv").expect("Failed to create file");
    let _ = out_file.write(output_sting.as_bytes());
}
