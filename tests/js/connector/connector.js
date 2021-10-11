import { Offset, Fluvio } from '../../../../../wasm-bindgen-test';
import { createUUID } from '../utils.js';

const topic = createUUID();
const connector_name = topic;
const connector_type = "test-connector";
const max_records = 10;

var fluvio;

export const setup = async () => {
  fluvio  = await Fluvio.connect("ws://localhost:3000");
  const admin = await fluvio.admin();
  console.log(`Starting connector named ${connector_name} on topic ${topic}`);
  await admin.createConnector(
    connector_name,
    connector_type,
    {
      topic,
      count: `${max_records}`, // These must be strings or the rust fails to convert it.
      timeout: "10",
    },
  );
  await admin.createTopic(topic, 1);
}

export const teardown = async () => {
  const admin = await fluvio.admin();
  await admin.deleteTopic(topic);
  await admin.deleteConnector(connector_name, connector_name);
}

export const test = async () => {
  const consumer = await fluvio.partitionConsumer(topic, 0);
  let stream = await consumer.stream(Offset.beginning()); // this is a work around as Offset is not in scope.

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
