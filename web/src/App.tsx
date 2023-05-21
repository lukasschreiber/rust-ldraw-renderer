import { useCallback, useContext, useEffect, useState } from "react"
import { RenderingContext } from "./context"

const BRICKS = ["3001", "3002", "3005"]

function App() {
  const rendering = useContext(RenderingContext)
  const [red, setRed] = useState(0);
  const [brick, setBrick] = useState("3001");

  const redChanged = useCallback((val: number) => {
    setRed(val);
  }, []);

  const brickChanged = useCallback((val: string) => {
    setBrick(val);
  }, []);


  useEffect(() => {
    rendering.update_prop(red)
  }, [red])

  useEffect(() => {
    const windowRef = rendering.create_window("canvas1", brick);
    return () => {
      rendering.delete_window(0) // use windowref but async shit
    }
  }, [brick])

  return (
    <div className="App">
      <canvas id="canvas1" style={{ display: "block", width: "100%", height: "50%" }}></canvas>
      <input type="range" min="0" max="255" className="slider" id="red" value={red} onChange={e => redChanged(parseInt(e.target.value))} />
      <select onChange={e => brickChanged(e.target.value)}>
        {BRICKS.map(brick =>
          <option value={brick}>{brick}</option>
        )}
      </select>
    </div>
  )
}

export default App
