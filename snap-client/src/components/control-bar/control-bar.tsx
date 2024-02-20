import { ConnectionStatus } from "./connection-status";
import { ReadyState } from "react-use-websocket";

interface ControlBarProps {
  onCopyClick(): void;
  onExportClick(): void;
  readyState: ReadyState;
}

export const ControlBar = ({
  onCopyClick,
  onExportClick,
  readyState,
}: ControlBarProps) => {
  return (
    <div
      className="bg-neutral rounded-xl mb-2 p-1 flex flex-row items-center"
      onClick={onCopyClick}
    >
      <ConnectionStatus readyState={readyState} />
      <div className="flex flex-row items-center">
        {/*
        * <button className="btn mr-1">
          <svg
            role="img"
            className="h-4 w-4 fill-neutral-500"
            viewBox="0 0 24 24"
            xmlns="http://www.w3.org/2000/svg"
          >
            <title>X</title>
            <path d="M18.901 1.153h3.68l-8.04 9.19L24 22.846h-7.406l-5.8-7.584-6.638 7.584H.474l8.6-9.83L0 1.154h7.594l5.243 6.932ZM17.61 20.644h2.039L6.486 3.24H4.298Z" />
          </svg>
        </button>
        */}
        <button className="btn mr-1" onClick={onExportClick}>
          Export
          <svg
            className="fill-neutral-content"
            xmlns="http://www.w3.org/2000/svg"
            height="24"
            viewBox="0 -960 960 960"
            width="24"
          >
            <path d="M480-320 280-520l56-58 104 104v-326h80v326l104-104 56 58-200 200ZM240-160q-33 0-56.5-23.5T160-240v-120h80v120h480v-120h80v120q0 33-23.5 56.5T720-160H240Z" />
          </svg>
        </button>
        <button className="btn">
          Copy
          <svg
            className="fill-neutral-content"
            xmlns="http://www.w3.org/2000/svg"
            height="20"
            viewBox="0 -960 960 960"
            width="20"
          >
            <path d="M360-240q-33 0-56.5-23.5T280-320v-480q0-33 23.5-56.5T360-880h360q33 0 56.5 23.5T800-800v480q0 33-23.5 56.5T720-240H360Zm0-80h360v-480H360v480ZM200-80q-33 0-56.5-23.5T120-160v-560h80v560h440v80H200Zm160-240v-480 480Z" />
          </svg>{" "}
        </button>
      </div>
    </div>
  );
};
