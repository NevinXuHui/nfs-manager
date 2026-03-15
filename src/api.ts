import { invoke } from "@tauri-apps/api/core";

export interface NfsConfig {
  name: string;
  server: string;
  mount_point: string;
  options: string;
}

export interface MountStatus {
  name: string;
  mounted: boolean;
  mount_point: string;
}

export async function getConfigs(): Promise<NfsConfig[]> {
  return await invoke("get_configs");
}

export async function addConfig(config: NfsConfig): Promise<void> {
  return await invoke("add_config", { config });
}

export async function removeConfig(name: string): Promise<void> {
  return await invoke("remove_config", { name });
}

export async function mountNfs(name: string): Promise<void> {
  return await invoke("mount_nfs", { name });
}

export async function mountAll(): Promise<string[]> {
  return await invoke("mount_all");
}

export async function umountNfs(name: string, force: boolean = false): Promise<void> {
  return await invoke("umount_nfs", { name, force });
}

export async function umountAll(force: boolean = false): Promise<string[]> {
  return await invoke("umount_all", { force });
}

export async function getStatus(): Promise<MountStatus[]> {
  return await invoke("get_status");
}
