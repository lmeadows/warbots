import { Config, start, player_fire } from "warbots";

const config = Config.new();
const canvas = document.getElementById("warbots-canvas");
canvas.height = config.height();
canvas.width = config.width();

// tell Rust-WASM code to start the game
start();

const powerBox = document.getElementById("power-box");
const angleBox = document.getElementById("angle-box");
window.addEventListener("keydown", (e) => {
	// disable scrolling for space and arrow keys
	if([32, 37, 38, 39, 40].indexOf(e.keyCode) > -1) {
			e.preventDefault();
	}

  switch (e.keyCode) {
    case 37:
      // left key
      incrementAngle(-1);
      break;
    case 38:
      // up key
      incrementPower(1);
      break;
    case 39:
      // right key
      incrementAngle(1);
      break;
    case 40:
      // down key
      incrementPower(-1);
      break;
  }
});

const maxAngle = config.max_angle();
const minAngle = config.min_angle();
const maxPower = config.max_power();
const minPower = config.min_power();
function incrementAngle(diff) {
  let value = parseInt(angleBox.value) + diff;
  if (minAngle <= value && value <= maxAngle) {
    angleBox.value = value;
  }
}
function incrementPower(diff) {
  let value = parseInt(powerBox.value) + diff;
  if (minPower <= value && value <= maxPower) {
    powerBox.value = value;
  }
}
