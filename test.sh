#!/bin/bash

# Endpoint URL
UPLOAD_ENDPOINT="https://3yyyw6og8d.execute-api.us-east-1.amazonaws.com/prod/upload"
EVALUATION_ENDPOINT="https://3yyyw6og8d.execute-api.us-east-1.amazonaws.com/prod/evaluations"

# Ensure file path is given as argument
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <path_to_pdf_file>"
    exit 1
fi

FILE_PATH="$1"
echo "Attempting to upload file: $FILE_PATH"

# Fetch upload_url
echo "Fetching upload URL and evaluation ID..."
RESPONSE=$(curl -s -X POST "$UPLOAD_ENDPOINT")
UPLOAD_URL=$(echo $RESPONSE | jq -r '.upload_url')
EVALUATION_ID=$(echo $RESPONSE | jq -r '.evaluation_id')

if [ -z "$UPLOAD_URL" ] || [ -z "$EVALUATION_ID" ]; then
    echo "Failed to retrieve upload URL or evaluation ID."
    exit 1
fi

echo "Received upload URL: $UPLOAD_URL"
echo "Received evaluation ID: $EVALUATION_ID"

# Upload the file
echo "Uploading the file to the provided URL..."
curl -X PUT -T "$FILE_PATH" -H 'Content-Type: application/pdf' "$UPLOAD_URL"
echo "Upload complete."

# Polling for the evaluation
echo "Polling for the evaluation. This may take up to 5 minutes..."
for i in {1..60}; do
    EVALUATION_RESPONSE=$(curl -s -o /dev/null -w '%{http_code}' "$EVALUATION_ENDPOINT/$EVALUATION_ID")
    if [ "$EVALUATION_RESPONSE" == "200" ]; then
        EVALUATION_CONTENT=$(curl -s "$EVALUATION_ENDPOINT/$EVALUATION_ID" | jq -r '.evaluation')
        echo "Evaluation received: $EVALUATION_CONTENT"
        exit 0
    elif [ "$EVALUATION_RESPONSE" != "404" ]; then
        echo "Unexpected error during polling. HTTP Status: $EVALUATION_RESPONSE"
        exit 1
    else
        echo "Evaluation not ready yet. Retrying in 5 seconds... (Attempt $i)"
    fi
    sleep 5
done

echo "Evaluation was not ready after 5 minutes. Exiting..."
exit 1
