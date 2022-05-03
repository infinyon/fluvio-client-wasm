import { Offset, Fluvio } from "../../../../../wasm-bindgen-test";
import { createUUID } from "../utils.js";

const topic = createUUID();
const NUMB_RECORDS = 100;

var fluvio;
export const setup = async () => {
  fluvio = await Fluvio.connect("ws://localhost:3000");
  const admin = await fluvio.admin();
  for (let i = 0; i < 3; i++) {
    try {
      await admin.createTopic(topic, 1);
      break;
    } catch (e) {
      console.error(`${e.message}`);
    }
  }
};
export const teardown = async () => {
  const admin = await fluvio.admin();
  await admin.deleteTopic(topic);
};

export const test = async () => {
  const compression = "snappy";
  const lingerTime = 50;
  const batchSize = 2048;
  const config = {
    lingerTime,
    batchSize,
    compression,
  };

  const producer = await fluvio.topicProducerWithConfig(topic, config);
  await producer.send("", `count`);
  const offset = Offset.beginning();

  const consumer = await fluvio.partitionConsumer(topic, 0);
  let stream = await consumer.stream(offset); // this is a work around as Offset is not in scope.
  let next = await stream.next();

  let count = 0;
  const userAgent = navigator.userAgent;
  while (count < NUMB_RECORDS) {
    count++;
    let in_record = `${count}-${userAgent}`;
    await producer.send("", in_record);
  }
  count = 0;
  while (count < NUMB_RECORDS) {
    count++;
    let in_record = `${count}-${userAgent}`;
    let next = await stream.next();
    let out_record = `${next.valueString()}`;
    if (in_record !== out_record) {
      throw `Records do not match! ${in_record} != ${out_record}`;
    }
  }
};
