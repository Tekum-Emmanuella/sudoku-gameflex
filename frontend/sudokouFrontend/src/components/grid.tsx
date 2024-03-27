import React, { useEffect, useState } from 'react';
import './grid.css';

const Board: React.FC = () => {
  const [grid, setGrid] = useState<number[][]>([]);
  const [initialGrid, setInitialGrid] = useState<number[][]>([]);
  const [selectedCell, setSelectedCell] = useState<{ row: number; col: number } | null>(null);
  const [retries, setRetries] = useState<number>(3);

  // Generate a new Sudoku grid when component mounts
  useEffect(() => {
    const newGrid = generateNewGrid();
    setInitialGrid(newGrid);
    setGrid(newGrid);
  }, []);

  const handleCellClick = (row: number, col: number) => {
    if (initialGrid[row][col] === 0) {
      setSelectedCell({ row, col });
    }
  };

  const handleNumberInsert = (num: number) => {
    if (selectedCell !== null) {
      const { row, col } = selectedCell;
      if (initialGrid[row][col] === 0) {
        setGrid(prevGrid => {
          const newGrid = [...prevGrid];
          newGrid[row][col] = num;
          return newGrid;
        });
        setSelectedCell(null);
      }
    }
  };

  const handleEraseNumber = () => {
    if (selectedCell !== null) {
      const { row, col } = selectedCell;
      if (initialGrid[row][col] === 0) {
        setGrid(prevGrid => {
          const newGrid = [...prevGrid];
          newGrid[row][col] = 0;
          return newGrid;
        });
        setSelectedCell(null);
      }
    }
  };

  const handleRetry = () => {
    if (retries > 0) {
      setRetries(prevRetries => prevRetries - 1);
      setSelectedCell(null);
    } else {
      const newGrid = generateNewGrid(); // Generate a new grid
      setGrid(newGrid); // Reset the grid to a new game
      setInitialGrid(newGrid); // Update initial grid
      setRetries(3); // Reset retries count
      setSelectedCell(null);
    }
  };

  const generateNewGrid = () => {
    // Function to generate a new Sudoku grid
    const newGrid: number[][] = [];
    for (let i = 0; i < 9; i++) {
      const newRow: number[] = [];
      for (let j = 0; j < 9; j++) {
        newRow.push(0);
      }
      newGrid.push(newRow);
    }
  
    // Fill the grid with a valid Sudoku puzzle
    fillGrid(newGrid);
  
    // Remove some numbers to create a playable puzzle
    removeNumbers(newGrid);
  
    return newGrid;
  };
  
  const fillGrid = (grid: number[][]) => {
    // Recursive function to fill the Sudoku grid
    for (let row = 0; row < 9; row++) {
      for (let col = 0; col < 9; col++) {
        if (grid[row][col] === 0) {
          const possibleValues = getPossibleValues(grid, row, col);
          for (const value of possibleValues) {
            grid[row][col] = value;
            if (fillGrid(grid)) {
              return true;
            }
            grid[row][col] = 0;
          }
          return false;
        }
      }
    }
    return true;
  };
  
  const getPossibleValues = (grid: number[][], row: number, col: number) => {
    // Function to get possible values for a cell based on Sudoku rules
    const possibleValues: number[] = [];
    for (let num = 1; num <= 9; num++) {
      if (isValidMove(grid, row, col, num)) {
        possibleValues.push(num);
      }
    }
    return possibleValues;
  };
  
  const isValidMove = (grid: number[][], row: number, col: number, num: number) => {
    // Check if placing 'num' in grid[row][col] is a valid move
    return (
      isRowValid(grid, row, num) &&
      isColValid(grid, col, num) &&
      isSubgridValid(grid, Math.floor(row / 3) * 3, Math.floor(col / 3) * 3, num)
    );
  };
  
  const isRowValid = (grid: number[][], row: number, num: number) => {
    // Check if 'num' is valid in the given row
    return !grid[row].includes(num);
  };
  
  const isColValid = (grid: number[][], col: number, num: number) => {
    // Check if 'num' is valid in the given column
    for (let i = 0; i < 9; i++) {
      if (grid[i][col] === num) {
        return false;
      }
    }
    return true;
  };
  
  const isSubgridValid = (grid: number[][], startRow: number, startCol: number, num: number) => {
    // Check if 'num' is valid in the given subgrid (3x3)
    for (let i = startRow; i < startRow + 3; i++) {
      for (let j = startCol; j < startCol + 3; j++) {
        if (grid[i][j] === num) {
          return false;
        }
      }
    }
    return true;
  };
  
  const removeNumbers = (grid: number[][]) => {
    // Remove some numbers from the filled grid to create a playable puzzle
    const totalCells = 81; // 9x9 grid
    const emptyCells = Math.floor(totalCells * 0.6); // 60% of cells will be empty
  
    let cellsRemoved = 0;
    while (cellsRemoved < emptyCells) {
      const row = Math.floor(Math.random() * 9);
      const col = Math.floor(Math.random() * 9);
      if (grid[row][col] !== 0) {
        grid[row][col] = 0;
        cellsRemoved++;
      }
    }
  };
  
  

  return (
    
    <div className="board-container">
        <div className="number-buttons">
          {[1, 2, 3, 4, 5, 6, 7, 8, 9].map(num => (
            <button key={num} onClick={() => handleNumberInsert(num)}>
              {num}
            </button>
          ))}
          <button className="erase-button" onClick={handleEraseNumber}>Erase</button>
        </div>
      <div className="retry-section">
        <span className="retry-count">Retries left: {retries}</span>
        <button className="retry-button" onClick={handleRetry} disabled={retries === 0}>
         Retry
        </button>
      </div>
      <div className="board-grid"> 
      <table className="sudoku-grid">
        <tbody>
          {grid.map((row, rowIndex) => (
            <tr key={rowIndex}>
              {row.map((cell, colIndex) => (
                <td
                  key={colIndex}
                  onClick={() => handleCellClick(rowIndex, colIndex)}
                  className={`
                    ${selectedCell?.row === rowIndex && selectedCell?.col === colIndex ? 'selected' : ''}
                    ${initialGrid[rowIndex][colIndex] !== 0 ? 'initial' : ''}
                  `}
                >
                  {cell !== 0 ? cell : ''}
                </td>
              ))}
            </tr>
          ))}
          
        </tbody>
      </table>
      </div>
      
    </div>
  );
};

export default Board;