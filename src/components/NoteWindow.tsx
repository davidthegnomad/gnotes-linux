import { useCallback, useEffect, useState } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useNoteStore } from "../stores/noteStore";
import type { Note } from "../stores/noteStore";
import TitleBar from "./TitleBar";
import Editor from "./Editor";

interface NoteWindowProps {
  note: Note;
  standalone?: boolean;
}

const PIN_UNSUPPORTED_MSG =
  "Pin not supported on this desktop (common on GNOME Wayland).";

export default function NoteWindow({ note, standalone: _standalone }: NoteWindowProps) {
  const updateNote = useNoteStore((s) => s.updateNote);
  const deleteNote = useNoteStore((s) => s.deleteNote);
  const [localOpacity, setLocalOpacity] = useState(note.opacity);
  const [pinWarning, setPinWarning] = useState<string | null>(null);

  useEffect(() => {
    setLocalOpacity(note.opacity);
  }, [note.id, note.opacity]);

  useEffect(() => {
    const timer = setTimeout(() => {
      if (localOpacity !== note.opacity) {
        updateNote(note.id, { opacity: localOpacity });
      }
    }, 200);
    return () => clearTimeout(timer);
  }, [localOpacity, note.id, note.opacity, updateNote]);

  const applyAlwaysOnTop = useCallback(async (enabled: boolean) => {
    try {
      await getCurrentWindow().setAlwaysOnTop(enabled);
      setPinWarning(null);
    } catch (e) {
      console.error("setAlwaysOnTop failed:", e);
      if (enabled) {
        setPinWarning(PIN_UNSUPPORTED_MSG);
      }
    }
  }, []);

  const handleDragStart = useCallback(
    async (e: React.MouseEvent) => {
      e.preventDefault();
      try {
        await getCurrentWindow().startDragging();
      } catch {
        const sx = e.screenX;
        const sy = e.screenY;
        const nx = note.position_x;
        const ny = note.position_y;
        const move = (ev: MouseEvent) =>
          updateNote(note.id, {
            position_x: nx + ev.screenX - sx,
            position_y: ny + ev.screenY - sy,
          });
        const up = () => {
          document.removeEventListener("mousemove", move);
          document.removeEventListener("mouseup", up);
        };
        document.addEventListener("mousemove", move);
        document.addEventListener("mouseup", up);
      }
    },
    [note.id, note.position_x, note.position_y, updateNote],
  );

  const handleResize = useCallback(
    (e: React.MouseEvent) => {
      e.stopPropagation();
      const sx = e.screenX;
      const sy = e.screenY;
      const sw = note.width;
      const sh = note.height;
      const move = (ev: MouseEvent) =>
        updateNote(note.id, {
          width: Math.max(200, sw + ev.screenX - sx),
          height: Math.max(100, sh + ev.screenY - sy),
        });
      const up = () => {
        document.removeEventListener("mousemove", move);
        document.removeEventListener("mouseup", up);
      };
      document.addEventListener("mousemove", move);
      document.addEventListener("mouseup", up);
    },
    [note.id, note.width, note.height, updateNote],
  );

  const toggleFloat = async () => {
    const newVal = !note.float_on_top;
    updateNote(note.id, { float_on_top: newVal });
    await applyAlwaysOnTop(newVal);
  };

  useEffect(() => {
    if (note.float_on_top) {
      applyAlwaysOnTop(true);
    }
  }, [note.id, note.float_on_top, applyAlwaysOnTop]);

  return (
    <div
      className="note-window"
      style={{
        width: "100vw",
        height: "100vh",
        backgroundColor: note.color,
        opacity: note.collapsed ? 1 : localOpacity,
      }}
    >
      {pinWarning && (
        <div className="pin-warning" role="status">
          {pinWarning}
          <button type="button" onClick={() => setPinWarning(null)} aria-label="Dismiss">
            ×
          </button>
        </div>
      )}
      <TitleBar
        title={note.title}
        color={note.color}
        locked={note.locked}
        collapsed={note.collapsed}
        floatOnTop={note.float_on_top}
        onTitleChange={(t) => updateNote(note.id, { title: t })}
        onColorChange={(c) => updateNote(note.id, { color: c })}
        onToggleLock={() => updateNote(note.id, { locked: !note.locked })}
        onToggleCollapse={() =>
          updateNote(note.id, { collapsed: !note.collapsed })
        }
        onToggleFloat={toggleFloat}
        onClose={() => deleteNote(note.id)}
        onDragStart={handleDragStart}
      />
      {!note.collapsed && (
        <>
          <div className="note-body">
            <Editor
              content={note.content}
              editable={!note.locked}
              onChange={(html, text) =>
                updateNote(note.id, { content: html, plain_text: text })
              }
            />
          </div>
          <div className="opacity-bar">
            <input
              type="range"
              min="0.2"
              max="1"
              step="0.05"
              value={localOpacity}
              onChange={(e) => setLocalOpacity(parseFloat(e.target.value))}
              className="opacity-slider"
              title="Opacity"
              aria-label="Note opacity"
              onMouseDown={(e) => e.stopPropagation()}
            />
          </div>
          <div className="resize-handle" onMouseDown={handleResize} />
        </>
      )}
    </div>
  );
}
