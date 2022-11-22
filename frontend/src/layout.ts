import config from "./config.json";

export interface GraphSettings {
  offset: number
  time_diff: number
  y_scale: number
  x_sep: number
  y_sep: number
  hue: number
}
export const defaultNode = {
  color: "#93ACB5",
  shape: "diamond",
  type: "Unknown"
}
export class StatefulLayout {
  private timee = 0
  private posx = 0
  private posy = 0
  private log = 1
  private curve = 0
  private curveSep = 0
  private colors = new Map<string, string>()
  private shapes = new Map<string, string>()
  private customTheme = config.Theme.ColorBlind
  private themeMap = new Map(Object.entries(this.customTheme))




  apply(node: any, graphOptions: GraphSettings) {
    if (this.curve === 0 && graphOptions.offset) {
      this.curve = graphOptions.offset
      this.curveSep = graphOptions.offset
    }
    node.style = {
      fill: this.nodeColor(node.event_type),
      lineWidth: 0.4,
    }
    node.size = 10
    node.type = this.nodeShape(node.event_type)


    const temp = node
    const tempTime: number = temp.time
    if (tempTime <= this.timee + graphOptions.time_diff) {
      temp.x = this.posx + this.curve
      if (this.posy < 0) {
        temp.y = this.posy
        this.posy =
          this.posy * -1 + graphOptions.y_sep * graphOptions.y_scale ** this.log
        this.log += 1
        this.curve += this.curveSep
      } else {
        temp.y = this.posy
        if (this.posy !== 0) {
          this.posy *= -1
        }
      }
    } else if (tempTime > this.timee) {
      this.posx = this.posx + this.curve + graphOptions.x_sep
      temp.x = this.posx
      this.curve = 0
      this.posy = 0
      this.log = 1
      temp.y = this.posy
      this.posy += graphOptions.y_sep
      this.timee = tempTime
    }
  }

  nodeColor(eventType: string) {
    if (!this.themeMap.has(eventType)) {
      // defaultNode color
      return defaultNode.color
    }
    return this.themeMap.get(eventType).Color
  }

  getNodeColor() {
    return this.colors
  }
  getNodeShape() {
    return this.shapes
  }

  getNodeStyle() {
    return this.themeMap
  }
  nodeShape(eventType: string) {
    if (!this.themeMap.has(eventType)) {
      return defaultNode.shape
    }

    return this.themeMap.get(eventType).Shape
  }
  nodeLabel(eventType: string) {
    if (!this.themeMap.has(eventType)) {
      return defaultNode.type
    }
    return this.themeMap.get(eventType).Acronym
  }

}
