import React, { useCallback, useEffect, useState } from "react";
import useWebSocket, { ReadyState } from "react-use-websocket";

function App() {
  const [socketUrl, setSocketUrl] = useState("ws://127.0.0.1:8080/ws");
  const [messageHistory, setMessageHistory] = useState([]);
  const { sendMessage, lastMessage, readyState } = useWebSocket(socketUrl);

  const handleClickSendMessage = useCallback(() => {
    sendMessage("Hello");
  }, []);

  console.info(lastMessage);

  return (
    <div className="App">
      <span className="text-3xl font-bold underline">
        {lastMessage?.data ?? ""}
      </span>
      <button onClick={handleClickSendMessage}>Send</button>
    </div>
  );
}

export default App;
