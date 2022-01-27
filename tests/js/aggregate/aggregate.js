import { Offset, Fluvio } from "../../../../../wasm-bindgen-test";
import { createUUID } from "../utils.js";
const { aggregate } = await import("./aggregate_code.js");

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
    smartmoduleType: "aggregate",
    smartmoduleData: aggregate,
  };
  let stream = await consumer.streamWithConfig(Offset.beginning(), config);

  const numbers = ["1", "2", "4", "4", "4", "4", "4", "4"];

  for (const num of numbers) {
    await producer.send(undefined, num);
  }

  for (let i = 0; i < numbers.length; i++) {
    let next = await stream.next();
    console.log(`KEY: ${next.keyString()}, VALUE: ${next.valueString()}`);
  }
};
