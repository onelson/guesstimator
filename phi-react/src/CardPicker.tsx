import React from 'react';

export function CardPicker() {
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
