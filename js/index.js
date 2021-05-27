import("../pkg").then(async fluvioWasm => {
  var Fluvio = fluvioWasm.Fluvio;
  var Offset = fluvioWasm.Offset;
  const fluvio = await Fluvio.connect("ws://localhost:3000")
  const fluvio_2 = await Fluvio.connect("ws://localhost:3000")
  const producer = await fluvio_2.topic_producer("foobar");
  await producer.send("", `count`);

  const consumer = await fluvio.partition_consumer("foobar", 0);
  let stream = await consumer.stream(Offset.from_end(10))

  let count = 0;
  let before = new Date();
  while (count < 1000) {

    await producer.send("", `count-${count}`);
    let next = await stream.next();
    console.log(`${next.keyString()} - ${next.valueString()}`);
    count++;
  }
  let after = new Date();
  console.log(`The recieved ${count} in took ${after - before} ms`);
});
