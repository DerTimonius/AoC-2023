export function solveDay1(type: "actual" | "basic") {
  const text = Bun.file(type === "basic" ? "basic.txt" : "actual.txt");

  console.log({ text });
}
