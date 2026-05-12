import gsap from "gsap";
import { useGSAP } from "@gsap/react";
import ScrollTrigger from "gsap/ScrollTrigger";
import { type RefObject, useRef } from "react";
import { renderFrame, setupCanvas } from "./canvasUtils";

if (typeof window !== "undefined") {
  gsap.registerPlugin(ScrollTrigger, useGSAP);
}

interface UseScrollSequenceProps {
  canvasRef: RefObject<HTMLCanvasElement | null>;
  containerRef: RefObject<HTMLDivElement | null>;
  images: HTMLImageElement[];
  frameCount: number;
  enabled: boolean;
}

export const useScrollSequence = ({
  canvasRef,
  containerRef,
  images,
  frameCount,
  enabled,
}: UseScrollSequenceProps) => {
  const currentFrameRef = useRef(0);

  useGSAP(
    () => {
      if (
        !enabled ||
        !canvasRef.current ||
        !containerRef.current ||
        images.length === 0
      ) {
        return;
      }

      const canvas = canvasRef.current;

      const ctx = setupCanvas(canvas);

      if (!ctx) {
        return;
      }

      // lightweight draw function
      const draw = (frame: number) => {
        renderFrame(ctx, canvas, images[frame]);
      };

      // initial frame
      draw(0);

      // resize only when actual resize happens
      const handleResize = () => {
        setupCanvas(canvas);
        draw(currentFrameRef.current);
        ScrollTrigger.refresh();
      };

      window.addEventListener("resize", handleResize, {
        passive: true,
      });

      const sequence = {
        frame: 0,
      };

      const tween = gsap.to(sequence, {
        frame: frameCount - 1,

        ease: "none",

        snap: "frame",

        scrollTrigger: {
          trigger: containerRef.current,

          start: "top top",

          end: "+=260%",

          pin: true,

          scrub: 1.2,

          anticipatePin: 1,

          invalidateOnRefresh: true,
        },

        onUpdate: () => {
          const nextFrame = Math.min(
            frameCount - 1,
            Math.max(0, Math.round(sequence.frame))
          );

          // avoid unnecessary redraws
          if (
            nextFrame !== currentFrameRef.current &&
            images[nextFrame]
          ) {
            currentFrameRef.current = nextFrame;

            draw(nextFrame);
          }
        },
      });

      return () => {
        window.removeEventListener("resize", handleResize);

        tween.scrollTrigger?.kill();

        tween.kill();
      };
    },
    {
      dependencies: [enabled, images, frameCount],
      scope: containerRef,
    }
  );

  const renderStaticFrame = () => {
    if (!canvasRef.current || images.length === 0) {
      return;
    }

    const canvas = canvasRef.current;

    const ctx = setupCanvas(canvas);

    if (!ctx) {
      return;
    }

    renderFrame(ctx, canvas, images[0]);
  };

  return {
    renderStaticFrame,
  };
};