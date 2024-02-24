import { MacStyleTitleBar } from "./mac-style-titlebar";
import hljs from "highlight.js";

export interface EditorProps {
  macStyleTitleBar?: boolean;
  language?: string;
  opacity?: boolean;
  codeFontFamily?: string;
  children: string;
}

const DEFAULT_CODE_FONT_FAMILY = "CaskaydiaCove Nerd Font";
const LSP_LANGUAGE_NAME_MAP: Record<string, string> = {
  typescriptreact: "tsx",
  javascriptreact: "jsx",
};

const highlightLanguage = (code: string, language?: string) => {
  if (!language) {
    return hljs.highlightAuto(code).value;
  }

  try {
    return hljs.highlight(code, {
      language: LSP_LANGUAGE_NAME_MAP[language] ?? language,
    }).value;
  } catch {
    return hljs.highlightAuto(code).value;
  }
};

export const Editor = ({
  children,
  language,
  codeFontFamily,
  opacity = true,
  macStyleTitleBar = true,
}: EditorProps) => (
  <div
    className={`editor-shadow ${opacity ? "bg-one-dark-base/[.93]" : "bg-one-dark-base"} rounded-2xl p-5 pb-7 border-border-color border-[1px]`}
  >
    {macStyleTitleBar && <MacStyleTitleBar />}
    <pre className="mt-4">
      <code
        className="code"
        style={{ fontFamily: codeFontFamily ?? DEFAULT_CODE_FONT_FAMILY }}
        dangerouslySetInnerHTML={{
          __html: highlightLanguage(children, language),
        }}
      />
    </pre>
  </div>
);
