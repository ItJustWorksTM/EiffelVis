import { Pane } from 'tweakpane'
import { useEffect, useState } from 'react'
import { TweakCb } from '../interfaces/types'

const useTweakPane = (cb: TweakCb): void => {
  const [paneData, setPaneData] = useState<any>({
    type: 'All',
    id: '',
  })

  useEffect(() => {
    const pane = new Pane({ title: 'Events', expanded: true })
    pane.addInput(paneData, 'type', {
      label: 'Type',
      options: { All: 'All', WithRoot: 'WithRoot' },
    })
    pane.addSeparator()
    pane.addInput(paneData, 'id', {
      label: 'node id',
      step: 1,
    })

    pane
      .addButton({
        title: 'New Nodes',
      })
      .on('click', () => cb(pane.exportPreset()))

    pane.on('change', () => {
      setPaneData(pane.exportPreset())
    })
  }, [])
}

export default useTweakPane
