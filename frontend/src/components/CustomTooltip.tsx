import { GraphinContext } from '@antv/graphin'
import React from 'react'
import Card from './Card'

const CustomTooltip = () => {
  const { tooltip } = React.useContext(GraphinContext)
  const context = tooltip.node
  const { item } = context
  const model = item && item.getModel()

  return (
    <div>
      <Card id={model.id} />
    </div>
  )
}

export default CustomTooltip
