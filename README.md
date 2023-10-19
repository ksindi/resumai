# Resumai

Score resumes based on a prompt.

## Deploying

Requires Python. Install dependencies with `pip install -r requirements.txt`.

```bash
cd infrastructure/ && cdk deploy
```

## Running locally

### Run the CLI

Make sure you have `OPENAPI_API_KEY` set in your environment.

```bash
cargo build & ./target/debug/cli --filepath ~/example-resume.pdf
```

### Starting the frontend

```bash
make frontend
```

## License

Dual licensed under MIT and Apache 2.0.
