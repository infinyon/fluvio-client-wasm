import { Offset, Fluvio } from '../../../../../wasm-bindgen-test';

const createUUID = () => {
  return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function(c) {
    var r = Math.random() * 16 | 0, v = c == 'x' ? r : (r & 0x3 | 0x8);
    return v.toString(16);
  });
};
const topic = createUUID();

var fluvio;
export const setup = async () => {
  fluvio  = await Fluvio.connect("ws://localhost:3000");
  const admin = await fluvio.admin();
  await admin.createTopic(topic, 1);
}
export const teardown = async () => {
  const admin = await fluvio.admin();
  await admin.deleteTopic(topic);
}

export const test = async () => {
  const producer = await fluvio.topicProducer(topic);
  await producer.send("", `count`);
  const offset = Offset.fromEnd(1);

  const consumer = await fluvio.partitionConsumer(topic, 0);
  let stream = await consumer.stream(offset); // this is a work around as Offset is not in scope.
  let next = await stream.next();

  let count = 0;
  const userAgent = navigator.userAgent;
  let before = new Date();
  while (count < 100) {
    count++;
    let in_record = `${count}-${userAgent}`;
    await producer.send("", in_record);
    let next = await stream.next();
    let out_record= `${next.valueString()}`;
    if (in_record !== out_record) {
      throw `Records do not match! ${in_record} != ${out_record}`;
    }
  }
}
