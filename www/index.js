import "./canvas.css"
import { Config } from "warbots";

const config = Config.new();
const canvas = document.getElementById("warbots-canvas");
canvas.height = config.height();
canvas.width = config.width();

const TERRAIN_COLORS = [
  "27FF00",
  "43AB08",
  "9D5109",
  "EABC00",
  "00960E",
  "CCCCCC",
  "FFFFFF",
  "BAEFFF"
]

const TERRAIN_COLOR = "#" + TERRAIN_COLORS[Math.floor(Math.random() * TERRAIN_COLORS.length)]
const CELL_SIZE = 5;
const ctx = canvas.getContext('2d');
const height = 5;
const width = 5;

ctx.strokeStyle = TERRAIN_COLOR;

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



const renderLoop = () => {
  ctx.fillStyle="red";
  ctx.fillRect(20, 20, 10, 10);
  ctx.fillStyle="blue";
  ctx.fillRect(880, 20, 10, 10);
	requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);
