const canvas = document.getElementById('imageCanvas');
const ctx = canvas.getContext('2d');

const imageA = new Image();
const imageB = new Image();

imageA.src = 'image_a.png';
imageB.src = 'image_b.jpg';

let currentImage = 'A';
let alpha = 0;

function drawImage(image, alpha) {
  ctx.globalAlpha = alpha;
  ctx.drawImage(image, 0, 0, canvas.width, canvas.height);
}

function animate() {
  ctx.clearRect(0, 0, canvas.width, canvas.height);

  if (currentImage === 'A') {
    drawImage(imageA, 1 - alpha);
    drawImage(imageB, alpha);
  } else {
    drawImage(imageA, alpha);
    drawImage(imageB, 1 - alpha);
  }

  alpha += 0.01;
  console.log("alpha: " + alpha);
  console.log("currentImage: " + currentImage);
  if (alpha >= 1) {
    alpha = 0;
    currentImage = currentImage === 'A' ? 'B' : 'A';
  }

  requestAnimationFrame(animate);
}

imageA.onload = () => {
  if (imageB.complete) {
    animate();
  }
};

imageB.onload = () => {
  if (imageA.complete) {
    animate();
  }
};
