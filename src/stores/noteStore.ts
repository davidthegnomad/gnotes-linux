import { create } from "zustand";
import { invoke } from "@tauri-apps/api/core";

export interface Note {
  id: string;
  title: string;
  content: string;
  plain_text: string;
  color: string;
  position_x: number;
  position_y: number;
  width: number;
  height: number;
  collapsed: boolean;
  float_on_top: boolean;
  opacity: number;
  locked: boolean;
  shape_id: string | null;
  bg_config: string | null;
  font_config: string | null;
  sort_order: number;
  created_at: string;
  updated_at: string;
}

interface NoteStore {
  notes: Note[];
  loading: boolean;
  loadNotes: () => Promise<void>;
  loadMyNote: () => Promise<void>;
  createNote: () => Promise<Note | null>;
  updateNote: (id: string, data: Partial<Note>) => Promise<void>;
  deleteNote: (id: string) => Promise<void>;
}

export const useNoteStore = create<NoteStore>((set, get) => ({
  notes: [],
  loading: true,

  loadNotes: async () => {
    try {
      const notes = await invoke<Note[]>("get_all_notes");
      set({ notes, loading: false });
    } catch (e) {
      console.error("Failed to load notes:", e);
      set({ loading: false });
    }
  },

  loadMyNote: async () => {
    try {
      const note = await invoke<Note>("get_my_note");
      set({ notes: [note], loading: false });
    } catch (e) {
      console.error("Failed to load note:", e);
      set({ loading: false });
    }
  },

  createNote: async () => {
    try {
      const note = await invoke<Note>("create_note");
      set({ notes: [note, ...get().notes] });
      return note;
    } catch (e) {
      console.error("Failed to create note:", e);
      return null;
    }
  },

  updateNote: async (id, data) => {
    const current = get().notes.find((n) => n.id === id);
    if (!current) return;
    const merged = { ...current, ...data };
    const prev = get().notes; // snapshot for rollback

    set({ notes: get().notes.map((n) => (n.id === id ? merged : n)) });

    try {
      await invoke("update_note", {
        id,
        title: merged.title,
        content: merged.content,
        plainText: merged.plain_text,
        color: merged.color,
        positionX: merged.position_x,
        positionY: merged.position_y,
        width: merged.width,
        height: merged.height,
        collapsed: merged.collapsed,
        floatOnTop: merged.float_on_top,
        opacity: merged.opacity,
        locked: merged.locked,
      });
    } catch (e) {
      console.error("Failed to update note, rolling back:", e);
      set({ notes: prev });
    }
  },

  deleteNote: async (id) => {
    try {
      await invoke("delete_note", { id });
      set({ notes: get().notes.filter((n) => n.id !== id) });
    } catch (e) {
      console.error("Failed to delete note:", e);
    }
  },
}));
