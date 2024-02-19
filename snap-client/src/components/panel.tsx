import { PropsWithChildren } from "react";

export interface PanelProps {}

export const Panel = ({ children }: PropsWithChildren<PanelProps>) => (
  <div className="p-10 br-10">{children}</div>
);
