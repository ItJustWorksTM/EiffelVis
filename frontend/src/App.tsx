import React from 'react'
import { Container } from 'react-bootstrap'
import { Route, BrowserRouter as Router } from 'react-router-dom'
import About from './pages/About'
import Home from './pages/Home'

const App: React.FC = () => (
  <Router>
    <main>
      <Container>
        <Route path="/about" exact component={About} />
        <Route path="/" exact component={Home} />
      </Container>
    </main>
  </Router>
)

export default App
