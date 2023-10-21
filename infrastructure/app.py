from os import path

import aws_cdk as cdk
from aws_cdk import aws_apigateway as apigw
from constructs import Construct
from cargo_lambda_cdk import RustFunction
from aws_cdk import aws_lambda as lambda_
from aws_cdk import aws_ssm as ssm


class ResumaiStack(cdk.Stack):
    def __init__(self, scope: Construct, construct_id: str, **kwargs) -> None:
        super().__init__(scope, construct_id, **kwargs)

        bucket = cdk.aws_s3.Bucket(self, "resumai")

        bucket.add_lifecycle_rule(
            expiration=cdk.Duration.days(1),
            enabled=True,
            prefix="resumes/",
        )

        # enable cors on the bucket
        bucket.add_cors_rule(
            allowed_methods=[
                cdk.aws_s3.HttpMethods.GET,
                cdk.aws_s3.HttpMethods.PUT,
                cdk.aws_s3.HttpMethods.POST,
            ],
            allowed_origins=["*"],
            allowed_headers=["*"],
        )

        layers = [
            lambda_.LayerVersion.from_layer_version_arn(
                self,
                id="lambda-adapter-layer",
                layer_version_arn=f"arn:aws:lambda:{self.region}:753240598075:layer:LambdaAdapterLayerX86:17",
            )
        ]

        lambda_function_api = RustFunction(
            self,
            "resumai-api-lambda",
            binary_name="resumai",
            layers=layers,
            manifest_path=path.join("..", "backend"),
            environment={
                "S3_BUCKET": bucket.bucket_name,
            },
            timeout=cdk.Duration.minutes(1),
        )

        # Grant the Lambda function permissions to write to the S3 bucket
        bucket.grant_read_write(lambda_function_api)

        # Create API Gateway HTTP API and integrate it with the Lambda function
        http_api = apigw.LambdaRestApi(
            self,
            id="resumai-api-gw",
            handler=lambda_function_api,
            default_cors_preflight_options=apigw.CorsOptions(  # Enable CORS by default
                allow_origins=apigw.Cors.ALL_ORIGINS,
                allow_methods=apigw.Cors.ALL_METHODS,
            ),
            proxy=True,
        )

        ssm_param = ssm.StringParameter.from_secure_string_parameter_attributes(
            self,
            "OpenAIKey",
            parameter_name="/prod/resumai/openai-key",
        )

        lambda_function_resume_evaluator = RustFunction(
            self,
            "resumai-evaluator-lambda",
            binary_name="evaluator",
            layers=layers,
            manifest_path=path.join("..", "backend"),
            environment={
                "OPENAI_KEY_PARAM": ssm_param.parameter_name,
                "S3_BUCKET": bucket.bucket_name,
            },
            timeout=cdk.Duration.minutes(15),
        )

        bucket.grant_read_write(lambda_function_resume_evaluator)
        ssm_param.grant_read(lambda_function_resume_evaluator)

        bucket.add_event_notification(
            cdk.aws_s3.EventType.OBJECT_CREATED,
            cdk.aws_s3_notifications.LambdaDestination(
                lambda_function_resume_evaluator
            ),
            cdk.aws_s3.NotificationKeyFilter(prefix="resumes/"),
        )

        cdk.CfnOutput(self, "Function ARN", value=lambda_function_api.function_arn)
        cdk.CfnOutput(self, "API URL", value=http_api.url)


app = cdk.App()
ResumaiStack(app, "ResumaiStack")
app.synth()
