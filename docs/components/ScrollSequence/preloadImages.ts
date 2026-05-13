export type FramePathOptions = {
  basePath?: string;
  extension?: string;
};

export type PreloadImagesOptions = FramePathOptions & {
  frameCount: number;
  onFirstFrame?: (image: HTMLImageElement) => void;
  onProgress?: (progress: number) => void;
};

export const getFramePath = (
  index: number,
  {
    basePath = "public/oracle-risk-animation-frames",
   extension = "jpg"
  }: FramePathOptions = {}
) => {
  const paddedIndex = index
    .toString()
    .padStart(3, "0");

  return `${basePath}/ezgif-frame-${paddedIndex}.${extension}`;
};

const loadImage = (
  src: string
): Promise<HTMLImageElement> => {
  return new Promise((resolve, reject) => {
    const image = new Image();

    // browser async decode
    image.decoding = "async";

    // browser hint
    image.loading = "eager";

    image.src = src;

    image.onload = async () => {
      try {
        // decode before render
        await image.decode();
      } catch {
        // ignore decode failures
      }

      resolve(image);
    };

    image.onerror = () => {
      reject(
        new Error(`Failed to load image: ${src}`)
      );
    };
  });
};

export const preloadImages = async ({
  frameCount,
  onFirstFrame,
  onProgress,
  ...pathOptions
}: PreloadImagesOptions): Promise<
  HTMLImageElement[]
> => {
  const images =
    new Array<HTMLImageElement>(frameCount);

  let loadedCount = 0;

  const markLoaded = () => {
    loadedCount += 1;

    onProgress?.(loadedCount / frameCount);
  };

  // -----------------------------------
  // LOAD FIRST FRAME IMMEDIATELY
  // -----------------------------------

  const firstFrame = await loadImage(
    getFramePath(1, pathOptions)
  );

  images[0] = firstFrame;

  markLoaded();

  onFirstFrame?.(firstFrame);

  // -----------------------------------
  // LOAD INITIAL CRITICAL FRAMES
  // -----------------------------------

  const INITIAL_LOAD_COUNT = 20;

  await Promise.all(
    Array.from(
      {
        length: INITIAL_LOAD_COUNT - 1,
      },
      async (_, offset) => {
        const frameIndex = offset + 2;

        try {
          const image = await loadImage(
            getFramePath(frameIndex, pathOptions)
          );

          images[frameIndex - 1] = image;
        } catch (error) {
          console.error(error);
        } finally {
          markLoaded();
        }
      }
    )
  );

  // -----------------------------------
  // LOAD REMAINING FRAMES IN BACKGROUND
  // -----------------------------------

  const loadRemainingFrames = async () => {
    await Promise.all(
      Array.from(
        {
          length:
            frameCount -
            INITIAL_LOAD_COUNT,
        },
        async (_, offset) => {
          const frameIndex =
            offset +
            INITIAL_LOAD_COUNT +
            1;

          try {
            const image = await loadImage(
              getFramePath(frameIndex, pathOptions)
            );

            images[frameIndex - 1] = image;
          } catch (error) {
            console.error(error);
          } finally {
            markLoaded();
          }
        }
      )
    );
  };

  // use idle time to avoid scroll jank
  if ("requestIdleCallback" in window) {
    requestIdleCallback(() => {
      loadRemainingFrames();
    });
  } else {
    setTimeout(() => {
      loadRemainingFrames();
    }, 0);
  }

  return images;
};