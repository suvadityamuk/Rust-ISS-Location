use serde_json::{Value};
use staticmap::{tools::{Color, CircleBuilder}, StaticMapBuilder};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>>{

    // Formatting and setting URL for Request
    
    let url = "https://api.wheretheiss.at/v1/satellites/25544";

    // Making of actual request - Waiting for response

    let resp = reqwest::get(url)
        .await?
        .text()
        .await?;

    let v : Value  = serde_json::from_str(&resp)?;
    println!("Location : {}, {}", v["latitude"], v["longitude"]);

    let mut map = StaticMapBuilder::default()
        .width(1080)
        .height(1080)
        .zoom(4)
        .padding((10,0))
        .build()?;
    
    let lat_str = v["latitude"].to_string();
    let lon_str = v["longitude"].to_string();

    let lat = &lat_str.parse::<f64>()?;
    let lon = &lon_str.parse::<f64>()?;

    let outer_color = Color::new(true, 255, 0, 0, 255);
    let mid_color = Color::new(true, 255, 140, 0, 255);
    let inner_color = Color::new(true, 0, 255, 0, 255);

    let marker_point = CircleBuilder::default()
        .lat_coordinate(*lat)
        .lon_coordinate(*lon)
        .radius(2.0)
        .color(inner_color)
        .build()?;
    let marker_point2 = CircleBuilder::default()
        .lat_coordinate(*lat)
        .lon_coordinate(*lon)
        .radius(10.0)
        .color(mid_color)
        .build()?;
    let marker_point3 = CircleBuilder::default()
        .lat_coordinate(*lat)
        .lon_coordinate(*lon)
        .radius(18.0)
        .color(outer_color)
        .build()?;


    map.add_tool(marker_point3);
    map.add_tool(marker_point2);
    map.add_tool(marker_point);
    map.save_png("iss_map.png")?;

    println!("Please check the root folder for \"iss_map.png\" to see the result!");
    Ok(())
}