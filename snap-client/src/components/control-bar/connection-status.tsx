import { useMemo } from "react";
import { ReadyState } from "react-use-websocket";

interface ConnectionStatusProps {
  readyState: ReadyState;
}

const CONNECTION_STATUS_MAP = {
  [ReadyState.CONNECTING]: {
    text: "Connecting",
    color: "#fdcb6e",
  },
  [ReadyState.OPEN]: {
    text: "Connected",
    color: "#00b894",
  },
  [ReadyState.CLOSING]: {
    text: "Closing",
    color: "#fab1a0",
  },
  [ReadyState.CLOSED]: {
    text: "Closed",
    color: "#636e72",
  },
  [ReadyState.UNINSTANTIATED]: {
    text: "Uninstantiated",
    color: "#2d3436",
  },
} as const;

const UNKNOWN_STATE = { text: "Unknown", color: "#a29bfe" };

export const ConnectionStatus = ({ readyState }: ConnectionStatusProps) => {
  const parsedState = useMemo(
    () => CONNECTION_STATUS_MAP[readyState] ?? UNKNOWN_STATE,
    [readyState],
  );

  return (
    <div className="flex flex-grow flex-row items-center mr-1 ml-2">
      <div
        className="flex w-5 h-5 mr-2 justify-center items-center rounded-full"
        style={{ backgroundColor: `${parsedState.color}50` }}
      >
        <div
          className="w-3 h-3 rounded-full"
          style={{ backgroundColor: parsedState.color }}
        ></div>
      </div>
      {parsedState.text}
    </div>
  );
};
