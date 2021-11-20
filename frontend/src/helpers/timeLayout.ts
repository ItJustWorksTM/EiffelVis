import G6 from "@antv/g6";

G6.registerLayout('timeLayout', {
    /**
     * The default configurations of the custom layout. It will be mixed by the configurations from users
     */
    getDefaultCfg() {
      return {
        center: [0, 0],
        nodeSize: 1,
      }
    },
  
    execute() {
      const self = this
      const { nodes } = self
      if (!nodes) return

      console.log("started")
  
      let time = 0
      let posx = 0
      let posy = 0
      let log = 1
      let flag = 0
      console.log("calculating new values for nodes: ", nodes?.length)
      // TODO: this runs for the whole graph everytime
      nodes?.forEach((node:any ) => {
        const temp = node
        const tempTime:number = temp.time
        if (tempTime === time) {
          temp.x = posx + flag
          if (posy < 0) {
            temp.y = posy
            posy = (posy * 1) + (100 * (0.99**log))
            log += 1
          } else {
            temp.y = posy
            if (posy !== 0){
              posy *= -1
            } 
          }
          flag += 10
        } else if (tempTime > time){
          posx += 100
          temp.x = posx
          posy = 0
          log = 1
          temp.y = posy
          posy += 100
          time = tempTime
          flag = 0
        }
      })
    },
  
    destroy() {},
  })

export default G6