import React from 'react';

type Player = any; // FIXME
export function PlayerCards() {
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
