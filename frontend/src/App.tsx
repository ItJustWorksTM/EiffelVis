import React from "react";
import { Container } from "react-bootstrap";
import { BrowserRouter as Router, Route } from "react-router-dom";
import About from "./pages/About";
import Home from "./pages/Home";

const App: React.FC = () => {
  return (
    <Router>
      <main>
        <Container>
          <Route path="/about" exact component={About} />
          <Route path="/" exact component={Home} />
        </Container>
      </main>
    </Router>
  );
};

export default App;
