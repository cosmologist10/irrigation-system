import React, { useCallback } from "react";
import { GiWateringCan as IrrigationIcon } from "react-icons/gi";
import { BASE_URL } from "../../../constants/url";

function ManualOverride(): JSX.Element {
  const onSubmit = useCallback(() => {
   fetch(`${BASE_URL}/app/sensors/irrigation/plant_1`)
      .then(() => console.log("start irrigation request sent"))
      .catch((error) => {
        console.error(error);
      });
  }, []);

  return (
    <div className="w-full p-2 space-y-2 overflow-hidden bg-white rounded-md shadow-md">
      <div className="flex items-center justify-center p-2 -mx-2 -mt-2 text-lg bg-red-100 border border-red-400 rounded-t-md heading-font">
        Override
      </div>

      <button
        onClick={onSubmit}
        className="flex items-center justify-center w-full p-2 space-x-4 bg-blue-100 border border-blue-400 rounded-md hover:bg-blue-300"
      >
        <div>
          <IrrigationIcon size={42} />
        </div>
        <div className="text-lg">Start Manual Irrigation</div>
      </button>
    </div>
  );
}

export default ManualOverride;
