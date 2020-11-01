import React from 'react';

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

type Player = any; // FIXME

function PlayerCards() {
  const isCalling = true; // FIXME
  const classes = ['player-cards', 'flex', 'space-x-2', 'py-4'];
  if (isCalling) {
    classes.push('calling');
  }

  const players: Player[] = [{ id: 'xxxx', selectedCard: 1, name: 'Trent' }]; // FIXME
  const cards: string[] = ['a', 'b']; // FIXME

  return (
    <div className={classes.join(' ')}>
      {players.map((player) => {
        return (
          <div key={player.id}>
            <div className={`card ${player.selectedCard ? '' : 'undecided'}`}>
              <div className={'value'}>
                {player.selectedCard ? cards[player.selectedCard] : ''}
              </div>
            </div>
            <div className={'name text-center'}>{player.name}</div>
          </div>
        );
      })}
    </div>
  );
}

function CardPicker() {
  const isCalling = false; // FIXME

  const classes = [
    'card-picker',
    'grid',
    'grid-flow-row',
    'grid-cols-4',
    'sm:grid-cols-6',
    'md:grid-cols-12',
    'gap-8',
    'py-4',
  ];

  if (isCalling) {
    classes.push('calling');
  }

  const playerName = 'J. Doe'; // FIXME
  const cards: string[] = []; // FIXME
  const player: any = {}; // FIXME
  const handleClick = () => {}; // FIXME

  return (
    <div>
      <p>{`${playerName}, please select a card:`}</p>

      <ul className={classes.join(' ')}>
        {cards.map((name, idx) => {
          const classes = ['card'];

          if (player.selectedCard === idx) {
            classes.push('active');
          }

          return (
            <li key={name} className={classes.join(' ')} onClick={handleClick}>
              <div className="value">{name}</div>
            </li>
          );
        })}
      </ul>
    </div>
  );
}

export default App;
