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

  // Set up Consumer using a SmartStream map.
  // This map takes all input and makes it uppercase ascii
  const consumer = await fluvio.partitionConsumer(TOPIC, 0);

  const { aggregate } = await import("./test-aggregate-code.js");
  const config = {
    smartstreamType: "aggregate",
    smartstream: aggregate,
  };
  let stream = await consumer.streamWithConfig(Offset.beginning(), config)

  const numbers = [
    "1",
    "1",
    "1",
  ];

  for (const num of numbers) {
    await producer.send("", num);
  }

  for (let i = 0; i < numbers.length; i++) {
    let next = await stream.next();
    console.log(`KEY: ${next.keyString()}, VALUE: ${next.valueString()}`);
  }
});
