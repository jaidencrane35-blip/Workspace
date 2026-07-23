export interface WorkspaceStatus {
  status: string;
  version: string;
  initialization: string;
}

export interface WorkspaceSettings {
  theme: string;
  first_run: boolean;
  settings_version: number;
}

export interface CommandError {
  code: string;
  message: string;
}
