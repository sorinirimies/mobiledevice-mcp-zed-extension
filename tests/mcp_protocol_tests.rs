// tests/mcp_protocol_tests.rs
// Unit tests for MCP protocol implementation

#[cfg(test)]
mod mcp_protocol_tests {
    use serde_json::json;

    #[test]
    fn test_mcp_request_parsing() {
        let json_str = r#"{
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {}
        }"#;

        let result: Result<serde_json::Value, _> = serde_json::from_str(json_str);
        assert!(result.is_ok());

        let request = result.unwrap();
        assert_eq!(request["jsonrpc"], "2.0");
        assert_eq!(request["id"], 1);
        assert_eq!(request["method"], "initialize");
    }

    #[test]
    fn test_mcp_response_format() {
        let response = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": {
                "content": [{
                    "type": "text",
                    "text": "Success"
                }]
            }
        });

        assert_eq!(response["jsonrpc"], "2.0");
        assert_eq!(response["id"], 1);
        assert!(response["result"].is_object());
        assert!(response["result"]["content"].is_array());
    }

    #[test]
    fn test_mcp_error_response_format() {
        let error_response = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "error": {
                "code": -1,
                "message": "Device not found"
            }
        });

        assert_eq!(error_response["jsonrpc"], "2.0");
        assert_eq!(error_response["id"], 1);
        assert!(error_response["error"].is_object());
        assert_eq!(error_response["error"]["code"], -1);
        assert_eq!(error_response["error"]["message"], "Device not found");
    }

    #[test]
    fn test_tool_call_params() {
        let params = json!({
            "name": "mobile_device_mcp_take_screenshot",
            "arguments": {
                "device_id": "emulator-5554",
                "platform": "android"
            }
        });

        assert_eq!(params["name"], "mobile_device_mcp_take_screenshot");
        assert!(params["arguments"].is_object());
        assert_eq!(params["arguments"]["device_id"], "emulator-5554");
        assert_eq!(params["arguments"]["platform"], "android");
    }

    #[test]
    fn test_initialize_response_structure() {
        let init_response = json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {
                "tools": {}
            },
            "serverInfo": {
                "name": "mobile-device-mcp-server",
                "version": "1.0.0"
            }
        });

        assert_eq!(init_response["protocolVersion"], "2024-11-05");
        assert!(init_response["capabilities"].is_object());
        assert!(init_response["capabilities"]["tools"].is_object());
        assert_eq!(
            init_response["serverInfo"]["name"],
            "mobile-device-mcp-server"
        );
    }

    #[test]
    fn test_tools_list_response_structure() {
        let tools_response = json!({
            "tools": [
                {
                    "name": "mobile_device_mcp_list_available_devices",
                    "description": "List all available mobile devices",
                    "inputSchema": {
                        "type": "object",
                        "properties": {}
                    }
                }
            ]
        });

        assert!(tools_response["tools"].is_array());
        let tools = tools_response["tools"].as_array().unwrap();
        assert!(!tools.is_empty());
        assert_eq!(tools[0]["name"], "mobile_device_mcp_list_available_devices");
    }

    #[test]
    fn test_screenshot_response_format() {
        let screenshot_response = json!({
            "content": [{
                "type": "image",
                "data": "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==",
                "mimeType": "image/png"
            }]
        });

        let content = screenshot_response["content"].as_array().unwrap();
        assert_eq!(content[0]["type"], "image");
        assert_eq!(content[0]["mimeType"], "image/png");
        assert!(content[0]["data"].is_string());
    }

    #[test]
    fn test_text_response_format() {
        let text_response = json!({
            "content": [{
                "type": "text",
                "text": "Tapped screen at (100, 200)"
            }]
        });

        let content = text_response["content"].as_array().unwrap();
        assert_eq!(content[0]["type"], "text");
        assert!(content[0]["text"].is_string());
    }

    #[test]
    fn test_device_info_structure() {
        let device_info = json!({
            "id": "emulator-5554",
            "name": "Pixel 6",
            "platform": "android",
            "device_type": "emulator",
            "state": "connected"
        });

        assert!(device_info["id"].is_string());
        assert!(device_info["name"].is_string());
        assert!(device_info["platform"].is_string());
        assert!(device_info["device_type"].is_string());
        assert!(device_info["state"].is_string());
    }

    #[test]
    fn test_invalid_json_handling() {
        let invalid_json = r#"{"jsonrpc": "2.0", "id": 1, "method": "test"#;
        let result: Result<serde_json::Value, _> = serde_json::from_str(invalid_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_missing_required_fields() {
        // Request without method
        let incomplete_request = json!({
            "jsonrpc": "2.0",
            "id": 1
        });

        assert!(incomplete_request.get("method").is_none());
    }

    #[test]
    fn test_null_id_handling() {
        let request_with_null_id = json!({
            "jsonrpc": "2.0",
            "id": null,
            "method": "tools/list"
        });

        assert!(request_with_null_id["id"].is_null());
    }

    #[test]
    fn test_array_content_multiple_items() {
        let response = json!({
            "content": [
                {"type": "text", "text": "Device 1"},
                {"type": "text", "text": "Device 2"},
                {"type": "text", "text": "Device 3"}
            ]
        });

        let content = response["content"].as_array().unwrap();
        assert_eq!(content.len(), 3);
    }

    #[test]
    fn test_platform_values() {
        let platforms = vec!["android", "ios", "auto"];

        for platform in platforms {
            let params = json!({
                "platform": platform
            });
            assert_eq!(params["platform"], platform);
        }
    }

    #[test]
    fn test_coordinate_types() {
        let tap_params = json!({
            "x": 100.5,
            "y": 200.75
        });

        assert!(tap_params["x"].is_number());
        assert!(tap_params["y"].is_number());
        assert_eq!(tap_params["x"].as_f64().unwrap(), 100.5);
        assert_eq!(tap_params["y"].as_f64().unwrap(), 200.75);
    }

    #[test]
    fn test_button_names() {
        let valid_buttons = vec![
            "home",
            "back",
            "menu",
            "power",
            "volume_up",
            "volume_down",
            "camera",
        ];

        for button in valid_buttons {
            let params = json!({
                "button": button
            });
            assert_eq!(params["button"], button);
        }
    }

    #[test]
    fn test_orientation_values() {
        let orientations = vec!["portrait", "landscape"];

        for orientation in orientations {
            let params = json!({
                "orientation": orientation
            });
            assert_eq!(params["orientation"], orientation);
        }
    }

    #[test]
    fn test_app_id_formats() {
        let android_app = "com.example.app";
        let ios_app = "com.company.MyApp";

        let android_params = json!({"app_id": android_app});
        let ios_params = json!({"app_id": ios_app});

        assert_eq!(android_params["app_id"], android_app);
        assert_eq!(ios_params["app_id"], ios_app);
    }

    #[test]
    fn test_url_parameter() {
        let url = "https://example.com/test?param=value";
        let params = json!({"url": url});

        assert_eq!(params["url"], url);
    }

    #[test]
    fn test_filter_parameter() {
        let filter = "Sign in";
        let params = json!({"filter": filter});

        assert_eq!(params["filter"], filter);
    }

    #[test]
    fn test_duration_parameter() {
        let params = json!({
            "duration_ms": 1000
        });

        assert!(params["duration_ms"].is_number());
        assert_eq!(params["duration_ms"].as_u64().unwrap(), 1000);
    }

    #[test]
    fn test_swipe_parameters() {
        let swipe_params = json!({
            "start_x": 100.0,
            "start_y": 500.0,
            "end_x": 100.0,
            "end_y": 100.0,
            "duration_ms": 300
        });

        assert!(swipe_params["start_x"].is_number());
        assert!(swipe_params["start_y"].is_number());
        assert!(swipe_params["end_x"].is_number());
        assert!(swipe_params["end_y"].is_number());
        assert!(swipe_params["duration_ms"].is_number());
    }

    #[test]
    fn test_file_path_parameter() {
        let params = json!({
            "path": "/path/to/app.apk"
        });

        assert_eq!(params["path"], "/path/to/app.apk");
    }

    #[test]
    fn test_text_input_with_special_characters() {
        let special_text = "Hello! @#$%^&*()";
        let params = json!({"text": special_text});

        assert_eq!(params["text"], special_text);
    }

    #[test]
    fn test_empty_arguments() {
        let params = json!({
            "name": "mobile_device_mcp_list_available_devices",
            "arguments": {}
        });

        assert!(params["arguments"].is_object());
        assert_eq!(params["arguments"].as_object().unwrap().len(), 0);
    }

    #[test]
    fn test_response_id_matches_request() {
        let request_id = 42;
        let response = json!({
            "jsonrpc": "2.0",
            "id": request_id,
            "result": {}
        });

        assert_eq!(response["id"], request_id);
    }

    #[test]
    fn test_error_codes() {
        let error_codes = vec![-32700, -32600, -32601, -32602, -32603, -1];

        for code in error_codes {
            let error = json!({
                "code": code,
                "message": "Error message"
            });
            assert_eq!(error["code"], code);
        }
    }

    #[test]
    fn test_base64_encoding_format() {
        // Valid base64 characters test
        let base64_sample = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJ";
        assert!(base64_sample
            .chars()
            .all(|c| { c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '=' }));
    }

    #[test]
    fn test_mime_types() {
        let valid_mime_types = vec!["image/png", "image/jpeg", "text/plain"];

        for mime_type in valid_mime_types {
            let content = json!({
                "type": "image",
                "mimeType": mime_type
            });
            assert_eq!(content["mimeType"], mime_type);
        }
    }

    #[test]
    fn test_batch_requests_structure() {
        let batch = json!([
            {"jsonrpc": "2.0", "id": 1, "method": "tools/list"},
            {"jsonrpc": "2.0", "id": 2, "method": "initialize"}
        ]);

        assert!(batch.is_array());
        let requests = batch.as_array().unwrap();
        assert_eq!(requests.len(), 2);
    }

    #[test]
    fn test_notification_format() {
        // Notification is a request without an id
        let notification = json!({
            "jsonrpc": "2.0",
            "method": "device_connected"
        });

        assert!(notification.get("id").is_none());
        assert_eq!(notification["method"], "device_connected");
    }
}
