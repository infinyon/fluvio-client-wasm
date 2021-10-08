import { Offset, Fluvio } from '../../../../../wasm-bindgen-test';

const createUUID = () => {
  return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function(c) {
    var r = Math.random() * 16 | 0, v = c == 'x' ? r : (r & 0x3 | 0x8);
    return v.toString(16);
  });
};
const topic = createUUID();
const connector_name = topic;
const connector_type = "test-connector";
const max_records = 10;

var fluvio;

export const setup = async () => {
  fluvio  = await Fluvio.connect("ws://localhost:3000");
  const admin = await fluvio.admin();
  //Fluvio.setupDebugging(true);
  await admin.createConnector(
    connector_name,
    connector_type,
    {
      topic,
      count: `${max_records}`,
      timeout: "10",
    },
  );
  try {
    await admin.createTopic(topic, 1);
  } catch(e) {
    if(`${e.message}` != `AdminApi(Code(TopicAlreadyExists, Some("topic '${topic}' already defined")))`) {
      throw e;
    }
  }
}

export const teardown = async () => {
  const admin = await fluvio.admin();
  await admin.deleteTopic(topic);
  await admin.deleteConnector(connector_name, connector_name);
}

export const test = async () => {
  const consumer = await fluvio.partitionConsumer(topic, 0);
  let stream = await consumer.stream(Offset.beginning()); // this is a work around as Offset is not in scope.

  const admin = await fluvio.admin();
  const connectors = await admin.listConnectors();


  let count = 0;
  const userAgent = navigator.userAgent;
  for(let i = 1; i < max_records; i++) {
    let next = await stream.next();
    let out_record = `${next.valueString()}`;
    console.log(`GOT A RECORD! ${out_record}`);
    let expected = `Hello, Fluvio! - ${i}`;
    if (expected !== out_record) {
      throw `Records do not match! ${expected} != ${out_record}`;
    }
  }
}
