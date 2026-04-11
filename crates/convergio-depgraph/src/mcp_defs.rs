//! MCP tool definitions for the dependency graph extension.

use convergio_types::extension::McpToolDef;
use serde_json::json;

pub fn depgraph_tools() -> Vec<McpToolDef> {
    vec![
        McpToolDef {
            name: "cvg_depgraph".into(),
            description: "Get the dependency graph of all extensions.".into(),
            method: "GET".into(),
            path: "/api/depgraph".into(),
            input_schema: json!({"type": "object", "properties": {}}),
            min_ring: "community".into(),
            path_params: vec![],
        },
        McpToolDef {
            name: "cvg_depgraph_validate".into(),
            description: "Validate the dependency graph (detect cycles).".into(),
            method: "GET".into(),
            path: "/api/depgraph/validate".into(),
            input_schema: json!({"type": "object", "properties": {}}),
            min_ring: "community".into(),
            path_params: vec![],
        },
        McpToolDef {
            name: "cvg_capabilities".into(),
            description: "List all extension capabilities.".into(),
            method: "GET".into(),
            path: "/api/capabilities".into(),
            input_schema: json!({"type": "object", "properties": {}}),
            min_ring: "sandboxed".into(),
            path_params: vec![],
        },
        McpToolDef {
            name: "cvg_openapi".into(),
            description: "Get OpenAPI specification.".into(),
            method: "GET".into(),
            path: "/api/openapi".into(),
            input_schema: json!({"type": "object", "properties": {}}),
            min_ring: "sandboxed".into(),
            path_params: vec![],
        },
        McpToolDef {
            name: "cvg_removal_check".into(),
            description: "Check impact of removing an extension module.".into(),
            method: "GET".into(),
            path: "/api/depgraph/removal-check/:module_id".into(),
            input_schema: json!({
                "type": "object",
                "properties": {"module_id": {"type": "string"}},
                "required": ["module_id"]
            }),
            min_ring: "trusted".into(),
            path_params: vec!["module_id".into()],
        },
    ]
}
