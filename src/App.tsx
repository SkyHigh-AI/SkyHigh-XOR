import { invoke } from "@tauri-apps/api";

export default function App(){
  return(
    <>
    <span>Hey</span>
    <button onClick={() => invoke('train_network')}>Click me!</button>
    </>
  )
}