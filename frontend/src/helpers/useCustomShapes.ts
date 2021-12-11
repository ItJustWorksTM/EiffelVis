import G6 from '@antv/g6'

G6.registerEdge(
  'custom',
  {
    update: undefined,
  },
  'line'
)

G6.registerNode(
  'custom',
  {
    update: undefined,
  },
  'circle'
)

export default G6
