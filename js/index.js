import("../pkg").then(async fluvioWasm => {
  var Fluvio = fluvioWasm.Fluvio;
  var Offset = fluvioWasm.Offset;
  while (true) {
    let topic = createUUID();
    try {
      Fluvio.setupDebugging(false, 'Debug');
      const fluvio = await Fluvio.connect("ws://localhost:3000");
      const admin = await fluvio.admin();
      await admin.createTopic(topic, 1);

      const producer = await fluvio.topicProducer(topic);
      await producer.send("", `count`);

      const consumer = await fluvio.partitionConsumer(topic, 0);
      let stream = await consumer.stream(Offset.fromEnd(1))
      const userAgent = navigator.userAgent;

      let count = 0;
      let before = new Date();
      while (count < 10000) {

        try {
          await producer.send("", `${count}-${userAgent}`);
          let next = await stream.next();
          let text = `${next.valueString()}`;
          console.log(text);
          document.body.innerHTML =
            `<div>${text}</div>` +
            document.body.innerHTML;
          count++;
        } catch (e) {
          console.error(e);
          console.error(e.message);
          console.error(e.stack);
          let text = `${e} - ${userAgent}`;
          document.body.innerHTML =
            `<div>${text}</div>` +
            document.body.innerHTML;
          break;
        }
        await sleep(50);
      }
      let after = new Date();
      console.log(`The recieved ${count} in took ${after - before} ms`);
      await admin.deleteTopic(topic);
    } catch (e) {
      console.error(e);
      console.error(e.message);
      console.error(e.stack);
      break;
    }
    await sleep(5000);
  }
});

const sleep = (milliseconds) => {
  return new Promise(resolve => setTimeout(resolve, milliseconds))
}
function createUUID() {
   return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function(c) {
      var r = Math.random() * 16 | 0, v = c == 'x' ? r : (r & 0x3 | 0x8);
      return v.toString(16);
   });
}
