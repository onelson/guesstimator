import React from 'react';
import { GetGameState } from './__generated__/GetGameState';
import Confetti from 'react-dom-confetti';

const confettiConfig = {
  angle: 90,
  spread: 360,
  startVelocity: 40,
  elementCount: 200,
  dragFriction: 0.12,
  duration: 4_000,
  stagger: 3,
  width: '10px',
  height: '10px',
  perspective: '500px',
  colors: ['#a864fd', '#29cdff', '#78ff44', '#ff718d', '#fdff6a'],
};

type Props = {
  // should be fine to receive as a prop since the websocket broadcast only
  // fires when the gameState data changes.
  gameStateData: GetGameState;
  cards: string[];
};

export function PlayerCards(props: Props) {
  const { isCalling, players } = props.gameStateData.gameState;
  const { cards } = props;
  const classes = ['player-cards', 'flex', 'space-x-2', 'py-4'];
  if (isCalling) {
    classes.push('calling');
  }

  // Celebrate when everybody picks the same card.
  const consensus =
    isCalling &&
    !!players.length &&
    players[0].selectedCard !== null &&
    players.every((p) => p.selectedCard === players[0].selectedCard);

  return (
    <div className={classes.join(' ')}>
      <Confetti active={consensus} config={confettiConfig} />
      {players.map((player) => {
        return (
          <div key={player.id}>
            <div
              className={`card ${
                player.selectedCard === null ? 'undecided' : ''
              }`}
            >
              <div className={'value'}>
                {player.selectedCard !== null ? cards[player.selectedCard] : ''}
              </div>
            </div>
            <div className={'name text-center'}>{player.name}</div>
          </div>
        );
      })}
    </div>
  );
}
