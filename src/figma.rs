use reqwest::Client;
use serde_json::Value;

use crate::config::FigmaConfig;

pub struct FigmaImageExtractor;

impl FigmaImageExtractor {
  const API_URL: &'static str = "https://api.figma.com/v1";

  pub async fn fetch_figma_images() -> Result<Option<Vec<(String, Value, String)>>, reqwest::Error>
  {
    let FigmaConfig {
      figma_access_token,
      figma_file_key,
      ..
    } = FigmaConfig::new();

    let client = Client::new();
    let file_url = format!("{}/images/{}", Self::API_URL, figma_file_key);
    let image_nodes = Self::get_image_nodes().await;

    let response = client
      .get(&file_url)
      .query(&[
        ("format", "png"),
        (
          "ids",
          &image_nodes
            .iter()
            .map(|(id, _)| id.as_str())
            .collect::<Vec<_>>()
            .join(","),
        ),
      ])
      .header("X-Figma-Token", figma_access_token)
      .send()
      .await?
      .text()
      .await?;

    let response: Value = serde_json::from_str(&response).unwrap();
    let images = response["images"].as_object();

    Ok(images.map(|imgs| {
      imgs
        .iter()
        .filter_map(|(id, url)| {
          let name = image_nodes
            .iter()
            .find(|(node_id, _)| node_id == id)
            .map(|(_, name)| name.clone())?;
          Some((id.clone(), url.clone(), name))
        })
        .collect()
    }))
  }

  async fn get_image_nodes() -> Vec<(String, String)> {
    let FigmaConfig {
      figma_access_token,
      figma_file_key,
      ..
    } = FigmaConfig::new();

    let client = Client::new();
    let file_url = format!("{}/files/{}", Self::API_URL, figma_file_key);

    let response = client
      .get(&file_url)
      .header("X-Figma-Token", figma_access_token)
      .send()
      .await
      .unwrap()
      .text()
      .await
      .unwrap();

    let response: Value = serde_json::from_str(&response).unwrap();
    let document = response.as_object().unwrap().get("document").unwrap();

    let mut image_nodes = Vec::new();
    let mut stack = vec![document.clone()];

    while let Some(node) = stack.pop() {
      if Self::is_image_node(&node) {
        if let Some((id, name)) = node
          .as_object()
          .and_then(|obj| Some((obj.get("id")?, obj.get("name")?)))
          .and_then(|(id, name)| Some((id.as_str()?, name.as_str()?)))
        {
          image_nodes.push((id.to_string(), name.to_string()));
        }
      }

      if let Some(children) = node
        .as_object()
        .and_then(|obj| obj.get("children"))
        .and_then(|c| c.as_array())
      {
        stack.extend(children.iter().cloned());
      }
    }

    image_nodes
  }

  fn is_image_node(node: &Value) -> bool {
    if let Some(obj) = node.as_object() {
      if obj.get("type").and_then(|t| t.as_str()) == Some("IMAGE") {
        return true;
      }

      if let Some(fills) = obj.get("fills").and_then(|f| f.as_array()) {
        return fills.iter().any(|fill| {
          fill
            .as_object()
            .and_then(|f| f.get("type"))
            .and_then(|t| t.as_str())
            == Some("IMAGE")
        });
      }
    }
    false
  }
}

#[cfg(test)]
mod tests {
  use serde_json::json;

  use super::*;

  #[tokio::test]
  async fn test_is_image_node() {
    let image_node = json!({
        "type": "IMAGE",
        "id": "1:1"
    });
    assert!(FigmaImageExtractor::is_image_node(&image_node));

    let fill_image_node = json!({
        "type": "RECTANGLE",
        "id": "1:2",
        "fills": [{
            "type": "IMAGE",
            "scaleMode": "FILL"
        }]
    });
    assert!(FigmaImageExtractor::is_image_node(&fill_image_node));

    let non_image_node = json!({
        "type": "RECTANGLE",
        "id": "1:3",
        "fills": [{
            "type": "SOLID",
            "color": {"r": 1, "g": 1, "b": 1}
        }]
    });
    assert!(!FigmaImageExtractor::is_image_node(&non_image_node));
  }
}
