import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [updateTimer, setUpdateTimer] = useState<any>(null);

  async function getWindowTitles() {
  }

  useEffect(() => {
    setUpdateTimer(setInterval(async () => {
      await invoke("get_foreground_window");
    }, 1000))

    return () => {
      clearInterval(updateTimer);
    }
  })

  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>

      <div className="row">
        <div>
          <button type="button" onClick={() => getWindowTitles()}>
            Run
          </button>
        </div>
      </div>
    </div>
  );
}

export default App;
