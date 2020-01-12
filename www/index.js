import "./canvas.css"
import { Config, start } from "warbots";

const config = Config.new();
const canvas = document.getElementById("warbots-canvas");
canvas.height = config.height();
canvas.width = config.width();

start();

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
