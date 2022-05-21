export interface ModrinthModpack extends ModrinthProject {
  project_id: string,
  project_type: "modpack"
}

export interface ModrinthProject {
  slug: string
  project_type: string
  author_id?: string,
  team?: string
  title: string
  description: string
  body: string
  body_url: string
  published: string
  updated: string
  status: string
  moderator_message: any
  license: {
    id: string
    platform: string
    url: string
  }
  client_side: string
  server_side: string
  downloads: number
  followers: number
  categories: string[]
  versions: string[]
  icon_url: string
  issues_url: string
  source_url: string
  wiki_url: any
  discord_url: string
  donation_urls: {
    id: string
    name: string
    url: string
  }
  gallery: any[]
}

// Project refers to a mod or a modpack
export type ExternalProject = ModrinthProject

export interface Entry {
  project: ExternalProject,
  installing: boolean
}

export interface ModrinthProjectVersion {
  name: string;
  version_number: string;
  changelog: string;
  dependencies: any[];
  game_versions: any[];
  version_type: string;
  loaders: any[];
  featured: boolean;
  id: string;
  project_id: string;
  author_id: string;
  date_published: Date;
  downloads: number;
  changelog_url?: any;
  files: any[];
}
