import json
import boto3
import os

# Initialize the SQS client
sqs = boto3.client("sqs")

# Get the SQS queue URL from the environment variables (set by CDK)
QUEUE_URL = os.environ.get("QUEUE_URL")


def handler(event, context):
    try:
        print(event)

        # Extract the body from the API Gateway event
        body = event.get("body")

        # Send the body to the SQS queue
        response = sqs.send_message(QueueUrl=QUEUE_URL, MessageBody=body)

        # Return a successful response
        return {
            "statusCode": 200,
            "body": json.dumps(
                {
                    "message": "Successfully sent to SQS.",
                    "MessageId": response["MessageId"],
                }
            ),
        }

    except Exception as e:
        # Handle any exceptions and return an error response
        return {"statusCode": 500, "body": json.dumps({"error": str(e)})}
