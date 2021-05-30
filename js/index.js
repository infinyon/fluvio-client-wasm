import("../pkg").then(async fluvioWasm => {
  var Fluvio = fluvioWasm.Fluvio;
  var Offset = fluvioWasm.Offset;

  const fluvio = await Fluvio.connect("ws://192.168.49.2:3030/api/v1/websocket/", "9003", "YOUR TOKEN HERE");

  console.log("Got a connection to fluvio!");
  const topic = "foobar";

  const consumer = await fluvio.partition_consumer(topic, 0);
  let stream = await consumer.stream(Offset.from_end(10))
  console.log(`Got a stream to for topic ${topic}`);

  let count = 0;
  let before = new Date();
  while (count < 1000) {

    //await producer.send("", `count-${count}`);
    let next = await stream.next();
    console.log(`${next.keyString()} - ${next.valueString()}`);
    count++;
  }

  /*
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
  */
});


