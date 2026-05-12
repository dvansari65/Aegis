"use client";

import { type ReactNode, useEffect, useRef, useState } from "react";
import { preloadImages } from "./preloadImages";
import { useScrollSequence } from "./useScrollSequence";

interface ScrollSequenceProps {
  frameCount: number;
  children?: ReactNode;
  className?: string;
  contentClassName?: string;
  frameBasePath?: string;
}

export default function ScrollSequence({
  frameCount,
  children,
  className = "",
  contentClassName = "",
  frameBasePath = "/oracle-risk-animation-frames",
}: ScrollSequenceProps) {
  const containerRef = useRef<HTMLDivElement>(null);

  const canvasRef = useRef<HTMLCanvasElement>(null);

  const [images, setImages] = useState<
    HTMLImageElement[]
  >([]);

  const [isLoaded, setIsLoaded] =
    useState(false);

  const [prefersReducedMotion, setPrefersReducedMotion] =
    useState(() =>
      typeof window === "undefined"
        ? false
        : window.matchMedia(
            "(prefers-reduced-motion: reduce)"
          ).matches
    );

  // -----------------------------------
  // REDUCED MOTION
  // -----------------------------------

  useEffect(() => {
    const mediaQuery = window.matchMedia(
      "(prefers-reduced-motion: reduce)"
    );

    const handleMotionChange = (
      e: MediaQueryListEvent
    ) => {
      setPrefersReducedMotion(e.matches);
    };

    mediaQuery.addEventListener(
      "change",
      handleMotionChange
    );

    return () => {
      mediaQuery.removeEventListener(
        "change",
        handleMotionChange
      );
    };
  }, []);

  // -----------------------------------
  // IMAGE PRELOAD
  // -----------------------------------

  useEffect(() => {
    if (prefersReducedMotion) return;

    let isMounted = true;

    preloadImages({
      frameCount,

      basePath: frameBasePath,

      onFirstFrame: (image) => {
        if (!isMounted) return;

        setImages([image]);
      },
    })
      .then((loadedImages) => {
        if (!isMounted) return;

        setImages(loadedImages);

        setIsLoaded(true);
      })
      .catch((error) => {
        console.error(error);
      });

    return () => {
      isMounted = false;
    };
  }, [
    frameCount,
    frameBasePath,
    prefersReducedMotion,
  ]);

  // -----------------------------------
  // SCROLL SEQUENCE
  // -----------------------------------

  const { renderStaticFrame } =
    useScrollSequence({
      canvasRef,

      containerRef,

      images,

      frameCount,

      enabled:
        isLoaded &&
        !prefersReducedMotion,
    });

  // -----------------------------------
  // STATIC FALLBACK
  // -----------------------------------

  useEffect(() => {
    if (
      prefersReducedMotion &&
      images[0]
    ) {
      renderStaticFrame();
    }
  }, [
    images,
    prefersReducedMotion,
    renderStaticFrame,
  ]);

  // -----------------------------------
  // RENDER
  // -----------------------------------

  return (
    <div
      ref={containerRef}
      className={`relative isolate min-h-screen overflow-hidden bg-[#05090c] ${className}`}
    >
      {/* CANVAS */}

      <canvas
        ref={canvasRef}
        aria-hidden="true"
        className="absolute inset-0 h-full w-full pointer-events-none will-change-transform"
        style={{
          opacity:
            isLoaded || images[0]
              ? 1
              : 0,
        }}
      />

      {/* CONTENT */}

      <div
        className={`relative z-20 flex min-h-screen items-center ${contentClassName}`}
      >
        {children}
      </div>
    </div>
  );
}