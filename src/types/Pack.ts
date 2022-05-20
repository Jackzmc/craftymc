import { ExternalProject } from './External';
export interface Modpack {
  id: string,
  name: string,
  author?: string,
  versions: {
    minecraft: string,
    modloader: string,
    pack?: string
  },
  settings: PackSettings,
  created: number,
  lastPlayed?: number,
  mods: Record<string, ExternalProject>
}

export interface PackSettings {
  modloaderType: string,
  javaMemory?: number,
  useCustomMemory: boolean,
}

export const enum View {
  Main,
  MyPacks,
  BrowsePacks,
  Settings
}

export const enum InstallState {
  Installed,
  NotInstalled,
  Installing,
}
