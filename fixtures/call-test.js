const originRequest = require("../build.function/origin-request");
const sampleEvent = require("./origin-request-with-body.json");

// mostly useful for logging and tracing purposes;
// note: the actual context is an object with references to functions
const sampleContext = require("./example-context.json");

// if you need more self-introspection, also consult process.*, but be careful what you expose,
// for example `process.env` might include sensitive data

(async () => {
  const result = await originRequest.handler(sampleEvent, sampleContext);
  console.log(JSON.stringify(result, undefined, 2));
})();
