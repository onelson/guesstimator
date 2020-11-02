import React from 'react';

type Props = {
  onSelect: (card: number) => void;
  isCalling: boolean;
  selectedCard: number | undefined;
  playerName: string;
  cards: string[];
};

export function CardPicker(props: Props) {
  const { isCalling, selectedCard, playerName, cards, onSelect } = props;

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

  return (
    <div>
      <p>{`${playerName}, please select a card:`}</p>

      <ul className={classes.join(' ')}>
        {cards.map((name, idx) => {
          const classes = ['card'];

          if (selectedCard === idx) {
            classes.push('active');
          }

          return (
            <li
              key={name}
              className={classes.join(' ')}
              onClick={() => onSelect(idx)}
            >
              <div className="value">{name}</div>
            </li>
          );
        })}
      </ul>
    </div>
  );
}
