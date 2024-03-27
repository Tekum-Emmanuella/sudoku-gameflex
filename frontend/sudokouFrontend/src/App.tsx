

import React from 'react';
import Board from './components/grid';
import './components/grid.css';

const App: React.FC = () => {
  return (
    <div className="App">
      {/* <h1>Sudoku Game</h1> */}
      <Board />
    </div>
  );
};

export default App;