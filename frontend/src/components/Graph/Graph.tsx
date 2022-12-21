import React from "react";
import ParentSize from "@visx/responsive/lib/components/ParentSize";
import appleStock from "@visx/mock-data/lib/mocks/appleStock";
import AreaPlot from "./components/AreaPlot";
import Select from "react-select";

const stock = appleStock.slice(800);

const options = [
  { value: "1D", label: "Last Day" },
  { value: "1W", label: "Last Week" },
];

function Graph(): JSX.Element {
  const [activeFilter, setActiveFilter] = React.useState(options[0]);

  return (
    <div className="w-full h-full p-2">
      <div className="flex flex-col w-full h-full p-2 space-y-2 bg-white rounded-md shadow-md">
        <div className="flex items-center justify-end">
          <div className="w-96">
            <Select
              options={options}
              isSearchable={false}
              isClearable={false}
              onChange={(newValue) =>
                newValue &&
                setActiveFilter({
                  label: newValue.label,
                  value: newValue.value,
                })
              }
              value={activeFilter}
            />
          </div>
        </div>
        <div className="flex-1">
          <ParentSize>
            {({ width, height }) => (
              <AreaPlot width={width} height={height} data={stock} />
            )}
          </ParentSize>
        </div>
      </div>
    </div>
  );
}

export default Graph;
