import { filterCode } from "./smartstream_filter_code.js";
import { Offset, Fluvio } from "../../../../../wasm-bindgen-test";
import { createUUID } from "../utils.js";

const topic = createUUID();
var fluvio;

export const setup = async () => {
  fluvio = await Fluvio.connect("ws://localhost:3000");
  const admin = await fluvio.admin();
  await admin.createTopic(topic, 1);
};
export const teardown = async () => {
  const admin = await fluvio.admin();
  await admin.deleteTopic(topic);
};

export const test = async () => {
  const producer = await fluvio.topicProducer(topic);
  const consumer = await fluvio.partitionConsumer(topic, 0);

  let config = {
    smartmoduleType: "filter",
    smartmoduleData: filterCode,
  };
  const fruits = [
    "apple",
    "APPLE",
    "banana",
    "BANANA",
    "cranberry",
    "CRANBERRY",
  ];

  for (const fruit of fruits) {
    await producer.send("", fruit);
    console.log(`SENT ${fruit}`);
  }

  console.log("CREATING STREAM");
  const offset = Offset.fromEnd(1);
  Fluvio.setupDebugging(true);
  let stream = await consumer.streamWithConfig(offset, config);
  console.log("CREATED STREAM");

  const lowerFruits = ["apple", "banana", "cranberry"];

  for (let i = 0; i < 3; i++) {
    console.log("WAITING ON ", i);
    let next = await stream.next();
    let value = next.valueString();
    let expected = lowerFruits[i];

    console.log(`Got stream value: ${value}`);
    if (value !== expected) {
      throw `Records do not match! ${value} !== ${expected}`;
    }
  }
};
