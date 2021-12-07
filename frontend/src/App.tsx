import React from 'react'
import { Container } from 'react-bootstrap'
import { Route, BrowserRouter as Router } from 'react-router-dom'
import About from './pages/About'
import Home from './pages/Home'
import styles from './css/app.module.css'
import './css/controlPanel.css'
import './css/global.css'

const App: React.FC = () => (
  <Router>
    <main className={styles.mainContainer}>
      <Container fluid className={styles.routeContainer}>
        <Route path="/about" exact component={About} />
        <Route path="/" exact component={Home} />
      </Container>
    </main>
  </Router>
)

export default App
