import { Offset, Fluvio } from "../../../../../wasm-bindgen-test";
import { createUUID } from "../utils.js";

const topic = createUUID();

var fluvio;
export const setup = async () => {
  fluvio = await Fluvio.connect("ws://localhost:3000");
  const admin = await fluvio.admin();
  for (let i = 0; i < 3; i++) {
    try {
      await admin.createTopic(topic, 1);
      break;
    } catch (e) {
      try {
        await admin.deleteTopic(topic);
      } catch (e) {
          console.error(`error deleting: ${e.message}`);
      }

      console.error(`${e.message}`);
    }
  }
};
export const teardown = async () => {
  const admin = await fluvio.admin();
  await admin.deleteTopic(topic);
};

export const test = async () => {
  const producer = await fluvio.topicProducer(topic);
  await producer.send("", `count`);
  const offset = Offset.beginning();

  const consumer = await fluvio.partitionConsumer(topic, 0);
  let stream = await consumer.stream(offset); // this is a work around as Offset is not in scope.
  let next = await stream.next();

  let count = 0;
  const userAgent = navigator.userAgent;
  while (count < 100) {
    count++;
    let in_record = `${count}-${userAgent}`;
    await producer.send("", in_record);
  }
  count = 0;
  while (count < 100) {
    count++;
    let in_record = `${count}-${userAgent}`;
    let next = await stream.next();
    let out_record = `${next.valueString()}`;
    if (in_record !== out_record) {
      throw `Records do not match! ${in_record} != ${out_record}`;
    }
  }
};
