import { Offset, Fluvio } from "../../../../../wasm-bindgen-test"
import { createUUID } from "../utils.js";
const { filter_map } = await import("./filter_map_code.js");

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
    
    const test_data = [2, 4, 5, 6, 7, 8, 9];
    for(const i of test_data) {
        producer.send(null, i.toString());
    }

    // Set up Consumer with SmartStream filter_map
    // The test module passes only even numbers, divided by 2
    const consumer = await fluvio.allPartitionsConsumer(topic);
    const config = {
        smartmoduleType: "filter_map",
        smartmoduleData: filter_map,
    };
    let stream = await consumer.streamWithConfig(Offset.beginning(), config);


    const expected = [1, 2, 3, 4];

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
