import { invoke } from "@tauri-apps/api/core";

export const useTauriInvoke = async <T = any>(command: string, args?: any) => {
  return await useAsyncData<T>(`tauri-${command}`, () =>
    invoke<T>(command, args),
  );
};
