import { useLocalStorage } from "./use-storage";

export interface Config {
  mac_window_bar: boolean;
  opacity: boolean;
  watermark: string;
  auto_load: boolean;
  preview_title: string;
  watermark_font_family: string;
  editor_font_family: string;
  highlight_theme: string;
}

const CONFIG_STORAGE_KEY = "CONFIG_STORAGE_KEY";

export const useConfig = (defaultConfig?: Config) => {
  const [config] = useLocalStorage(CONFIG_STORAGE_KEY, defaultConfig);

  return config;
};
