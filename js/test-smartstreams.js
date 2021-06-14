// In this example, we produce and consume records from Topic "foobar"
// We send the following records:
//
// ("", "Record with a")
// ("", "little 'a', count 0")
// ("", "BIG 'A', count 1")
// ("", "little 'a', count 2")
// ("", "BIG 'A', count 3")
// ("", "little 'a', count 4")
// ("", "BIG 'A', count 5")
// ("", "little 'a', count 6")
// ("", "BIG 'A', count 7")
// ("", "little 'a', count 8")
// ("", "BIG 'A', count 9")
//
// However, in this example we are also applying a SmartStream Filter
// to the consumer. This filter keeps only records that contain an 'a'
// in them. If everything works correctly, you should see the following
// records appear when running the example:
//
// KEY: , VALUE: Record with a
// KEY: , VALUE: little 'a', count 0
// KEY: , VALUE: little 'a', count 2
// KEY: , VALUE: little 'a', count 4
// KEY: , VALUE: little 'a', count 6
// KEY: , VALUE: little 'a', count 8

import("../pkg").then(async ({Fluvio, Offset, ConsumerConfig}) => {
  const TOPIC = "foobar";
  const fluvio = await Fluvio.connect("ws://localhost:3000")

  const producer = await fluvio.topicProducer(TOPIC);
  await producer.send("", 'Record with a');

  // Set up Consumer using a SmartStream filter.
  // This filter keeps only Records whose value contains an 'a'.
  const consumer = await fluvio.partitionConsumer(TOPIC, 0);
  const config = new ConsumerConfig();
  const {filter} = await import("./smartstream.js");
  config.wasmFilterBase64 = filter;
  let stream = await consumer.streamWithConfig(Offset.beginning(), config)

  let count = 0;
  let before = new Date();
  while (count < 10000) {

    // Every other iteration, send a record with 'a'
    if (count % 2 === 0) {
      await producer.send("", `little 'a', count ${count}`);
    } else {
      await producer.send("", `BIG 'A', COUNT ${count}`);
    }

    let next = await stream.next();
    console.log(`KEY: ${next.keyString()}, VALUE: ${next.valueString()}`);
    count++;
  }
  let after = new Date();
  console.log(`The received ${count} in took ${after - before} ms`);
});
