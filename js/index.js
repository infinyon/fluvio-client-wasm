import("../pkg").then(async fluvioWasm => {
  var Fluvio = fluvioWasm.Fluvio;
  var Offset = fluvioWasm.Offset;

  const fluvio = await Fluvio.connect("ws://localhost:3000")
  const consumer = await fluvio.partitionConsumer("foobar", 0);
  const stream = await consumer.stream(Offset.beginning());

  console.log("STREAM: ", stream);
  const one = await stream.next();
  console.log("ONE: ", one);

  for await (const record of stream) {
    console.log("GOT RECORD: ", record);
  }

  // while (true) {
  //   try {
  //     const fluvio = await Fluvio.connect("ws://localhost:3000")
  //     const producer = await fluvio.topicProducer("foobar");
  //     await producer.send("", `count`);
  //
  //     const consumer = await fluvio.partitionConsumer("foobar", 0);
  //     let stream = await consumer.stream(Offset.fromEnd(1))
  //     const userAgent = navigator.userAgent;
  //
  //     let count = 0;
  //     let before = new Date();
  //     while (count < 10000) {
  //
  //       try {
  //         await producer.send("", `${count}-${userAgent}`);
  //         let next = await stream.next();
  //         let text = `${next.valueString()}`;
  //         console.log(text);
  //         document.body.innerHTML =
  //           `<div>${text}</div>` +
  //           document.body.innerHTML;
  //         count++;
  //       } catch (e) {
  //         console.error(e);
  //         console.error(e.message);
  //         console.error(e.stack);
  //         let text = `${e} - ${userAgent}`;
  //         document.body.innerHTML =
  //           `<div>${text}</div>` +
  //           document.body.innerHTML;
  //         break;
  //       }
  //       await sleep(50);
  //     }
  //     let after = new Date();
  //     console.log(`The recieved ${count} in took ${after - before} ms`);
  //   } catch (e) {
  //     console.error(e);
  //     console.error(e.message);
  //     console.error(e.stack);
  //     break;
  //   }
  //   await sleep(5000);
  // }
});

const sleep = (milliseconds) => {
  return new Promise(resolve => setTimeout(resolve, milliseconds))
}
