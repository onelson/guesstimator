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
import { SendHeartbeat } from './__generated__/SendHeartbeat';

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

const SEND_HEARTBEAT = gql`
  mutation SendHeartbeat($playerId: UUID!) {
    heartbeat(playerId: $playerId)
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
  // FIXME: look at adding a separate page to ask for a player name.
  //  Should work as a "landing page", and leverage a cookie.
  //  Skip if the player name is already set.
  //  Add a way to logout too.
  //  Doesn't require a router, tbh. Just show/hide components.

  // FIXME: look at loading/error states for Queries instead

  const qs = new URLSearchParams(window.location.search);
  const adminKey = qs.get('key');

  const [checkAdminKey, { data: adminChallengeData }] =
    useMutation<CheckAdminKey>(CHECK_ADMIN_KEY);

  const isAdmin = !!adminChallengeData?.adminChallenge;
  const { data: cardData } = useQuery<GetCards>(GET_CARDS);
  const { data: gameStateData } = useSubscription<GetGameState>(GET_GAME_STATE);

  const [getClientId, { data: registerData }] =
    useMutation<GetClientId>(REGISTER);
  const [setPlayerCard] = useMutation<SetPlayerCard>(SET_PLAYER_CARD);
  const [setPlayerName] = useMutation<SetPlayerName>(SET_PLAYER_NAME);
  const [removePlayer] = useMutation<RemovePlayer>(REMOVE_PLAYER);
  const [sendHeartbeat] = useMutation<SendHeartbeat>(SEND_HEARTBEAT);

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
    // When a page unloads, try to remove the player from the game.
    //
    // This isn't always reliable since:
    // - the request to register can fire, and
    // - the page can unload before the client id comes back
    //
    // To cover this gap we also use heartbeats to see when the last contact
    // from a player was, then remove players that haven't phoned home within
    // some deadline.
    window.addEventListener('beforeunload', () => {
      removePlayer({ variables: { playerId: clientId } }).catch((reason) =>
        console.error(reason)
      );
    });

    const timer = window.setInterval(() => {
      sendHeartbeat({
        variables: { playerId: clientId },
      }).catch((reason) => {
        console.error(reason);
        // If the heartbeat fails, it could be because the server is down.
        // If the server is down, that probably means the clientId is stale, so
        // reload the page to try and get a new one.
        window.location.reload();
      });
    }, 3_000);

    return () => {
      window.clearTimeout(timer);
    };
  }, [clientId, removePlayer, sendHeartbeat]);

  const isCalling = !!gameStateData?.gameState.isCalling;

  const toggleCalling = useCallback(() => {
    if (isCalling) {
      resume().catch((reason) => console.error(reason));
    } else {
      call().catch((reason) => console.error(reason));
    }
  }, [isCalling, call, resume]);

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
        initialValue={player.name}
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
