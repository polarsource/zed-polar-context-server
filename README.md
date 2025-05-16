# Zed Polar Context Server

This extension provides a Model Context Server for Polar, for use with the Zed AI assistant.

## Configuration

To use the extension, you will need to set a valid Polar access token in your Zed `settings.json`:

```json
{
  "context_servers": {
    "polar-context-server": {
      "settings": {
        "access_token": "polar_oat_XXX"
      }
    }
  }
}
```

## Usage

Once done, enable the Polar MCP tools you need in your agent profile.

Read more: https://zed.dev/docs/ai/agent-panel#profiles
