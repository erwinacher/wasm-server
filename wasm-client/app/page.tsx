"use client";

import Image from "next/image";
import { useEffect, useState } from "react";

// load WASM module using instantiateStreaming
async function loadWasm(url: string) {
  const result = await WebAssembly.instantiateStreaming(fetch(url), {});
  return result.instance;
}

// read null-terminated string
function readCString(memory: WebAssembly.Memory, ptr: number): string {
  const bytes = new Uint8Array(memory.buffer);

  let s = "";
  for (let i = ptr; bytes[i] != 0; i++) {
    s += String.fromCharCode(bytes[i]);
  }

  return s;
}

//
export default function Home() {
  const [graph, setGraph] = useState<any>(null);
  const [currentNode, setCurrentNode] = useState<string | null>(null);
  const [logs, setLogs] = useState<string[]>([]);
  const [boxColor, setBoxColor] = useState("#000");

  useEffect(() => {
    async function loadGraph() {
      console.log("Fetching graph.json...");
      const graph = await fetch("http://localhost:3002/graph.json").then((r) =>
        r.json()
      );
      setGraph(graph);
      setCurrentNode(graph.start);
    }
    loadGraph();
  }, []);

  async function step() {
    if (!graph || !currentNode) return;

    const entry = graph.nodes[currentNode];
    if (!entry) {
      console.error("Node missing:", currentNode);
      return;
    }

    const url = "http://localhost:3002" + entry.wasm;
    console.log("Loading module:", url);

    const instance = await loadWasm(url);
    const exports = instance.exports as any;
    const memory = exports.memory as WebAssembly.Memory;

    const textPtr = exports.get_text();
    const text = readCString(memory, textPtr);

    const rgba = exports.get_color();
    const hex = "#" + (rgba >>> 8).toString(16).padStart(6, "0");

    setLogs((prev) => [...prev, `[${currentNode}] ${text}`]);
    setBoxColor(hex);

    // Move to the next node
    setCurrentNode(entry.next);
  }
  return (
    <div style={{ padding: 30, fontFamily: "sans-serif" }}>
      <h1>WASM Graph Stepper</h1>

      <div
        style={{
          width: 200,
          height: 200,
          background: boxColor,
          borderRadius: 10,
          marginTop: 20,
          marginBottom: 20,
          transition: "background 0.5s ease",
        }}
      />

      {!graph && <p>Loading graph…</p>}

      {graph && currentNode && (
        <button
          onClick={step}
          style={{
            padding: "10px 20px",
            fontSize: "16px",
            marginBottom: "20px",
            cursor: "pointer",
          }}
        >
          Step to Next Module
        </button>
      )}

      {graph && !currentNode && (
        <p>
          <strong>Graph Complete!</strong>
        </p>
      )}

      <h2>Execution Log</h2>
      <pre style={{ background: "#f0f0f0", padding: 10 }}>
        {logs.join("\n")}
      </pre>
    </div>
  );
}
