import { Pane } from 'tweakpane'
import { useEffect, useState } from 'react'

// eslint-disable-next-line no-unused-vars
const useTweakPane = (cb: (data: any) => void): void => {
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
