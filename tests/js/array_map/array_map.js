import { Offset, Fluvio } from "../../../../../wasm-bindgen-test"
import { createUUID } from "../utils.js";
const { array_map } = await import("./array_map_code.js");

const topic = createUUID();

var fluvio;

export const setup = async () => {
    fluvio = await Fluvio.connect("ws://localhost:3000");
    const admin = await fluvio.admin();
    admin.createTopic(topic, 1);

};

export const teardown = async () => {
    const admin = await fluvio.admin();
    admin.deleteTopic(topic);
};

export const test = async () => {
    const producer = await fluvio.topicProducer(topic);
    
    // Test data
    const multipliers = [5, 13, 29];
    const indices = [1, 2, 3, 4, 5, 6, 7];

    // Send test data as JSON record
    for(const i of indices) {
        let record = {
            uselessData: "this will be stripped by the array_map_code.js smartmodule",

            // i*5, i*13, i*29
            importantNumbers: multipliers.map((n) => (n * i)),
        }

        producer.send(i.toString(), JSON.stringify(record));
    }

    // Set up Consumer with SmartStream array_map
    // This map takes each JSON record, extracts importantNumbers and re-emits
    // them individually
    const consumer = await fluvio.allPartitionsConsumer(topic);
    const config = {
        smartmoduleType: "array_map",
        smartmoduleData: array_map,
    };
    let stream = await consumer.streamWithConfig(Offset.beginning(), config);


    // 1*5, 1*13, 1*29, 2*5, ...
    const expected = indices.flatMap((i) => multipliers.map((j) => i * j))

    for(const x of expected) {
        let next = await stream.next();
        let key = next.keyString()
        let value = next.valueString();
        console.log(`KEY: ${next.keyString()}, VALUE: ${value}`);

        if (x != value) {
            throw `Records do not match ${x} != ${value}`
        }
    }
};
