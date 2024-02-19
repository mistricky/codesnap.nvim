import { forwardRef, PropsWithChildren } from "react";

export interface FrameProps {
  watermark?: string;
}

export const Frame = forwardRef<HTMLDivElement, PropsWithChildren<FrameProps>>(
  ({ children, watermark }, ref) => (
    <div ref={ref} className="bg-stripe min-w-[800px] p-20">
      {children}
      {watermark && (
        <p className="pacifico-regular text-xl opacity-50 font-bold text-white text-center w-full mt-14">
          {watermark}
        </p>
      )}
    </div>
  ),
);
