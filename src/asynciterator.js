export const build_asynciterator = (stream) => {
  stream[Symbol.asyncIterator] = () => stream;
  return stream;
};
