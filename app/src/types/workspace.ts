export interface IpcErrorBody {
  code: string;
  message: string;
}

export interface IpcResponse<T> {
  success: boolean;
  data?: T;
  error?: IpcErrorBody;
}

export interface WorkspaceStatus {
  status: string;
  version: string;
  initialized: boolean;
}

export interface WorkspaceHealth {
  status: string;
  initialized: boolean;
  services: string[];
  version: string;
}

export interface WorkspaceSettings {
  theme: string;
  first_run: boolean;
  settings_version: number;
}
