import React, { useState } from 'react';
import './grid.css';

const Board: React.FC = () => {
  const [grid, setGrid] = useState<number[][]>([
    [0, 0, 5, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 3, 4, 0, 0, 0, 0],
    [0, 0, 4, 0, 0, 0, 7, 0, 0],
    [0, 0, 0, 0, 0, 6, 0, 0, 0],
    [1, 0, 0, 0, 0, 0, 9, 0, 0],
    [0, 0, 7, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 3, 0, 0, 0, 0],
    [4, 0, 0, 6, 0, 0, 0, 4, 0],
    [0, 2, 0, 0, 0, 0, 0, 0, 0],
  ]);

  const [selectedRow, setSelectedRow] = useState<number | null>(null);
  const [selectedCol, setSelectedCol] = useState<number | null>(null);
  const [retries, setRetries] = useState<number>(3);

  const handleCellClick = (row: number, col: number) => {
    setSelectedRow(row);
    setSelectedCol(col);
  };

  const handleNumberInsert = (num: number) => {
    if (selectedRow !== null && selectedCol !== null) {
      const newGrid = [...grid];
      newGrid[selectedRow][selectedCol] = num;
      setGrid(newGrid);
      setSelectedRow(null);
      setSelectedCol(null);
    }
  };

  const handleEraseNumber = () => {
    if (selectedRow !== null && selectedCol !== null) {
      const newGrid = [...grid];
      newGrid[selectedRow][selectedCol] = 0;
      setGrid(newGrid);
      setSelectedRow(null);
      setSelectedCol(null);
    }
  };

  const handleRetry = () => {
    if (retries > 0) {
      setRetries(retries - 1);
      setSelectedRow(null);
      setSelectedCol(null);
    }
  };

  // Validation logic can be added here based on your specific Sudoku rules

  return (
    <div className="grid-container">
      <div className="retry-section">
        <span className="retry-count">Retries left: {retries}</span>
        <button
          className="retry-button"
          onClick={handleRetry}
          disabled={retries === 0}
        >
        </button>
      </div>
      <table>
        <tbody>
          {grid.map((row, rowIndex) => (
            <tr key={rowIndex}>
              {row.map((cell, colIndex) => (
                <td
                  key={colIndex}
                  onClick={() => handleCellClick(rowIndex, colIndex)}
                  style={{
                    backgroundColor:
                      selectedRow === rowIndex || selectedCol === colIndex
                        ? 'lightblue'
                        : 'white',
                  }}
                >
                  {cell !== 0 ? cell : ''}
                </td>
              ))}
            </tr>
          ))}
        </tbody>
      </table>
      <div>
        <div className="number-buttons">
          <button onClick={() => handleNumberInsert(1)}> 1</button>
          <button onClick={() => handleNumberInsert(2)}> 2</button>
          <button onClick={() => handleNumberInsert(3)}> 3</button>
          <button onClick={() => handleNumberInsert(4)}> 4</button>
          <button onClick={() => handleNumberInsert(5)}> 5</button>
          <button onClick={() => handleNumberInsert(6)}> 6</button>
          <button onClick={() => handleNumberInsert(7)}> 7</button>
          <button onClick={() => handleNumberInsert(8)}> 8</button>
          <button onClick={() => handleNumberInsert(9)}> 9</button>
          <button onClick={handleEraseNumber}>Erase</button>
        </div>
      </div>
    </div>
  );
};

export default Board;