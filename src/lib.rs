#![allow(non_snake_case)]

use minidom::Element;
use std::{collections::HashMap, error::Error};

#[derive(serde::Serialize)]
pub struct Row {
    pub LocalID: String,
    pub OriginTag: String,
    pub Description: String,
    pub DeviceID: String,
    pub Unit: String,
    pub r#Type: String,
}

pub fn parse_dp(path: &str) -> Result<Vec<Row>, Box<dyn Error>> {
    let dp_path = path;
    let namespace: HashMap<&str, &str> = HashMap::from([
        ("sdd", "urn:ISO19848:SHIP_DATA_DEFINITION"),
        ("device", "urn:BLUEONE:DEVICE_DATA_MAP"),
        ("dmd", "urn:BLUEONE:DATA_MODEL_DEFINITION"),
    ]);

    let get_element_err =
        |name: &str| -> String { format!("Failed to get the element of {name}.") };

    // read the dp file from 'path'
    let txt = std::fs::read_to_string(dp_path)?;

    // parse the read string to Element
    let root = txt.parse::<Element>()?;

    // get the element of DataChannelList
    let list = root
        .get_child("DataChannelList", namespace["sdd"])
        .ok_or(get_element_err("DataChannelList"))?;
    let mut data = Vec::with_capacity(list.children().count());

    // extract needed data
    for dataChannel in list.children() {
        // get LocalID
        let localID = dataChannel
            .get_child("DataChannelID", namespace["sdd"])
            .ok_or(get_element_err("DataChannelId"))?
            .get_child("LocalID", namespace["sdd"])
            .ok_or(get_element_err("LocalID"))?
            .text();

        // get Property
        let props = dataChannel
            .get_child("Property", namespace["sdd"])
            .ok_or(get_element_err("Property"))?;

        // get Type
        let r#type = props
            .get_child("Format", namespace["sdd"])
            .ok_or(get_element_err("Format"))?
            .get_child("Type", namespace["sdd"])
            .ok_or(get_element_err("Type"))?
            .text();

        // get Device Property
        let dev_props = props
            .get_child("DeviceProperty", namespace["device"])
            .ok_or(get_element_err("DeviceProperty"))?;

        // get DeviceID
        let deviceID = dev_props
            .get_child("ID", namespace["device"])
            .ok_or(get_element_err("ID"))?
            .text();

        // get OriginTag
        let originTag = dev_props
            .get_child("OriginTag", namespace["device"])
            .ok_or(get_element_err("OriginTag"))?
            .text();

        // get Description
        let description = props
            .get_child("Description", namespace["dmd"])
            .ok_or(get_element_err("Description"))?
            .text();

        // get Unit
        let unit = match props
            .get_child("Unit", namespace["sdd"])
            .ok_or(get_element_err("Unit"))?
            .get_child("QuantityName", namespace["sdd"])
        {
            Some(element) => element.text(),
            None => String::default(),
        };

        data.push(Row {
            r#Type: r#type,
            LocalID: localID,
            OriginTag: originTag,
            Description: description,
            DeviceID: deviceID,
            Unit: unit,
        });
    }

    Ok(data)
}
