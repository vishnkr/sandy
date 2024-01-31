import { PARTICLE_PROPERTIES, getSelectedParticleProps } from "./shared";
import init,{World, ElementType} from './sandy-rs/pkg/sandy_rs';


class FPS {
  constructor(){
    this.fps = document.getElementById("fps");
    this.frames = [];
    this.lastFrameTimeStamp = performance.now();
  }

  render() {
    const now = performance.now();
    const delta = now - this.lastFrameTimeStamp;
    this.lastFrameTimeStamp = now;
    const fps = (1 / delta) * 1000;

    this.frames.push(fps);
    if (this.frames.length > 100) {
      this.frames.shift();
    }

    let sum = 0;
    for (let i = 0; i < this.frames.length; i++) {
      sum += this.frames[i];
    }
    let mean = sum / this.frames.length;
    this.fps.textContent = `
         Frames per Second: latest = ${Math.round(fps)}
        avg of last 100 = ${Math.round(mean)}
        `.trim();    
  }
    
}

async function run(){
  let raw = await init();
  const canvas = document.getElementById('canvas');
  const ctx = canvas.getContext('2d');
  

  function clearRect(){
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      ctx.fillStyle = "rgb(255,255,255)";
      ctx.fillRect(0, 0, canvas.width, canvas.height);
  }
  
  let CELL_SIZE = 5;
  const canvasWidth = 700;
  const canvasHeight = 700;
  const height = canvasHeight/CELL_SIZE;
  const width = canvasWidth/CELL_SIZE;
  const world = new World(height,width);
  const elementTypeMap = {
      "Sand":ElementType.Sand, 
      "Stone":ElementType.Stone,
      "Water":ElementType.Water,
      "Acid":ElementType.Acid,
      "Fire":ElementType.Fire,
      "Ice": ElementType.Ice
    };
    const reverseElementTypeMap = {};
    for (const key in elementTypeMap) {
        const value = elementTypeMap[key];
        reverseElementTypeMap[value] = key;
    }
  
  canvas.height = canvasHeight;
  canvas.width = canvasWidth;
  const cell_count = width*height;
  const getIndex = (row, column) => {
    return row * width + column;
  };
  
  const fps = new FPS();
  
  const isInBounds = (idx)=>{
    return idx>=0 && idx<cell_count
  }
  const renderLoop = ()=> {
      fps.render();
      world.tick();
      draw();
      requestAnimationFrame(renderLoop);
  }
  
  function fillSquare(col,row,color){
      ctx.fillStyle=color;
      ctx.fillRect(row * CELL_SIZE, col* CELL_SIZE,CELL_SIZE,CELL_SIZE)
  }
  let count = 0
  function draw() {
      clearRect()
      //const cellsPtr = world.cells();
      //const cells = new Uint8Array(raw.memory.buffer, cellsPtr, width * height * 2);
      for (let y = 0; y < height; y++) {
          for (let x = 0; x < width; x++) {
            const idx = getIndex(y, x);
            let elementType = world.get_element_type(idx)
            if (elementType!==ElementType.Empty){
              let elementName = reverseElementTypeMap[elementType];
              let props = PARTICLE_PROPERTIES[elementName]
              let color = `rgb(${props.color.r},${props.color.g},${props.color.b})`
              fillSquare(y,x,color)
            };
               
          }
      }
  }
  
  
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
    const boundingRect = canvas.getBoundingClientRect();
  
    const elementProps = getSelectedParticleProps();
    const elementType = elementTypeMap[elementProps.name];
    /*
    const canvasLeft = (e.clientX - boundingRect.left) * scaleX;
    const canvasTop = (e.clientY - boundingRect.top) * scaleY;
    const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
    const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);*/
      var x = e.clientX - boundingRect.left,
          y = e.clientY - boundingRect.top;
      let col = Math.floor(x/CELL_SIZE),
         row = Math.floor(y/CELL_SIZE);

      let idx = getIndex(row,col);
      if (isInBounds(idx)){
        world.paint(row,col,elementType);
      }
      draw();
  }
  
  requestAnimationFrame(renderLoop);
  canvas.addEventListener('mousedown',handleMouseDown);
  canvas.addEventListener('mouseup',handleMouseUp);
  canvas.addEventListener('mousemove',handleMouseMove);
  
  function reset(){
    world.reset();
    draw();
  }
  document.getElementById("reset-button").addEventListener("click",reset);

}

run()
