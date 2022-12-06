import config from "./config.json";
<<<<<<< HEAD
<<<<<<< HEAD
import type { Event } from "./apidefinition"
=======

>>>>>>> ebdd8b1 (Frontend: Graph Legend Update (#124))
=======
import type { Event } from "./apidefinition"
>>>>>>> db0a866 (FrontEnd: Re-factor the overall layout (#141))
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
  type: "???"
}
<<<<<<< HEAD
<<<<<<< HEAD
type node = Event & {
  event_type: string
  style?: object
=======
type node = Event & {
  event_type: string
  style?: object 
>>>>>>> db0a866 (FrontEnd: Re-factor the overall layout (#141))
  size?: number
  type?: string
  time: number
  x?: number
  y?: number
}
<<<<<<< HEAD
export class StatefulLayout {
  private timee: number = 0
  private posx: number = 0
  private posy: number = 0
  private log: number = 1
  private curve: number = 0
  private curveSep: number = 0
  private colors: Map<string, string> = new Map<string, string>()
  private shapes: Map<string, string> = new Map<string, string>()
  private customTheme: Object = config.Theme.ColorBlind
  private themeMap: Map<string, any> = new Map(Object.entries(this.customTheme))
=======
=======
>>>>>>> db0a866 (FrontEnd: Re-factor the overall layout (#141))
export class StatefulLayout {
  private timee: number = 0
  private posx: number = 0
  private posy: number = 0
  private log: number = 1
  private curve: number = 0
  private curveSep: number = 0
  private colors: Map<string, string> = new Map<string, string>()
  private shapes: Map<string, string> = new Map<string, string>()
  private customTheme: Object = config.Theme.ColorBlind
  private themeMap: Map<string, any> = new Map(Object.entries(this.customTheme))



>>>>>>> ebdd8b1 (Frontend: Graph Legend Update (#124))

<<<<<<< HEAD



  apply(node: node, graphOptions: GraphSettings) {
=======
  apply(node: node, graphOptions: GraphSettings) { 
>>>>>>> db0a866 (FrontEnd: Re-factor the overall layout (#141))
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
<<<<<<< HEAD
=======

>>>>>>> ebdd8b1 (Frontend: Graph Legend Update (#124))


    const temp: node = node;
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

<<<<<<< HEAD
<<<<<<< HEAD
  nodeColor(eventType: string): string {
=======
  nodeColor(eventType: string) {
>>>>>>> ebdd8b1 (Frontend: Graph Legend Update (#124))
=======
  nodeColor(eventType: string): string {
>>>>>>> db0a866 (FrontEnd: Re-factor the overall layout (#141))
    if (!this.themeMap.has(eventType)) {
      // defaultNode color
      return defaultNode.color
    }
    return this.themeMap.get(eventType).Color
  }

  getNodeColor(): Map<string, string> {
    return this.colors
  }
<<<<<<< HEAD
<<<<<<< HEAD
  getNodeShape(): Map<string, string> {
    return this.shapes
  }

  getNodeStyle(): Map<string, any> {
    return this.themeMap
  }
  nodeShape(eventType: string): string {
=======
  getNodeShape() {
=======
 getNodeShape(): Map<string, string> {
>>>>>>> db0a866 (FrontEnd: Re-factor the overall layout (#141))
    return this.shapes
  }

  getNodeStyle(): Map<string, any> {
    return this.themeMap
  }
<<<<<<< HEAD
  nodeShape(eventType: string) {
>>>>>>> ebdd8b1 (Frontend: Graph Legend Update (#124))
=======
  nodeShape(eventType: string): string {
>>>>>>> db0a866 (FrontEnd: Re-factor the overall layout (#141))
    if (!this.themeMap.has(eventType)) {
      return defaultNode.shape
    }

    return this.themeMap.get(eventType).Shape
  }
<<<<<<< HEAD
<<<<<<< HEAD
  nodeLabel(eventType: string): string {
=======
  nodeLabel(eventType: string) {
>>>>>>> ebdd8b1 (Frontend: Graph Legend Update (#124))
=======
  nodeLabel(eventType: string): string {
>>>>>>> db0a866 (FrontEnd: Re-factor the overall layout (#141))
    if (!this.themeMap.has(eventType)) {
      return defaultNode.type
    }
    return this.themeMap.get(eventType).Acronym
  }

}
