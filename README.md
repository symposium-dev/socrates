# Socrates

A Zulip bot that responds with famous Socrates quotes when mentioned. Built in Rust for serverless deployment.

## Features

- 🤖 Responds to `@socrates` mentions in Zulip messages
- 📚 Shares famous Socrates quotes randomly selected from a curated collection
- 👋 Personalized greetings when users say "hi"
- ⚡ Serverless architecture for efficient scaling
- 🦀 Written in Rust for performance and reliability

## How it works

The bot listens for Zulip webhook events and responds when:
- A message contains `@socrates` or `@**Socrates**` (case insensitive)
- It will respond with a random Socrates quote
- If the message contains "hi", it will include a personalized greeting

## Example interactions

**User:** `@socrates hi!`
**Socrates:** `Hello, John! All I know is that I know nothing.`

**User:** `@socrates what is wisdom?`
**Socrates:** `Wonder is the beginning of wisdom.`

## Quotes included

The bot includes 12 famous Socrates quotes:
- "All I know is that I know nothing."
- "The only true wisdom is in knowing you know nothing."
- "An unexamined life is not worth living."
- "There is only one good, knowledge, and one evil, ignorance."
- "I cannot teach anybody anything. I can only make them think."
- And more...

## Development

### Prerequisites

- Rust 1.89+ 
- Cargo

### Building

```bash
cargo build --release
```

### Testing

```bash
cargo test
```

### Running locally

```bash
cargo run
```

## Deployment

### AWS Lambda (Recommended)

1. Install AWS SAM CLI
2. Install cargo-lambda:
   ```bash
   cargo install cargo-lambda
   ```
3. Deploy:
   ```bash
   sam build
   sam deploy --guided
   ```

### Using Docker

```bash
docker build -t socrates-bot .
docker run -p 8080:8080 socrates-bot
```

## Zulip Configuration

1. Go to your Zulip organization settings
2. Navigate to "Integrations" -> "Incoming webhooks"
3. Create a new webhook pointing to your deployed function URL
4. Add `/webhook` to the end of your function URL
5. Configure the webhook to trigger on "Message sent"

## Environment Variables

- `RUST_LOG`: Set logging level (default: `info`)

## License

MIT License - see LICENSE file for details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request
