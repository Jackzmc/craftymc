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
  mods: Record<string, SavedModEntry>
}

export interface SavedModEntry {
  name: string,
  version_id: string,
  filenames: string[]
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
