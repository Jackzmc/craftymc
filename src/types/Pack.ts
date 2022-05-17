export interface Modpack {
  id: string,
  name: string,
  versions: {
    minecraft: string,
    forge: string,
    pack: string
  }
}

export enum View {
  Main,
  MyPacks,
  BrowsePacks,
  Settings
}
