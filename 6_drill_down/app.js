const canvas = document.querySelector('#canvas');
const ctx = canvas.getContext('2d');

const slide1 = document.querySelector('#slide-1');
const slide2 = document.querySelector('#slide-2');
const slideName = document.querySelector('#slide-name');

const nodes = [
  {x: 100, y: 100, name: 'Node 1'},
  {x: 200, y: 100, name: 'Node 2'},
  {x: 300, y: 100, name: 'Node 3'},
  {x: 400, y: 100, name: 'Node 4'},
  {x: 500, y: 100, name: 'Node 5'},
  {x: 100, y: 200, name: 'Node 6'},
  {x: 200, y: 200, name: 'Node 7'},
  {x: 300, y: 200, name: 'Node 8'},
  {x: 400, y: 200, name: 'Node 9'},
  {x: 500, y: 200, name: 'Node 10'}
];

let selectedNode = null;
let zoomed = false;
let scale = 1;

slide1.addEventListener('mousemove', function(event) {
  const rect = canvas.getBoundingClientRect();
  const x = event.clientX - rect.left;
  const y = event.clientY - rect.top;
  
  if (x >= 0 && x <= canvas.width && y >= 0 && y <= canvas.height) {
    const node = findNode(x, y);
    if (node) {
      canvas.style.cursor = 'pointer';
    } else {
      canvas.style.cursor = 'default';
    }
  }
});

slide1.addEventListener('click', function(event) {
  const rect = canvas.getBoundingClientRect();
  const x = event.clientX - rect.left;
  const y = event.clientY - rect.top;
  
  if (x >= 0 && x <= canvas.width && y >= 0 && y <= canvas.height) {
    const node = findNode(x, y);
    if (node) {
      if (node === nodes[4]) {
        slideName.textContent = 'Security Union';
      } else {
        slideName.textContent = node.name;
      }
      selectedNode = node;
      zoomed = !zoomed;
      if (zoomed) {
        animate(node.x, node.y, 2);
      } else {
        animate(canvas.width / 2, canvas.height / 2, 1);
      }
    }
  }
});

function findNode(x, y) {
  for (let i = 0; i < nodes.length; i++) {
    const node = nodes[i];
    const dx = x - node.x;
    const dy = y - node.y;
    const distance = Math.sqrt(dx * dx + dy * dy);
    if (distance <= 50) {
      return node;
    }
  }
  return null;
}

function animate(x, y, targetScale) {
  const startScale = scale;
  const step = (targetScale - startScale) / 10;
  function loop() {
    if ((step > 0 && scale < targetScale) || (step < 0 && scale > targetScale)) {
      scale += step;
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      drawNodes();
      ctx.setTransform(1, 0, 0, 1, 0, 0);
      ctx.scale(scale, scale);
      ctx.translate(canvas.width / 2 - x, canvas.height / 2 - y);
      requestAnimationFrame(loop);
    } else {
      scale = targetScale;
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      drawNodes();
      ctx.setTransform(1, 0, 0, 1, 0, 0);
      ctx.scale(scale, scale);
      ctx.translate(canvas.width / 2 - x, canvas.height / 2 - y);
    }
  }
  loop();
}

function drawNodes() {
  for (let i = 0; i < nodes.length; i++) {
    const node = nodes[i];
    ctx.beginPath();
    ctx.arc(node.x, node.y, 50, 0, 2 * Math.PI);
    ctx.fillStyle = '#0072c6';
    ctx.fill();
    ctx.fillStyle = '#ffffff';
    ctx.font = '12px Arial';
    ctx.textAlign = 'center';
    ctx.textBaseline = 'middle';
    ctx.fillText(node.name, node.x, node.y);
  }
}

// Initial draw
drawNodes();

