import * as cdk from "@aws-cdk/core";
import * as ecspatterns from "@aws-cdk/aws-ecs-patterns";
import * as ecs from "@aws-cdk/aws-ecs";
import * as ec2 from "@aws-cdk/aws-ec2";
import * as dynamodb from "@aws-cdk/aws-dynamodb";
import * as route53 from "@aws-cdk/aws-route53";
import * as elasticloadbalancingv2 from "@aws-cdk/aws-elasticloadbalancingv2";

const domainName = "dev.getft.io";

export class CdkStack extends cdk.Stack {
  constructor(scope: cdk.Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const hostedZone = route53.HostedZone.fromLookup(this, "HostedZone", {
      domainName,
    });

    const vpc = new ec2.Vpc(this, "VPC");

    const cluster = new ecs.Cluster(this, "Cluster", {
      vpc,
    });

    const table = new dynamodb.Table(this, "Table", {
      partitionKey: { name: "id", type: dynamodb.AttributeType.STRING },
    });

    const service = new ecspatterns.ApplicationLoadBalancedFargateService(
      this,
      "Service",
      {
        cluster,
        memoryLimitMiB: 1024,
        publicLoadBalancer: true,
        domainName: `grpc.${domainName}`,
        domainZone: hostedZone,
        protocol: elasticloadbalancingv2.ApplicationProtocol.HTTPS,
        protocolVersion: elasticloadbalancingv2.ApplicationProtocolVersion.GRPC,
        platformVersion: ecs.FargatePlatformVersion.LATEST,
        taskImageOptions: {
          image: ecs.ContainerImage.fromAsset("."),
          environment: {
            PORT: "80",
            // ADDRESS: "0.0.0.0",
            PIZZA_TABLE: table.tableName,
          },
        },
        desiredCount: 1,
      }
    );

    service.targetGroup.configureHealthCheck({
      healthyGrpcCodes: "12",
      interval: cdk.Duration.seconds(5),
      timeout: cdk.Duration.seconds(3),
      healthyThresholdCount: 3,
      unhealthyThresholdCount: 5,
    });

    service.targetGroup.setAttribute(
      "deregistration_delay.timeout_seconds",
      "3"
    );

    table.grantReadWriteData(service.service.taskDefinition.taskRole);
  }
}
