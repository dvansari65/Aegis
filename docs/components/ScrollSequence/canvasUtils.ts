export const setupCanvas = (
  canvas: HTMLCanvasElement
): CanvasRenderingContext2D | null => {
  const pixelRatio = Math.min(window.devicePixelRatio || 1, 2);

  const width = window.innerWidth;
  const height = window.innerHeight;

  canvas.width = width * pixelRatio;
  canvas.height = height * pixelRatio;

  canvas.style.width = `${width}px`;
  canvas.style.height = `${height}px`;

  const ctx = canvas.getContext("2d", {
    alpha: true,
  });

  if (!ctx) {
    return null;
  }

  // reset transform before scaling
  ctx.setTransform(1, 0, 0, 1, 0, 0);

  // retina scaling
  ctx.scale(pixelRatio, pixelRatio);

  return ctx;
};

export const renderFrame = (
  ctx: CanvasRenderingContext2D,
  canvas: HTMLCanvasElement,
  image?: HTMLImageElement
) => {
  if (!image) return;

  const canvasWidth = window.innerWidth;
  const canvasHeight = window.innerHeight;

  ctx.clearRect(0, 0, canvasWidth, canvasHeight);

  // smaller render size
  const renderWidth = image.width * 0.55;
  const renderHeight = image.height * 0.55;

  // centered position
  const x =
    (canvasWidth - renderWidth) / 2;

  const y =
    (canvasHeight - renderHeight) / 2;

  ctx.drawImage(
    image,
    x,
    y,
    renderWidth,
    renderHeight
  );
};