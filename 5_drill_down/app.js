const canvas = document.getElementById('imageCanvas');
const ctx = canvas.getContext('2d');

const zoomSpeed = 0.05;
const maxZoomLevel = 3;

let zoomLevel = 1;
let zoomedIn = false;

class Node {
  constructor(x, y, label) {
    this.x = x;
    this.y = y;
    this.label = label;
  }

  draw(alpha) {
    ctx.globalAlpha = alpha;
    ctx.beginPath();
    ctx.arc(this.x, this.y, 15, 0, 2 * Math.PI);
    ctx.fillStyle = '#f00';
    ctx.fill();
    ctx.font = '14px Arial';
    ctx.fillStyle = '#000';
    ctx.fillText(this.label, this.x - 3, this.y + 5);
    ctx.closePath();
  }
}

function generateGraph(zoomLevel) {
  const nodes = [];
  const padding = 50;
  const rangeX = (canvas.width - padding * 2) / zoomLevel;
  const rangeY = (canvas.height - padding * 2) / zoomLevel;
  const offsetX = (canvas.width - rangeX) / 2;
  const offsetY = (canvas.height - rangeY) / 2;

  for (let i = 0; i < 10; i++) {
    const x = offsetX + Math.random() * rangeX;
    const y = offsetY + Math.random() * rangeY;
    nodes.push(new Node(x, y, i + 1));
  }
  return nodes;
}

let currentGraph = generateGraph(1);
let nextGraph = generateGraph(maxZoomLevel);

function drawGraph(graph, alpha) {
  graph.forEach((node) => node.draw(alpha));
}

function draw() {
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  ctx.save();
  ctx.translate(canvas.width / 2, canvas.height / 2);
  ctx.scale(zoomLevel, zoomLevel);
  ctx.translate(-canvas.width / 2, -canvas.height / 2);

  const currentGraphAlpha = 1 - Math.min(1, Math.max(0, (zoomLevel - 1) * 2));
  const nextGraphAlpha = Math.min(1, Math.max(0, (zoomLevel - 1) * 2));

  if (!zoomedIn || zoomLevel <= maxZoomLevel / 2) {
    drawGraph(currentGraph, currentGraphAlpha);
  }
  if (zoomedIn || zoomLevel >= maxZoomLevel / 2) {
    drawGraph(nextGraph, nextGraphAlpha);
  }

  ctx.restore();
}

function zoomIn() {
  if (!zoomedIn && zoomLevel < maxZoomLevel) {
    zoomLevel += zoomSpeed;
    draw();
    requestAnimationFrame(zoomIn);
  } else {
    zoomedIn = true;
    currentGraph = nextGraph;
    nextGraph = generateGraph(maxZoomLevel);
  }
}

canvas.addEventListener('click', (e) => {
  const rect = canvas.getBoundingClientRect();
  const mouseX = e.clientX - rect.left;
  const mouseY = e.clientY - rect.top;

  for (let i = 0; i < currentGraph.length; i++) {
    const node = currentGraph[i];
    const distance = Math.sqrt(
      Math.pow(mouseX - node.x, 2) + Math.pow(mouseY - node.y, 2)
    );
    if (distance <= 10) {
      zoomIn();
      break;
    }
  }
});

draw();
