
export const simple = async (fluvio, offset) => {
  const topic = "simple-js-test";
  const admin = await fluvio.admin();
  try {
    await admin.createTopic(topic, 1);
  } catch(e) {
  }

  const producer = await fluvio.topicProducer(topic);
  await producer.send("", `count`);

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
  await admin.deleteTopic(topic);
}
