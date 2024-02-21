import { useMemo } from "react";
import { Config } from "./use-config";

export enum EventType {
  CONFIG_SETUP = "config_setup",
  CODE = "code",
  COPY = "copy",
}

type CodeMessage = {
  content: string;
  language: string;
};

type ParsedConfig = {
  [EventType.CODE]: CodeMessage;
  [EventType.CONFIG_SETUP]: Config;
  [EventType.COPY]: undefined;
};

export const useEvent = (
  event: MessageEvent<string> | null,
): Partial<ParsedConfig> | undefined =>
  useMemo(() => {
    if (!event) {
      return undefined;
    }

    const parsedEvent = JSON.parse(event.data);

    return {
      [parsedEvent.name]: parsedEvent.data,
    };
  }, [event]);
