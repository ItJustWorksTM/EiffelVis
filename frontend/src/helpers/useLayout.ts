/* eslint-disable no-bitwise */
let timee = 0
let posx = 0
let posy = 0
let log = 1
export const colors = new Map()

export const layout = (node: any) => {
  const temp = node
  temp.type = 'custom'
  const tempTime: number = temp.time
  if (tempTime <= timee + 1000) {
    temp.x = posx
    if (posy < 0) {
      temp.y = posy
      posy = posy * -1 + 60 * 0.99 ** log
      log += 1
    } else {
      temp.y = posy
      if (posy !== 0) {
        posy *= -1
      }
    }
  } else if (tempTime > timee) {
    posx += 60
    temp.x = posx
    posy = 0
    log = 1
    temp.y = posy
    posy += 60
    timee = tempTime
  }
}

export const resetLayout = () => {
  timee = 0
  posx = 0
  posy = 0
  log = 1
}

export function nodeColor(eventType: string) {
  if (!colors.has(eventType)) {
    const hash = [...eventType].reduce(
      (acc, char) => char.charCodeAt(0) + ((acc << 5) - acc),
      2
    )
    const color = `hsl(${hash % 360}, 50%, 50%)`
    colors.set(eventType, color)
  }
  return colors.get(eventType)
}
