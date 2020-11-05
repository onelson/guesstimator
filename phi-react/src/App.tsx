import React, { useCallback, useEffect } from 'react';
import { PlayerCards } from './PlayerCards';
import { CardPicker } from './CardPicker';
import { gql, useMutation, useQuery, useSubscription } from '@apollo/client';
import { GetCards } from './__generated__/GetCards';
import { GetGameState } from './__generated__/GetGameState';
import { GetClientId } from './__generated__/GetClientId';
import { SetPlayerCard } from './__generated__/SetPlayerCard';
import { NameSetter } from './NameSetter';
import { SetPlayerName } from './__generated__/SetPlayerName';
import { CheckAdminKey } from './__generated__/CheckAdminKey';
import { RemovePlayer } from './__generated__/RemovePlayer';

const REGISTER = gql`
  mutation GetClientId {
    register
  }
`;

const REMOVE_PLAYER = gql`
  mutation RemovePlayer($playerId: UUID!) {
    removePlayer(playerId: $playerId)
  }
`;

const SET_PLAYER_NAME = gql`
  mutation SetPlayerName($playerId: UUID!, $name: String!) {
    setPlayerName(playerId: $playerId, name: $name)
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

const CHECK_ADMIN_KEY = gql`
  mutation CheckAdminKey($key: UUID!) {
    adminChallenge(key: $key)
  }
`;

const CALL = gql`
  mutation Call {
    call
  }
`;

const RESUME = gql`
  mutation Resume {
    resume
  }
`;

const RESET = gql`
  mutation Reset {
    reset
  }
`;

function App() {
  const qs = new URLSearchParams(window.location.search);
  const adminKey = qs.get('key');

  const [checkAdminKey, { data: adminChallengeData }] = useMutation<
    CheckAdminKey
  >(CHECK_ADMIN_KEY);

  const isAdmin = !!adminChallengeData?.adminChallenge;
  const { data: cardData } = useQuery<GetCards>(GET_CARDS);
  const { data: gameStateData } = useSubscription<GetGameState>(GET_GAME_STATE);

  const [getClientId, { data: registerData }] = useMutation<GetClientId>(
    REGISTER
  );
  const [setPlayerCard] = useMutation<SetPlayerCard>(SET_PLAYER_CARD);
  const [setPlayerName] = useMutation<SetPlayerName>(SET_PLAYER_NAME);
  const [removePlayer] = useMutation<RemovePlayer>(REMOVE_PLAYER);

  const [call] = useMutation(CALL);
  const [resume] = useMutation(RESUME);
  const [reset] = useMutation(RESET);

  useEffect(
    () => {
      getClientId().catch((reason) => console.error(reason));
      if (adminKey) {
        checkAdminKey({
          variables: {
            key: adminKey,
          },
        }).catch((reason) => console.error(reason));
      }
    },
    // eslint-disable-next-line
    []
  );

  const clientId = registerData?.register;
  useEffect(() => {
    window.addEventListener('beforeunload', () => {
      removePlayer({ variables: { playerId: clientId } }).catch((reason) =>
        console.error(reason)
      );
    });
  }, [clientId, removePlayer]);

  const isCalling = !!gameStateData?.gameState.isCalling;

  const toggleCalling = useCallback(() => {
    if (isCalling) {
      resume().catch((reason) => console.error(reason));
    } else {
      call().catch((reason) => console.error(reason));
    }
  }, [isCalling]);

  const players = gameStateData?.gameState.players;

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

  if (!cards || !player || !gameStateData) {
    return <h1>loading...</h1>;
  }

  return (
    <div className="container mx-auto flex flex-col space-y-4">
      <PlayerCards gameStateData={gameStateData} cards={cards} />
      <NameSetter
        onSubmit={(name) =>
          setPlayerName({
            variables: {
              playerId: clientId,
              name,
            },
          })
        }
      />
      <CardPicker
        cards={cards}
        playerName={player?.name}
        selectedCard={player?.selectedCard}
        onSelect={onSelectCard}
        isCalling={isCalling}
      />
      {isAdmin ? (
        <div className="flex space-x-4">
          <button className="btn-red flex-grow" onClick={toggleCalling}>
            Call
          </button>
          <button className="btn-red flex-grow" onClick={() => reset()}>
            Reset
          </button>
        </div>
      ) : null}
    </div>
  );
}

export default App;
