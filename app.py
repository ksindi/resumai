from aws_cdk import (
    core,
    aws_apigateway as apigateway,
    aws_lambda as _lambda,
    aws_sqs as sqs,
    aws_lambda_event_sources as lambda_event_sources,
)


class WebhookProcessingStack(core.Stack):
    def __init__(self, scope: core.Construct, id: str, **kwargs) -> None:
        super().__init__(scope, id, **kwargs)

        # Define the Dead Letter Queue
        dlq = sqs.Queue(
            self, "DeadLetterQueue", retention_period=core.Duration.days(14)
        )

        # Define the SQS queue with a redrive policy pointing to the DLQ
        processing_queue = sqs.Queue(
            self,
            "ProcessingQueue",
            dead_letter_queue={
                "max_receive_count": 5,  # adjust as needed
                "queue": dlq,
            },
        )

        # Define the Lambda function that gets triggered by API Gateway
        api_lambda = _lambda.Function(
            self,
            "ApiHandler",
            runtime=_lambda.Runtime.PYTHON_3_8,
            handler="api_handler.handler",
            code=_lambda.Code.from_asset("path_to_your_lambda_directory"),
            memory_size=512,
            environment={"QUEUE_URL": processing_queue.queue_url},
        )

        # Define the API Gateway that triggers the above Lambda
        api = apigateway.LambdaRestApi(  # noqa: F841
            self,
            "WebhookAPI",
            handler=api_lambda,
        )

        # Define the Lambda function that gets triggered by SQS
        processing_lambda = _lambda.Function(
            self,
            "ProcessingHandler",
            runtime=_lambda.Runtime.PROVIDED_AL2,
            handler="not.used",  # Not used in provided runtime
            code=_lambda.Code.from_asset("path_to_rust_deployment_package"),  # TODO
            memory_size=512,
        )

        # Grant the API Lambda permissions to send messages to the SQS queue
        processing_queue.grant_send_messages(api_lambda)

        # Add the SQS event source to the processing Lambda
        processing_lambda.add_event_source(
            lambda_event_sources.SqsEventSource(processing_queue)
        )


app = core.App()
WebhookProcessingStack(app, "WebhookProcessingStack")
app.synth()
