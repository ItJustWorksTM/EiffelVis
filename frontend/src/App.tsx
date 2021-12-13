import React from 'react'
import { Container } from 'react-bootstrap'
import Home from './pages/Home'
import styles from './css/app.module.css'
import './css/controlPanel.css'
import './css/global.css'

const App: React.FC = () => (
  <main className={styles.mainContainer}>
    <Container fluid className={styles.routeContainer}>
      <Home />
    </Container>
  </main>
)

export default App
