import React from "react";
import Graph from "./components/Graph";
import ControlPanel from "./components/ControlPanel";

function App(): JSX.Element {
  return (
    <div className="w-full h-screen bg-slate-100">
      <div className="flex w-full h-full">
        <div className="flex-1 h-full">
          <Graph />
        </div>

        <div className="h-full w-96">
          <ControlPanel />
        </div>
      </div>
    </div>
  );
}

export default App;
