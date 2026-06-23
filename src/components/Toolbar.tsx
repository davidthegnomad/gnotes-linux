import { Editor as TipTapEditor } from "@tiptap/react";

interface ToolbarProps {
  editor: TipTapEditor | null;
}

export default function Toolbar({ editor }: ToolbarProps) {
  if (!editor) return null;

  const btn = (action: () => void, active: boolean, label: string) => (
    <button
      type="button"
      className={`toolbar-btn ${active ? "active" : ""}`}
      onClick={action}
      title={label}
    >
      {label}
    </button>
  );

  const setFontSize = (px: string) => {
    editor.chain().focus().setFontSize(px).run();
  };

  return (
    <div className="toolbar">
      {btn(() => editor.chain().focus().toggleBold().run(), editor.isActive("bold"), "B")}
      {btn(() => editor.chain().focus().toggleItalic().run(), editor.isActive("italic"), "I")}
      {btn(() => editor.chain().focus().toggleUnderline().run(), editor.isActive("underline"), "U")}
      {btn(() => editor.chain().focus().toggleStrike().run(), editor.isActive("strike"), "S")}
      <span className="toolbar-sep" />
      {btn(() => editor.chain().focus().toggleHeading({ level: 1 }).run(), editor.isActive("heading", { level: 1 }), "H1")}
      {btn(() => editor.chain().focus().toggleHeading({ level: 2 }).run(), editor.isActive("heading", { level: 2 }), "H2")}
      {btn(() => editor.chain().focus().toggleHeading({ level: 3 }).run(), editor.isActive("heading", { level: 3 }), "H3")}
      <span className="toolbar-sep" />
      {btn(() => editor.chain().focus().toggleBulletList().run(), editor.isActive("bulletList"), "•")}
      {btn(() => editor.chain().focus().toggleOrderedList().run(), editor.isActive("orderedList"), "1.")}
      <span className="toolbar-sep" />
      <select
        className="toolbar-select"
        onChange={(e) => setFontSize(e.target.value)}
        value={editor.getAttributes("textStyle").fontSize || ""}
      >
        <option value="">Size</option>
        <option value="12px">12</option>
        <option value="14px">14</option>
        <option value="16px">16</option>
        <option value="20px">20</option>
        <option value="24px">24</option>
        <option value="32px">32</option>
      </select>
    </div>
  );
}
