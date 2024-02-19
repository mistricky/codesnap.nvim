import { useCallback, useMemo } from "react";

const createStorageHook =
  (storage: Storage) =>
  <T>(
    key: string,
    defaultData?: T,
  ): [NonNullable<T> | null, (value: T) => void] => {
    const setValue = useCallback(
      (value: T) => {
        storage.setItem(key, JSON.stringify(value));
      },
      [key],
    );

    const value = useMemo(() => {
      const value = storage.getItem(key);

      if (defaultData) {
        setValue(defaultData);

        return defaultData;
      }

      return value ? JSON.parse(value) : value;
    }, [key, defaultData, setValue]);

    return [value, setValue];
  };

export const useLocalStorage = createStorageHook(localStorage);

export const useSessionStorage = createStorageHook(sessionStorage);
