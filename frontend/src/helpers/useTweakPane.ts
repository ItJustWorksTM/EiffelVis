import { Pane } from 'tweakpane'
import { useEffect, useState } from 'react'
import { TweakCb } from '../interfaces/types'

const useTweakPane = (cb: TweakCb): void => {
  const [paneData, setPaneData] = useState<any>({
    collection_type: 'Forward',
    filter_type: 'None',
    id: '',
    begin: 0,
    end: -1,
    name: '',
    offset: 0,
  })

  useEffect(() => {
    const pane = new Pane({ title: 'Events', expanded: true })
    pane.addInput(paneData, 'collection_type', {
      label: 'Collection',
      options: { Forward: 'Forward', AsRoots: 'AsRoots' },
    })
    pane.addInput(paneData, 'filter_type', {
      label: 'Filter',
      options: { None: 'None', Time: 'Time', Ids: 'Ids', Type: 'Type' },
    })
    pane.addSeparator()
    pane.addInput(paneData, 'begin', {
      label: 'begin',
      step: 1,
    })

    pane.addInput(paneData, 'end', {
      label: 'end',
      step: 1,
    })

    pane.addInput(paneData, 'id', {
      label: 'node id',
    })

    pane.addInput(paneData, 'name', {
      label: 'type name',
      step: 1,
    })

    pane.addSeparator()
    pane.addInput(paneData, 'offset', {
      label: 'offset',
      step: 1,
    })

    pane
      .addButton({
        title: 'Submit',
      })
      .on('click', () => cb(pane.exportPreset()))

    pane.on('change', () => {
      setPaneData(pane.exportPreset())
    })
  }, [])
}

export default useTweakPane
