import React from "react";
import Parameters from "./components/Parameters";
import ManualOverride from "./components/ManualOverride";

function ControlPanel(): JSX.Element {
  return (
    <div className="w-full h-full py-2 pr-2 space-y-4 overflow-auto">
      <Parameters />
      <ManualOverride />
    </div>
  );
}

export default ControlPanel;
