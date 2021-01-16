import { handler } from "wasm_pkg";

export const originRequestHandler = async (
  event: AWSLambda.CloudFrontRequestEvent,
  context: AWSLambda.Context
): Promise<AWSLambda.CloudFrontRequestResult> => {
  return handler(event, context)
};
