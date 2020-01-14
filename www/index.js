import "./canvas.css"
import { Config, start } from "warbots";

const config = Config.new();
const canvas = document.getElementById("warbots-canvas");
canvas.height = config.height();
canvas.width = config.width();

start();
