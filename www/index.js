import "./canvas.css"
import * as wasm from "warbots";

wasm.greet();

const canvas = document.getElementById("warbots-canvas");

const TERRAIN_COLOR = "#CCCCCC";
const CELL_SIZE = 5;
const ctx = canvas.getContext('2d');
//const width = universe.width();
const height = 5;
const width = 5;

  ctx.strokeStyle = "#FFF";

// parameters - change to your liking
 var STEP_MAX = 2.5;
 var STEP_CHANGE = 1.0;
 var HEIGHT_MAX = canvas.height;

 // starting conditions
 var terrain_height = Math.random() * HEIGHT_MAX;
 var slope = (Math.random() * STEP_MAX) * 2 - STEP_MAX;

 // creating the landscape
 for (var x = 0; x < canvas.width; x++) {
      // change height and slope
      terrain_height += slope;
      slope += (Math.random() * STEP_CHANGE) * 2 - STEP_CHANGE;

      // clip height and slope to maximum
      if (slope > STEP_MAX) { slope = STEP_MAX };
      if (slope < -STEP_MAX) { slope = -STEP_MAX };
 
      if (terrain_height > HEIGHT_MAX) { 
          terrain_height = HEIGHT_MAX;
          slope *= -1;
      }
      if (terrain_height < 0) { 
          terrain_height = 0;
          slope *= -1;
      }
      
      // draw column
      ctx.beginPath();
      ctx.moveTo(x, HEIGHT_MAX);
      ctx.lineTo(x, terrain_height);
      ctx.stroke();
 }



let counter = 1;
const renderLoop = () => {
	requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);
