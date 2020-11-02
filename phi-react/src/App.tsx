import React, { useCallback, useEffect } from 'react';
import { PlayerCards } from './PlayerCards';
import { CardPicker } from './CardPicker';
import { useQuery, gql, useMutation } from '@apollo/client';

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
  query GetGameState {
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

type Player = { id: string; selectedCard: number | undefined; name: string };

function getCurrentPlayer(
  players: Player[],
  clientId: string
): Player | undefined {
  return players.find((x) => x.id === clientId);
}

function App() {
  const { loading, error, data: cardData } = useQuery(GET_CARDS);
  const { data: gameStateData, refetch: refetchGameState } = useQuery(
    GET_GAME_STATE
  );

  const [getClientId, { data: registerData }] = useMutation(REGISTER);
  const [setPlayerCard] = useMutation(SET_PLAYER_CARD);

  useEffect(
    () => {
      getClientId()
        .then(refetchGameState)
        .catch((reason) => console.error(reason));
    },
    // eslint-disable-next-line
    []
  );

  const isCalling = gameStateData?.gameState.isCalling;

  const players: Player[] | undefined = gameStateData?.gameState.players;
  const clientId = registerData?.register;
  const player = players && clientId && getCurrentPlayer(players, clientId);
  const cards = cardData?.cards;

  const onSelectCard = useCallback(
    (card: number) => {
      setPlayerCard({
        variables: { playerId: clientId, card: card },
      })
        .then(refetchGameState)
        .catch((reason) => console.error(reason));
    },
    [setPlayerCard, clientId, refetchGameState]
  );

  const handleCall = () => {}; // FIXME
  const isAdmin = true; // FIXME

  if (!cards || !player) {
    return <h1>loading...</h1>;
  }

  return (
    <div className="container mx-auto flex flex-col space-y-4">
      <PlayerCards />
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
