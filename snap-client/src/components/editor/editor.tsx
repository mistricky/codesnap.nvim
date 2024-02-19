import { MacStyleTitleBar } from "./mac-style-titlebar";
import hljs from "highlight.js";

export interface EditorProps {
  macStyleTitleBar?: boolean;
  language?: string;
  opacity?: boolean;
  children: string;
}

const highlightLanguage = (code: string, language?: string) => {
  if (!language) {
    return hljs.highlightAuto(code).value;
  }

  try {
    return hljs.highlight(code, { language }).value;
  } catch {
    return hljs.highlightAuto(code).value;
  }
};

export const Editor = ({
  children,
  language,
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
        dangerouslySetInnerHTML={{
          __html: highlightLanguage(children, language),
        }}
      />
    </pre>
  </div>
);
