const canvas = document.getElementById('imageCanvas');
const ctx = canvas.getContext('2d');

let currentShape = 'rectangle';
let alpha = 0;

function drawRectangle(alpha) {
  const width = 200;
  const height = 100;
  const x = (canvas.width - width) / 2;
  const y = (canvas.height - height) / 2;

  ctx.globalAlpha = alpha;
  ctx.fillStyle = '#f00';
  ctx.fillRect(x, y, width, height);
}

function drawCircle(alpha) {
  const radius = 50;
  const x = canvas.width / 2;
  const y = canvas.height / 2;

  ctx.globalAlpha = alpha;
  ctx.fillStyle = '#00f';
  ctx.beginPath();
  ctx.arc(x, y, radius, 0, 2 * Math.PI);
  ctx.closePath();
  ctx.fill();
}

function animate() {
  ctx.clearRect(0, 0, canvas.width, canvas.height);

  if (currentShape === 'rectangle') {
    drawRectangle(1 - alpha);
    drawCircle(alpha);
  } else {
    drawRectangle(alpha);
    drawCircle(1 - alpha);
  }

  alpha += 0.01;

  if (alpha >= 1) {
    alpha = 0;
    currentShape = currentShape === 'rectangle' ? 'circle' : 'rectangle';
  }

  requestAnimationFrame(animate);
}

animate();
