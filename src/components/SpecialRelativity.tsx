import React, {forwardRef, useEffect, useRef,} from "react";
import init, {GolemBackend,} from "@crate/special-relativity-rs/pkg";

export type Handler = {
};

export type SpecialRelativityProps = {};

export const SpecialRelativity = forwardRef<Handler, SpecialRelativityProps>((_, ref) => {
  const wrapper = useRef<HTMLDivElement>(null);
  const canvas = useRef<HTMLCanvasElement>(null);
  useEffect(() => {
    const webgl = canvas.current!.getContext("webgl")!;
    let isUnmounted = false;
    init().then(() => {
      if (isUnmounted) {
        return;
      }
      const backend = new GolemBackend(webgl);
      const loop = () => {
        if (isUnmounted) {
          backend.free();
          return;
        }
        canvas.current!.width = 800;
        canvas.current!.height = 600;
        backend.render();
        requestAnimationFrame(loop);
      };
      requestAnimationFrame(loop);
    });
  }, []);
  return (
    <div ref={wrapper}>
      <canvas ref={canvas} tabIndex={0}/>
    </div>
  );
});
SpecialRelativity.displayName = "SpecialRelativity";
