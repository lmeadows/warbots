export function get_power() {
  return Number(document.getElementById("power-box").value);
}

export function get_angle() {
  return Number(document.getElementById("angle-box").value);
}


export class UserInput {
  constructor() {
    this.power = parseInt(document.getElementById("power-box").value);
    this.angle = parseInt(document.getElementById("angle-box").value);
  }

  get power() {
    return this.power;
  }

  get angle() {
    return this.angle;
  }

  set power(n) {
    this.power = n;
  }

  set angle(n) {
    this.angle = n;
  }
}
