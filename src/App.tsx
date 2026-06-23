import { useEffect, useState } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useNoteStore } from "./stores/noteStore";
import NoteWindow from "./components/NoteWindow";
import "./App.css";

export default function App() {
  const store = useNoteStore();
  const [mode, setMode] = useState<"loading" | "main" | "note">("loading");

  useEffect(() => {
    (async () => {
      const label = getCurrentWindow().label;
      if (label === "main") {
        await store.loadNotes();
        setMode("main");
      } else {
        await store.loadMyNote();
        setMode("note");
      }
    })();
  }, []);

  if (mode === "loading") return null;

  if (mode === "note") {
    const note = store.notes[0];
    if (!note) return null;
    return <NoteWindow note={note} standalone />;
  }

  return (
    <div className="desktop">
      <div className="controller">
        <h1>gnotes</h1>
        <p>{store.notes.length} note{store.notes.length !== 1 ? "s" : ""}</p>
        <button className="new-note-btn" onClick={() => store.createNote()}>
          + New Note
        </button>
      </div>
    </div>
  );
}
