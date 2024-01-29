import { getSelectedParticleProps } from "./shared";

const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');

ctx.fillStyle = "rgb(255,255,255)";
ctx.fillRect(0, 0, canvas.width, canvas.height);

let lastRenderTime= 0;
const DIFF = 1000/60;

function clearRect(){
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.fillStyle = "rgb(255,255,255)";
    ctx.fillRect(0, 0, canvas.width, canvas.height);
}


let grid, velocityGrid;
let w = 5;
let cols = canvas.width/w
let rows = canvas.height/w;
let hueValue = 200;

let gravity = 0.1;

function setup() {
    grid = make2DArray(cols, rows);
    velocityGrid = make2DArray(cols, rows, 1);
}

function make2DArray(cols, rows) {
    let arr = new Array(cols);
    for (let i = 0; i < arr.length; i++) {
      arr[i] = new Array(rows);
      for (let j = 0; j < arr[i].length; j++) {
        arr[i][j] = 0;
      }
    }
    return arr;
  }

  function withinCols(i) {
    return i >= 0 && i <= cols - 1;
  }
  
  function withinRows(j) {
    return j >= 0 && j <= rows - 1;
  }
  
function frame(currentTime) {
    update();
    draw();
    requestAnimationFrame(frame);
}

function fillSquare(x,y,w){
    let particleProps = getSelectedParticleProps();
    const color = `rgb(${particleProps.color.r}, ${particleProps.color.g}, ${particleProps.color.b})`
    ctx.fillStyle = color
    ctx.fillRect(x,y,w,w)
}

function draw() {
    clearRect()
    for (let i = 0; i < cols; i++) {
        for (let j = 0; j < rows; j++) {
          if (grid[i][j] > 0) {
            var x = i*w;
            var y = j*w; 
            fillSquare(x,y,w,'rgb(0,0,0)');
          }
        }
    }

}

function update(){
  let nextGrid = make2DArray(cols, rows);
  let nextVelocityGrid = make2DArray(cols, rows);

  for (let i = 0; i < cols; i++) {
    for (let j = 0; j < rows; j++) {
      let state = grid[i][j];
      let velocity = velocityGrid[i][j];
      let moved = false;
      if (state > 0) {
        let newPos = Math.floor(j + velocity);
        for (let y = newPos; y > j; y--) {
          let below = grid[i][y];
          let dir = 1;
          if (Math.random(1) < 0.5) {
            dir *= -1;
          }
          let belowA = -1;
          let belowB = -1;
          if (withinCols(i + dir)) belowA = grid[i + dir][y];
          if (withinCols(i - dir)) belowB = grid[i - dir][y];

          if (below === 0) {
            nextGrid[i][y] = state;
            nextVelocityGrid[i][y] = velocity + gravity;
            moved = true;
            break;
          } else if (belowA === 0) {
            nextGrid[i + dir][y] = state;
            nextVelocityGrid[i + dir][y] = velocity + gravity;
            moved = true;
            break;
          } else if (belowB === 0) {
            nextGrid[i - dir][y] = state;
            nextVelocityGrid[i - dir][y] = velocity + gravity;
            moved = true;
            break;
          }
        }
      }

      if (state > 0 && !moved) {
        nextGrid[i][j] = grid[i][j];
        nextVelocityGrid[i][j] = velocityGrid[i][j] + gravity;
      }
    }
  }
  grid = nextGrid;
  velocityGrid = nextVelocityGrid;
}

var elemLeft = canvas.offsetLeft + canvas.clientLeft;
var elemTop = canvas.offsetTop + canvas.clientTop;
let isMousePressed = false;
function handleMouseDown(e) {
    isMousePressed = true;
    handleMouseAction(e);
}

function handleMouseUp() {
    isMousePressed = false;
}

function handleMouseMove(e) {
    if (isMousePressed) {
        handleMouseAction(e);
    }
}

function handleMouseAction(e){
    var x = e.pageX - elemLeft,
        y = e.pageY - elemTop;
    let mouseCol = Math.floor(x/w);
    let mouseRow = Math.floor(y/w);
    let matrix = 5;
    let extent = Math.floor(matrix / 2);
    for (let i = -extent; i <= extent; i++) {
      for (let j = -extent; j <= extent; j++) {
        if (Math.random(1) < 0.75) {
          let col = mouseCol + i;
          let row = mouseRow + j;
          if (withinCols(col) && withinRows(row)) {
            grid[col][row] = hueValue;
            velocityGrid[col][row] = 1;
          }
        }
      }
    }
    if (withinCols(mouseCol) && withinRows(mouseRow)) {
        grid[mouseCol][mouseRow] = 1;
    }
}

setup();
requestAnimationFrame(frame);
document.addEventListener('mousedown',handleMouseDown);
document.addEventListener('mouseup',handleMouseUp);
document.addEventListener('mousemove',handleMouseMove);
