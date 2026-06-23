import { useEffect, useState } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useNoteStore } from "./stores/noteStore";
import NoteWindow from "./components/NoteWindow";
import "./App.css";

function ErrorPanel({
  message,
  onRetry,
  onDismiss,
}: {
  message: string;
  onRetry: () => void;
  onDismiss: () => void;
}) {
  return (
    <div className="error-panel" role="alert">
      <p>{message}</p>
      <div className="error-actions">
        <button type="button" className="new-note-btn" onClick={onRetry}>
          Retry
        </button>
        <button type="button" className="error-dismiss" onClick={onDismiss}>
          Dismiss
        </button>
      </div>
    </div>
  );
}

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

  if (store.error) {
    return (
      <div className="desktop">
        <ErrorPanel
          message={store.error}
          onRetry={() => {
            if (mode === "main") store.loadNotes();
            else store.loadMyNote();
          }}
          onDismiss={store.clearError}
        />
      </div>
    );
  }

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
