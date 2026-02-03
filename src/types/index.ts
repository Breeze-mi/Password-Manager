// Entry types
export interface Entry {
  id: string;
  groupId: string | null;
  title: string;
  url: string;
  username: string;
  password: string;
  notes: string;
  isFavorite: boolean;
  sortOrder: number;
  createdAt: number;
  updatedAt: number;
}

export interface CreateEntryDto {
  groupId?: string | null;
  title: string;
  url?: string;
  username?: string;
  password?: string;
  notes?: string;
}

export interface UpdateEntryDto {
  groupId?: string | null;
  title?: string;
  url?: string;
  username?: string;
  password?: string;
  notes?: string;
  isFavorite?: boolean;
  sortOrder?: number;
}

// Group types
export interface Group {
  id: string;
  name: string;
  icon: string;
  sortOrder: number;
  createdAt: number;
  updatedAt: number;
}

export interface CreateGroupDto {
  name: string;
  icon?: string;
}

export interface UpdateGroupDto {
  name?: string;
  icon?: string;
  sortOrder?: number;
}

// Settings types
export interface Settings {
  autoLockMinutes: number;
  clearClipboardSeconds: number;
  theme: "light" | "dark" | "system";
}

// App state types
export type ViewMode = "all" | "favorites" | "group";

export interface AppState {
  isLocked: boolean;
  isInitialized: boolean;
  currentView: ViewMode;
  currentGroupId: string | null;
  searchKeyword: string;
}

// Toast types
export type ToastType = "success" | "error" | "info" | "warning";

export interface Toast {
  id: string;
  type: ToastType;
  message: string;
  duration?: number;
}
