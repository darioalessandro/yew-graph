const canvas = document.getElementById('imageCanvas');
const ctx = canvas.getContext('2d');

let currentSet = 'setA';
let alpha = 0;

function drawPolygon(x, y, sides, radius, rotation, alpha) {
  ctx.globalAlpha = alpha;
  ctx.beginPath();
  for (let i = 0; i < sides; i++) {
    const angle = ((2 * Math.PI) / sides) * i + rotation;
    const px = x + radius * Math.cos(angle);
    const py = y + radius * Math.sin(angle);
    if (i === 0) {
      ctx.moveTo(px, py);
    } else {
      ctx.lineTo(px, py);
    }
  }
  ctx.closePath();
  ctx.fill();
}

function drawSetA(alpha) {
  ctx.fillStyle = '#f00';
  drawPolygon(canvas.width / 3, canvas.height / 2, 3, 50, 0, alpha);
  ctx.fillStyle = '#0f0';
  drawPolygon(2 * canvas.width / 3, canvas.height / 2, 5, 50, Math.PI / 5, alpha);
}

function drawSetB(alpha) {
  ctx.fillStyle = '#00f';
  drawPolygon(canvas.width / 3, canvas.height / 2, 4, 50, Math.PI / 4, alpha);
  ctx.fillStyle = '#ff0';
  drawPolygon(2 * canvas.width / 3, canvas.height / 2, 6, 50, Math.PI / 6, alpha);
}

function animate() {
  ctx.clearRect(0, 0, canvas.width, canvas.height);

  if (currentSet === 'setA') {
    drawSetA(1 - alpha);
    drawSetB(alpha);
  } else {
    drawSetA(alpha);
    drawSetB(1 - alpha);
  }

  alpha += 0.01;

  if (alpha >= 1) {
    alpha = 0;
    currentSet = currentSet === 'setA' ? 'setB' : 'setA';
  }

  requestAnimationFrame(animate);
}

animate();
