
export const test = async (fluvio, offset) => {
  let base_url = "http://127.0.0.1:3000";
  try {
    await fetch(`${base_url}/off`, {
      method: 'GET',
    });
  } catch (e) {
    console.error(`${e.message}`);
  }
  let error = null;
  try {
    await fluvio.topicProducer("foobar");
  } catch (e) {
    error = e;
  }
  try {
    await fetch(`${base_url}/on`, {
      method: 'GET',
    });
  } catch (e) {
    console.error(`${e.message}`);
  }
  let expectedError = "Socket(Io(Kind(NotConnected)))";
  if (error.message !== expectedError) {
    throw `Error code does not match! ${error.message} != ${expectedError}`;
  }
}

export const setup = async (fluvio) => {
}

export const teardown = async (fluvio) => {
}
