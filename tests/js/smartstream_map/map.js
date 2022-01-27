import { Offset, Fluvio } from "../../../../../wasm-bindgen-test";
import { createUUID } from "../utils.js";
import { mapCode } from "./map_code.js";

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
      } catch (e) {}
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

  // Set up Consumer using a SmartStream map.
  // This map takes all input and makes it uppercase ascii
  const consumer = await fluvio.partitionConsumer(topic, 0);

  const config = {
    smartstreamType: "map",
    smartstream: mapCode,
  };
  let stream = await consumer.streamWithConfig(Offset.beginning(), config);

  const mixedFruits = ["apple", "banana", "cranberry"];

  for (const fruit of mixedFruits) {
    await producer.send(undefined, fruit);
  }

  for (let i = 0; i < mixedFruits.length; i++) {
    let next = await stream.next();
    let out = next.valueString();
    let expected = mixedFruits[i].toUpperCase();
    if (expected !== out) {
      throw `Records do not match! ${expected} != ${out}`;
    }
  }
};
