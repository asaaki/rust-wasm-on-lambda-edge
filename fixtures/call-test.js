const originRequest = require("../build.function/origin-request");
// const otherTrigger = require("../function/REPLACE-ME");
const sampleEvent = require("./origin-request.json");

// mostly useful for logging and tracing purposes
const sampleContext = {
  callbackWaitsForEmptyEventLoop: "[Getter / Setter]",
  succeed: "[Function]",
  fail: "[Function]",
  done: "[Function]",
  functionVersion: "2",
  functionName: "edge-node-wasm-test",
  memoryLimitInMB: "128",
  logGroupName: "/aws/lambda/edge-node-wasm-test",
  logStreamName: "2021/01/16/[2]d3b48f1b5fbe4b9690468dce768c20b5",
  clientContext: undefined,
  identity: undefined,
  invokedFunctionArn:
    "arn:aws:lambda:us-east-1:258911450629:function:edge-node-wasm-test:2",
  awsRequestId: "9bdfd183-43f8-45dd-baf7-26f0e1511bda",
  getRemainingTimeInMillis: "[(Function: getRemainingTimeInMillis)]",
};
// if you need more self-introspection, also consult process.*, but be careful what you expose,
// for example `process.env` might include sensitive data

(async () => {
  console.log(
    global.r = await originRequest.handler(sampleEvent, sampleContext)
  );
})();
