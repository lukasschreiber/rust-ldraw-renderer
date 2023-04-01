import { useCallback, useContext, useEffect, useState } from "react"
import { RenderingContext } from "./context"

function App() {
  const rendering = useContext(RenderingContext)
  const [red, setRed] = useState(0);

  const redChanged = useCallback((val: number) => {
    setRed(val);
  }, []);

  useEffect(() => {
    rendering.update_prop(red)
  }, [red])

  useEffect(() => {
    const windowRef = rendering.create_window("canvas1");
    return () => {
      rendering.delete_window(windowRef)
    }
  }, [])

  return (
    <div className="App">
      <canvas id="canvas1" style={{ display: "block", width: "100%", height: "50%" }}></canvas>
      <input type="range" min="0" max="255" className="slider" id="red" value={red} onChange={e => redChanged(parseInt(e.target.value))} />
    </div>
  )
}

export default App
