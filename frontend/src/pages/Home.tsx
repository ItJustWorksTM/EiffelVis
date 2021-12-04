import React from 'react'
import { SigmaContainer } from 'react-sigma-v2/lib/esm'
import Graph from '../components/Graph'
import 'react-sigma-v2/lib/react-sigma-v2.css'

const Home: React.FC = () => (
  <SigmaContainer
    style={{ background: '#1f1f1f' }}
    initialSettings={{
      hideEdgesOnMove: true,
      enableEdgeHoverEvents: false,
      renderLabels: false,
      renderEdgeLabels: false,
      defaultNodeColor: '#ad0c04',
      defaultEdgeColor: '#ffffff',
      zIndex: true,
    }}
    graphOptions={{ type: 'directed', allowSelfLoops: false, multi: false }}
  >
    <Graph />
  </SigmaContainer>
)

export default Home
