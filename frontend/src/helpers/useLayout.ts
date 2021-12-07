let timee = 0
let posx = 0
let posy = 0
let log = 1

export const layout = (node: any) => {
  const temp = node
  temp.type = 'custom'
  const tempTime: number = temp.time
  if (tempTime <= timee + 1000) {
    temp.x = posx
    if (posy < 0) {
      temp.y = posy
      posy = posy * -1 + 50 * 0.99 ** log
      log += 1
    } else {
      temp.y = posy
      if (posy !== 0) {
        posy *= -1
      }
    }
  } else if (tempTime > timee) {
    posx += 50
    temp.x = posx
    posy = 0
    log = 1
    temp.y = posy
    posy += 50
    timee = tempTime
  }
}

export const resetLayout = () => {
  timee = 0
  posx = 0
  posy = 0
  log = 1
}
