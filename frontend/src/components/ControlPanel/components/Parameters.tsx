import React, { useCallback, useState } from "react";

const DEFAULT_VALUES = {
  irrigationTime: 5,
  minimumIrrigationInterval: 10,
  capacityBuffer: 500,
  signalPin: 16,
};

const inputCheck = (value: string, cb: (value: string) => void) => {
  if (value === "") {
    cb("");
  } else if (value.match(/^[0-9]+$/)) {
    cb(value);
  }
};

function Parameters(): JSX.Element {
  const [irrigationTime, setIrrigationTime] = useState(
    `${DEFAULT_VALUES.irrigationTime}`
  );
  const [minimumIrrigationInterval, setMinimumIrrigationInterval] = useState(
    `${DEFAULT_VALUES.irrigationTime}`
  );
  const [capacityBuffer, setCapacityBuffer] = useState(
    `${DEFAULT_VALUES.capacityBuffer}`
  );
  const [signalPin, setSignalPin] = useState(`${DEFAULT_VALUES.signalPin}`);

  const onSubmit = useCallback(() => {
    const params = {
      irrigationTime: parseInt(irrigationTime),
      minimumIrrigationInterval: parseInt(minimumIrrigationInterval),
      capacityBuffer: parseInt(capacityBuffer),
      signalPin: parseInt(signalPin),
    };

    if (irrigationTime === "") {
      setIrrigationTime(`${DEFAULT_VALUES.irrigationTime}`);
      params.irrigationTime = DEFAULT_VALUES.irrigationTime;
    }
    if (minimumIrrigationInterval === "") {
      setMinimumIrrigationInterval(
        `${DEFAULT_VALUES.minimumIrrigationInterval}`
      );
      params.minimumIrrigationInterval =
        DEFAULT_VALUES.minimumIrrigationInterval;
    }
    if (capacityBuffer === "") {
      setCapacityBuffer(`${DEFAULT_VALUES.capacityBuffer}`);
      params.capacityBuffer = DEFAULT_VALUES.capacityBuffer;
    }
    if (signalPin === "") {
      setSignalPin(`${DEFAULT_VALUES.signalPin}`);
      params.signalPin = DEFAULT_VALUES.signalPin;
    }

    console.log(params);
  }, [irrigationTime, minimumIrrigationInterval, capacityBuffer, signalPin]);

  return (
    <div className="w-full p-2 space-y-2 overflow-hidden bg-white rounded-md shadow-md">
      <div className="flex items-center justify-center p-2 -mx-2 -mt-2 text-lg bg-blue-100 border border-blue-400 rounded-t-md heading-font">
        Parameters
      </div>

      <div className="space-y-2">
        <div className="flex items-center justify-between">
          <div>Irrigation time [s] :</div>
          <input
            value={irrigationTime}
            onChange={(event) =>
              inputCheck(event.target.value, setIrrigationTime)
            }
            className="w-16 px-1 py-0.5 border border-purple-400 rounded-md outline-none focus:border-red-400"
          ></input>
        </div>

        <div className="flex items-center justify-between">
          <div>Minimum irrigation interval [min] :</div>
          <input
            value={minimumIrrigationInterval}
            onChange={(event) =>
              inputCheck(event.target.value, setMinimumIrrigationInterval)
            }
            className="w-16 px-1 py-0.5 border border-purple-400 rounded-md outline-none focus:border-red-400"
          ></input>
        </div>

        <div className="flex items-center justify-between">
          <div>Capacity Buffer :</div>
          <input
            value={capacityBuffer}
            onChange={(event) =>
              inputCheck(event.target.value, setCapacityBuffer)
            }
            className="w-16 px-1 py-0.5 border border-purple-400 rounded-md outline-none focus:border-red-400"
          ></input>
        </div>

        <div className="flex items-center justify-between">
          <div>Signal Pin :</div>
          <input
            value={signalPin}
            onChange={(event) => inputCheck(event.target.value, setSignalPin)}
            className="w-16 px-1 py-0.5 border border-purple-400 rounded-md outline-none focus:border-red-400"
          ></input>
        </div>
      </div>

      <button
        onClick={onSubmit}
        className="flex items-center justify-center w-full p-2 bg-green-100 border border-green-400 rounded-md hover:bg-green-300"
      >
        Submit
      </button>
    </div>
  );
}

export default Parameters;
