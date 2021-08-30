// In this example, we produce and consume records from Topic "foobar"
// We send the following records:
//
// ("", "apple")
// ("", "APPLE")
// ("", "banana")
// ("", "BANANA")
// ("", "cranberry")
// ("", "CRANBERRY")
//
// However, in this example we are also applying a SmartStream Filter
// to the consumer. This filter keeps only records that contain an 'a'
// in them. If everything works correctly, you should see the following
// records appear when running the example:
//
// KEY: , VALUE: apple
// KEY: , VALUE: banana
// KEY: , VALUE: cranberry

import("../pkg").then(async ({Fluvio, Offset}) => {
  const TOPIC = "foobar";
  const fluvio = await Fluvio.connect("ws://localhost:3000")

  const producer = await fluvio.topicProducer(TOPIC);

  // Set up Consumer using a SmartStream filter.
  // This filter keeps only Records whose value contains an 'a'.
  const consumer = await fluvio.partitionConsumer(TOPIC, 0);

  const { filter } = await import("./test-filter-code.js");
  const config = {
    smartstreamType: "filter",
    smartstream: filter,
  };
  let stream = await consumer.streamWithConfig(Offset.beginning(), config)

  const mixedFruits = [
    "apple",
    "APPLE",
    "banana",
    "BANANA",
    "cranberry",
    "CRANBERRY",
  ];

  for (const fruit of mixedFruits) {
    await producer.send("", fruit);
  }

  for (let i = 0; i < mixedFruits.length / 2; i++) {
    let next = await stream.next();
    console.log(`KEY: ${next.keyString()}, VALUE: ${next.valueString()}`);
  }
});
