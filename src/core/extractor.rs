use reqwest::Client;
use serde_json::Value;

use crate::config::FigmaConfig;

pub struct FigmaImageExtractor {
  client: Client,
  config: FigmaConfig,
  api_url: String,
}

impl FigmaImageExtractor {
  pub fn new(client: Client, config: FigmaConfig) -> Self {
    Self {
      client,
      config,
      api_url: "https://api.figma.com/v1".to_string(),
    }
  }

  #[cfg(test)]
  fn with_api_url(client: Client, config: FigmaConfig, api_url: String) -> Self {
    Self {
      client,
      config,
      api_url,
    }
  }

  fn build_url(&self, endpoint: &str) -> String {
    format!(
      "{}/{}/{}",
      self.api_url, endpoint, self.config.figma_file_key
    )
  }

  pub async fn fetch_figma_images(&self) -> Result<Vec<(String, Value, String)>, reqwest::Error> {
    let file_url = self.build_url("images");
    let image_nodes = self.get_image_nodes().await?;
    let ids = image_nodes
      .iter()
      .map(|(id, _)| id.as_str())
      .collect::<Vec<_>>()
      .join(",");

    let response = self
      .client
      .get(&file_url)
      .query(&[("format", "png"), ("ids", &ids)])
      .header("X-Figma-Token", &self.config.figma_access_token)
      .send()
      .await?
      .json::<Value>()
      .await?;

    let images = response["images"]
      .as_object()
      .map(|imgs| {
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
      })
      .unwrap_or_default();

    Ok(images)
  }

  async fn get_image_nodes(&self) -> Result<Vec<(String, String)>, reqwest::Error> {
    // Generate files URL using build_url
    let file_url = self.build_url("files");
    let response = self
      .client
      .get(&file_url)
      .header("X-Figma-Token", &self.config.figma_access_token)
      .send()
      .await?
      .json::<Value>()
      .await?;

    let document = response
      .as_object()
      .and_then(|obj| obj.get("document"))
      .unwrap_or(&Value::Null);

    let image_nodes = Self::extract_image_nodes(document);
    Ok(image_nodes)
  }

  fn extract_image_nodes(document: &Value) -> Vec<(String, String)> {
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
  use httpmock::prelude::*;
  use serde_json::json;

  use super::*;

  #[tokio::test]
  async fn test_fetch_figma_images() {
    let server = MockServer::start();

    let file_mock = server.mock(|when, then| {
      when
        .method(GET)
        .path("/files/test-key")
        .header("X-Figma-Token", "test-token");
      then
        .status(200)
        .header("content-type", "application/json")
        .json_body(json!({
            "document": {
                "id": "0:0",
                "children": [{
                    "id": "1:1",
                    "type": "IMAGE",
                    "name": "test_image"
                }]
            }
        }));
    });

    let images_mock = server.mock(|when, then| {
      when
        .method(GET)
        .path("/images/test-key")
        .header("X-Figma-Token", "test-token")
        .query_param("ids", "1:1");
      then
        .status(200)
        .header("content-type", "application/json")
        .json_body(json!({
            "images": {
                "1:1": "https://example.com/test_image.png"
            }
        }));
    });

    let config = FigmaConfig {
      figma_access_token: "test-token".to_string(),
      figma_file_key: "test-key".to_string(),
    };

    let extractor = FigmaImageExtractor::with_api_url(Client::new(), config, server.base_url());

    let images = extractor.fetch_figma_images().await.unwrap();

    file_mock.assert();
    images_mock.assert();

    assert_eq!(images.len(), 1);
    assert_eq!(images[0].0, "1:1");
    assert_eq!(
      images[0].1.as_str().unwrap(),
      "https://example.com/test_image.png"
    );
    assert_eq!(images[0].2, "test_image");
  }

  #[test]
  fn test_is_image_node() {
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

  #[test]
  fn test_new() {
    let config = FigmaConfig {
      figma_access_token: "test-token".to_string(),
      figma_file_key: "test-key".to_string(),
    };
    let extractor = FigmaImageExtractor::new(Client::new(), config);

    assert_eq!(extractor.api_url, "https://api.figma.com/v1");
    assert_eq!(extractor.config.figma_access_token, "test-token");
    assert_eq!(extractor.config.figma_file_key, "test-key");
  }

  #[test]
  fn test_build_url() {
    let config = FigmaConfig {
      figma_access_token: "test-token".to_string(),
      figma_file_key: "test-key".to_string(),
    };
    let extractor = FigmaImageExtractor::new(Client::new(), config);

    assert_eq!(
      extractor.build_url("files"),
      "https://api.figma.com/v1/files/test-key"
    );
    assert_eq!(
      extractor.build_url("images"),
      "https://api.figma.com/v1/images/test-key"
    );
  }

  #[test]
  fn test_extract_image_nodes() {
    let document = json!({
        "id": "0:0",
        "children": [
            {
                "id": "1:1",
                "type": "IMAGE",
                "name": "test_image1"
            },
            {
                "id": "1:2",
                "type": "FRAME",
                "name": "frame",
                "children": [
                    {
                        "id": "1:3",
                        "type": "IMAGE",
                        "name": "test_image2"
                    }
                ]
            }
        ]
    });

    let nodes = FigmaImageExtractor::extract_image_nodes(&document);
    assert_eq!(nodes.len(), 2);
    assert_eq!(nodes[0], ("1:3".to_string(), "test_image2".to_string()));
    assert_eq!(nodes[1], ("1:1".to_string(), "test_image1".to_string()));
  }

  #[tokio::test]
  async fn test_get_image_nodes() {
    let server = MockServer::start();
    server.mock(|when, then| {
      when
        .method(GET)
        .path("/files/test-key")
        .header("X-Figma-Token", "test-token");
      then
        .status(200)
        .header("content-type", "application/json")
        .json_body(json!({
            "document": {
                "id": "0:0",
                "children": [
                    {
                        "id": "1:1",
                        "type": "IMAGE",
                        "name": "test_image"
                    }
                ]
            }
        }));
    });

    let config = FigmaConfig {
      figma_access_token: "test-token".to_string(),
      figma_file_key: "test-key".to_string(),
    };
    let extractor = FigmaImageExtractor::with_api_url(Client::new(), config, server.base_url());

    let nodes = extractor.get_image_nodes().await.unwrap();
    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes[0], ("1:1".to_string(), "test_image".to_string()));
  }
}
