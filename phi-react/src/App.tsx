import React from 'react';
import { PlayerCards } from './PlayerCards';
import { CardPicker } from './CardPicker';

function App() {
  const handleCall = () => {}; // FIXME
  const isAdmin = true; // FIXME

  return (
    <div className="container mx-auto flex flex-col space-y-4">
      <PlayerCards />
      <div>
        <label>Name:</label>
        <input />
      </div>
      <CardPicker />
      {isAdmin ? (
        <button className="btn-red" onClick={handleCall}>
          Call
        </button>
      ) : null}
    </div>
  );
}

export default App;
