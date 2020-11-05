import React from 'react';
import { GetGameState } from './__generated__/GetGameState';

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

  return (
    <div className={classes.join(' ')}>
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
