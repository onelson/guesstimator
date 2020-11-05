import React, { useCallback, useEffect } from 'react';
import { PlayerCards } from './PlayerCards';
import { CardPicker } from './CardPicker';
import { useQuery, gql, useMutation, useSubscription } from '@apollo/client';
import { GetCards } from './__generated__/GetCards';
import { GetGameState } from './__generated__/GetGameState';
import { GetClientId } from './__generated__/GetClientId';
import { SetPlayerCard } from './__generated__/SetPlayerCard';

const REGISTER = gql`
  mutation GetClientId {
    register
  }
`;

const GET_CARDS = gql`
  query GetCards {
    cards
  }
`;

const GET_GAME_STATE = gql`
  subscription GetGameState {
    gameState {
      isCalling
      players {
        id
        selectedCard
        name
      }
    }
  }
`;

const SET_PLAYER_CARD = gql`
  mutation SetPlayerCard($playerId: UUID!, $card: Int!) {
    setPlayerCard(card: $card, playerId: $playerId)
  }
`;

function App() {
  const { data: cardData } = useQuery<GetCards>(GET_CARDS);
  const { data: gameStateData } = useSubscription<GetGameState>(GET_GAME_STATE);

  const [getClientId, { data: registerData }] = useMutation<GetClientId>(
    REGISTER
  );
  const [setPlayerCard] = useMutation<SetPlayerCard>(SET_PLAYER_CARD);

  useEffect(
    () => {
      getClientId().catch((reason) => console.error(reason));
    },
    // eslint-disable-next-line
    []
  );

  const isCalling = !!gameStateData?.gameState.isCalling;
  const players = gameStateData?.gameState.players;
  const clientId = registerData?.register;
  const player = players && clientId && players.find((x) => x.id === clientId);
  const cards = cardData?.cards;

  const onSelectCard = useCallback(
    (card: number) => {
      setPlayerCard({
        variables: { playerId: clientId, card: card },
      }).catch((reason) => console.error(reason));
    },
    [setPlayerCard, clientId]
  );

  const handleCall = () => {}; // FIXME
  const isAdmin = true; // FIXME

  if (!cards || !player || !gameStateData) {
    return <h1>loading...</h1>;
  }

  return (
    <div className="container mx-auto flex flex-col space-y-4">
      <PlayerCards gameStateData={gameStateData} cards={cards} />
      <div>
        <label>Name:</label>
        <input />
      </div>
      <CardPicker
        cards={cards}
        playerName={player?.name}
        selectedCard={player?.selectedCard}
        onSelect={onSelectCard}
        isCalling={isCalling}
      />
      {isAdmin ? (
        <button className="btn-red" onClick={handleCall}>
          Call
        </button>
      ) : null}
    </div>
  );
}

export default App;
