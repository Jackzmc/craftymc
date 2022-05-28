export interface Modpack {
  folder_name: string,
  img_ext?: string,

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
  timesPlayed?: number,
  mods: Record<string, SavedModEntry>
}

export interface SavedModEntry {
  name: string,
  version_id: string,
  filenames: string[]
}

export interface PackSettings {
  modloaderType: string,
  javaMemory: number,
  useCustomMemory: boolean,
  modSource: string
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
