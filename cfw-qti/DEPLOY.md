# Deployment Guide

## Prerequisites

1. Install Wrangler CLI:
```bash
npm install -g wrangler
```

2. Login to Cloudflare:
```bash
wrangler login
```

## Local Development

1. Build the worker:
```bash
worker-build --release
```

2. Run locally:
```bash
wrangler dev
```

3. Open your browser to `http://localhost:8787`

## Deployment to Cloudflare

1. Update `wrangler.toml` with your account details (if needed):
```toml
name = "cfw-qti"
account_id = "your-account-id"  # Optional
workers_dev = true  # or set to false if using a custom domain
```

2. Deploy:
```bash
wrangler deploy
```

3. Your worker will be available at:
   - `https://cfw-qti.your-subdomain.workers.dev`

## Testing

### Test Health Endpoint
```bash
curl http://localhost:8787/health
```

Expected response:
```json
{
  "status": "healthy",
  "service": "cfw-qti"
}
```

### Test Quiz Generation

Save this to a file called `test_quiz.json`:
```json
{
  "content": "title: Test Quiz\n\n1. What is 2+2?\n*a) 4\nb) 5\nc) 3",
  "canvas": false,
  "skip_validation": false
}
```

Then test:
```bash
curl -X POST http://localhost:8787/generate \
  -H "Content-Type: application/json" \
  -d @test_quiz.json \
  --output test_quiz.zip
```

Verify the ZIP was created:
```bash
unzip -l test_quiz.zip
```

## Configuration

### AI Binding

The worker uses Cloudflare Workers AI for quiz generation. The AI binding is configured in `wrangler.toml`:

```toml
[ai]
binding = "AI"
```

This is included by default. No additional setup required - Workers AI is available on all paid plans.

### Environment Variables

You can add environment variables in `wrangler.toml`:

```toml
[env.production]
vars = { ENVIRONMENT = "production" }

[env.staging]
vars = { ENVIRONMENT = "staging" }
```

### Custom Domain

To use a custom domain, add a route in `wrangler.toml`:

```toml
routes = [
  { pattern = "qti.example.com/*", zone_name = "example.com" }
]
```

## Monitoring

View logs in real-time:
```bash
wrangler tail
```

## Troubleshooting

### Build fails
- Ensure you're in the `cfw-qti` directory
- Try: `cargo clean && worker-build --release`

### Worker doesn't start
- Check that port 8787 isn't already in use
- Try: `wrangler dev --port 8788`

### ZIP generation fails
- Check the quiz format is correct
- Look at browser console for error messages
- Check worker logs with `wrangler tail`

## Performance

The worker is optimized for Cloudflare's limits:
- WASM binary is size-optimized (`opt-level = "z"`)
- LTO is enabled for better code generation
- All dependencies are minimal and WASM-compatible

Typical response times:
- Health check: < 10ms
- Small quiz (5 questions): 50-100ms
- Large quiz (50 questions): 200-500ms
