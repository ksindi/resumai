# ResumAI

Upload your resume and receive professional feedback within minutes. Score resumes based on a prompt using GPT.

## Requirements

1. [AWS CDK](https://docs.aws.amazon.com/cdk/latest/guide/getting_started.html)
2. [Rust](https://www.rust-lang.org/tools/install)
3. [Node.js](https://nodejs.org/en/download/)
4. [Python](https://www.python.org/downloads/)

## Deploying

Install dependencies with `pip install -r infrastructure/requirements.txt`.

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
