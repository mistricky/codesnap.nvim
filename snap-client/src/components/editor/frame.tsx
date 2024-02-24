import { forwardRef, PropsWithChildren } from "react";

const DEFAULT_WATERMARK_FONT_FAMILY = "Pacifico";

export interface FrameProps {
  watermark?: string;
  watermarkFontFamily?: string;
}

export const Frame = forwardRef<HTMLDivElement, PropsWithChildren<FrameProps>>(
  ({ children, watermark, watermarkFontFamily }, ref) => (
    <div ref={ref} className="bg-relay min-w-[800px] p-20">
      {children}
      {watermark && (
        <p
          style={{
            fontFamily: watermarkFontFamily ?? DEFAULT_WATERMARK_FONT_FAMILY,
          }}
          className="text-xl opacity-50 font-bold text-white text-center w-full mt-14"
        >
          {watermark}
        </p>
      )}
    </div>
  ),
);
