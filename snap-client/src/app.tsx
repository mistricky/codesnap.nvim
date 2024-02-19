import React, {
  useCallback,
  useEffect,
  useMemo,
  useRef,
  useState,
} from "react";
import useWebSocket, { ReadyState } from "react-use-websocket";
import { ControlBar, Editor, Frame, Panel } from "./components";
import { useConfig, useEvent } from "./hooks";
import { toPng, toJpeg, toBlob, toPixelData, toSvg } from "html-to-image";

const CODE_EMPTY_PLACEHOLDER = `
 ██████╗ ██████╗ ██████╗ ███████╗███████╗███╗   ██╗ █████╗ ██████╗ 
██╔════╝██╔═══██╗██╔══██╗██╔════╝██╔════╝████╗  ██║██╔══██╗██╔══██╗
██║     ██║   ██║██║  ██║█████╗  ███████╗██╔██╗ ██║███████║██████╔╝
██║     ██║   ██║██║  ██║██╔══╝  ╚════██║██║╚██╗██║██╔══██║██╔═══╝ 
╚██████╗╚██████╔╝██████╔╝███████╗███████║██║ ╚████║██║  ██║██║     
 ╚═════╝ ╚═════╝ ╚═════╝ ╚══════╝╚══════╝╚═╝  ╚═══╝╚═╝  ╚═╝╚═╝     
`;

function App() {
  const [socketUrl] = useState("ws://127.0.0.1:8080/ws");
  const { sendMessage, lastMessage } = useWebSocket(socketUrl);
  const event = useEvent(lastMessage);
  const config = useConfig(event?.config_setup);
  const frameRef = useRef<HTMLDivElement | null>(null);

  const handleCopyButtonClick = useCallback(async () => {
    if (!frameRef.current) {
      return;
    }

    const blob = await toBlob(frameRef.current);
    const clipboardItem = new ClipboardItem({ "image/png": blob! });

    navigator.clipboard.write([clipboardItem]);
  }, []);

  return (
    <div className="w-full h-full flex flex-col items-center bg-deep-gray">
      <p className="rainbow-text text-4xl font-extrabold mt-20">
        CodeSnap.nvim
      </p>
      <Panel>
        <ControlBar onCopyClick={handleCopyButtonClick}></ControlBar>
        <div className="rounded-xl overflow-hidden">
          <Frame ref={frameRef} watermark={config?.watermark}>
            <Editor
              language={event?.code?.language}
              macStyleTitleBar={config?.mac_window_bar}
            >
              {event?.code?.content ?? CODE_EMPTY_PLACEHOLDER}
            </Editor>
          </Frame>
        </div>
      </Panel>
    </div>
  );
}

export default App;
