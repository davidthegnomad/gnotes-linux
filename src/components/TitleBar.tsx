import { useState } from "react";

interface TitleBarProps {
  title: string;
  color: string;
  locked: boolean;
  collapsed: boolean;
  floatOnTop: boolean;
  onTitleChange: (title: string) => void;
  onColorChange: (color: string) => void;
  onToggleLock: () => void;
  onToggleCollapse: () => void;
  onToggleFloat: () => void;
  onClose: () => void;
  onDragStart: (e: React.MouseEvent) => void;
}

const PRESET_COLORS = [
  "#ffeb3b", "#a5d6ff", "#a5d6a5",
  "#f8bbd0", "#ce93d8", "#bdbdbd",
];

export default function TitleBar({
  title,
  color,
  locked,
  collapsed,
  floatOnTop,
  onTitleChange,
  onColorChange,
  onToggleLock,
  onToggleCollapse,
  onToggleFloat,
  onClose,
  onDragStart,
}: TitleBarProps) {
  const [editing, setEditing] = useState(false);
  const [draft, setDraft] = useState(title);

  const commit = () => {
    setEditing(false);
    if (draft !== title) onTitleChange(draft || "Untitled");
  };

  const stop = (e: React.MouseEvent) => e.stopPropagation();

  return (
    <div className="titlebar" onMouseDown={onDragStart}>
      <button
        className="titlebar-btn collapse-btn"
        onClick={(e) => { stop(e); onToggleCollapse(); }}
        title={collapsed ? "Expand" : "Collapse"}
      >
        {collapsed ? "▸" : "▾"}
      </button>

      {editing ? (
        <input
          className="titlebar-input"
          value={draft}
          onChange={(e) => setDraft(e.target.value)}
          onBlur={commit}
          onKeyDown={(e) => e.key === "Enter" && commit()}
          autoFocus
          onMouseDown={stop}
        />
      ) : (
        <span
          className="titlebar-text"
          onDoubleClick={(e) => {
            stop(e);
            setDraft(title);
            setEditing(true);
          }}
        >
          {title || "Untitled"}
        </span>
      )}

      <div className="titlebar-colors">
        {PRESET_COLORS.map((c) => (
          <span
            key={c}
            className={`color-dot ${color === c ? "active" : ""}`}
            style={{ background: c }}
            onClick={(e) => { stop(e); onColorChange(c); }}
          />
        ))}
        <input
          type="color"
          className="color-input"
          value={color}
          onChange={(e) => onColorChange(e.target.value)}
          onClick={stop}
          onMouseDown={stop}
        />
      </div>

      <div className="titlebar-actions">
        <button
          className={`titlebar-btn ${floatOnTop ? "active" : ""}`}
          onClick={(e) => { stop(e); onToggleFloat(); }}
          title={floatOnTop ? "Unpin" : "Float on top"}
        >
          📌
        </button>
        <button
          className={`titlebar-btn ${locked ? "active" : ""}`}
          onClick={(e) => { stop(e); onToggleLock(); }}
          title={locked ? "Unlock" : "Lock"}
        >
          {locked ? "🔒" : "🔓"}
        </button>
        <button
          className="titlebar-btn"
          onClick={(e) => { stop(e); onClose(); }}
          title="Close"
        >
          ×
        </button>
      </div>
    </div>
  );
}
