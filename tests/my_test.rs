#![allow(non_snake_case)]

use csv::Writer;
use minidom::Element;
use std::error::Error;

#[derive(serde::Serialize)]
struct Row {
    localID: String,
    originTag: String,
    description: String,
    unit: String,
}

#[test]
fn my_first_test() -> Result<(), Box<dyn Error>> {
    let dp_path = std::path::Path::new("res/DP_IMO9986104_20250530230013.xml");
    let namespace = "urn:ISO19848:SHIP_DATA_DEFINITION";
    let fname = dp_path.file_stem().unwrap().to_str().unwrap();
    let mut writer = Writer::from_path(format!("{fname}.csv"))?;

    // read dp file as String
    let mut start = std::time::Instant::now();
    let txt = std::fs::read_to_string(dp_path)?;
    println!(
        "read the dp content to string. {:.2} sec",
        start.elapsed().as_secs_f32()
    );

    // parse the string to Element
    start = std::time::Instant::now();
    let root = txt.parse::<Element>()?;
    println!(
        "parse the String to Element. {:.2} sec",
        start.elapsed().as_secs_f32()
    );

    // get the element of DataChannelList
    let list = root
        .get_child("DataChannelList", namespace)
        .ok_or("failed to get the element of DataChannelList")?;

    for dataChannel in list.children() {
        // get LocalID
        let localID = dataChannel
            .get_child("DataChannelID", namespace)
            .ok_or("Failed to get the element of DataChannelId")?
            .get_child("LocalID", namespace)
            .ok_or("Failed to get the element of LocalID")?
            .text();

        // get Property
        let props = dataChannel
            .get_child("Property", namespace)
            .ok_or("Failed to get the element of Property.")?;

        // get OriginalTag
        let originTag = props
            .get_child("DeviceProperty", "urn:BLUEONE:DEVICE_DATA_MAP")
            .ok_or("Failed to get the element of DeviceProperty")?
            .get_child("OriginTag", "urn:BLUEONE:DEVICE_DATA_MAP")
            .ok_or("err")?
            .text();

        // get Description
        let description = props
            .get_child("Description", "urn:BLUEONE:DATA_MODEL_DEFINITION")
            .ok_or("err")?
            .text();

        // get Unit
        let unit = match props
            .get_child("Unit", namespace)
            .ok_or("Failed to get the element of Unit")?
            .get_child("QuantityName", namespace)
        {
            Some(element) => element.text(),
            None => String::default(),
        };

        // for tmp in props
        //     .get_child("Unit", namespace)
        //     .ok_or("Failed to get the element of Unit")?.children() {
        //         println!();
        //     }

        writer.serialize(Row {
            localID,
            originTag,
            description,
            unit,
        })?;
    }

    Ok(())
}

const DATA: &str = r#"<articles xmlns="article">
    <article>
        <title>10 Terrible Bugs You Would NEVER Believe Happened</title>
        <body>
            Rust fixed them all. &lt;3
        </body>
    </article>
    <article>
        <title>BREAKING NEWS: Physical Bug Jumps Out Of Programmer's Screen</title>
        <body>
            Just kidding!
        </body>
    </article>
</articles>"#;

const ARTICLE_NS: &str = "article";

#[derive(Debug)]
pub struct Article {
    title: String,
    body: String,
}

#[test]
fn minidom_test() -> Result<(), Box<dyn Error>> {
    let root: Element = DATA.parse().unwrap();

    let mut articles: Vec<Article> = Vec::new();

    for child in root.children() {
        if child.is("article", ARTICLE_NS) {
            let title = child.get_child("title", ARTICLE_NS).unwrap().text();
            let body = child.get_child("body", ARTICLE_NS).unwrap().text();
            articles.push(Article {
                title,
                body: body.trim().to_owned(),
            });
        }
    }

    println!("{:?}", articles);

    Ok(())
}
