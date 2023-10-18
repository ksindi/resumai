# Resumai

Score resumes based on a prompt.

## Deploying

Requires Python. Install dependencies with `pip install -r requirements.txt`.

```bash
cd infrastructure/ & cdk deploy
```

## Running locally

### Run the CLI

```bash
cargo build & ./target/debug/cli --filepath ~/resume.pdf
```

### Starting the frontend

```bash
npm start nodemon server
```

## License

Dual licensed under MIT and Apache 2.0.
