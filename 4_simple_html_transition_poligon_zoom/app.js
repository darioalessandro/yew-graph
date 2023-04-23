const canvas = document.getElementById('imageCanvas');
const ctx = canvas.getContext('2d');
const zoomButton = document.getElementById('zoomButton');

let zoomedIn = false;
let zoomLevel = 1;
const zoomSpeed = 0.05;

function drawCircle(alpha) {
  ctx.globalAlpha = alpha;
  ctx.fillStyle = '#f00';
  ctx.beginPath();
  ctx.arc(canvas.width / 2, canvas.height / 2, 100, 0, 2 * Math.PI);
  ctx.closePath();
  ctx.fill();
}

function drawTriangle(alpha) {
  ctx.globalAlpha = alpha;
  ctx.fillStyle = '#00f';
  ctx.beginPath();
  ctx.moveTo(canvas.width / 2, canvas.height / 2 - 50);
  ctx.lineTo(canvas.width / 2 - 50, canvas.height / 2 + 50);
  ctx.lineTo(canvas.width / 2 + 50, canvas.height / 2 + 50);
  ctx.closePath();
  ctx.fill();
}


function draw() {
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  ctx.save();
  ctx.translate(canvas.width / 2, canvas.height / 2);
  ctx.scale(zoomLevel, zoomLevel);
  ctx.translate(-canvas.width / 2, -canvas.height / 2);

  const circleAlpha = 1 - Math.min(1, Math.max(0, zoomLevel - 1));
  const triangleAlpha = Math.min(1, Math.max(0, zoomLevel - 1));

  if (zoomedIn || zoomLevel >= 2) {
    drawTriangle(triangleAlpha);
  }
  if (!zoomedIn || zoomLevel <= 2) {
    drawCircle(circleAlpha);
  }

  ctx.restore();
}



function toggleZoom() {
  if (zoomedIn && zoomLevel > 1) {
    zoomLevel -= zoomSpeed;
    draw();
    requestAnimationFrame(toggleZoom);
  } else if (!zoomedIn && zoomLevel < 3) {
    zoomLevel += zoomSpeed;
    draw();
    requestAnimationFrame(toggleZoom);
  } else {
    zoomedIn = !zoomedIn;
  }
}

zoomButton.addEventListener('click', toggleZoom);
draw();
