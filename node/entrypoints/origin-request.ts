// import * as config from "./config";

import { originRequestHandler } from "../lib/handlers/origin-request"

export const handler: AWSLambda.CloudFrontRequestHandler = (
  event: AWSLambda.CloudFrontRequestEvent,
  context: AWSLambda.Context
) => originRequestHandler(event, context);
