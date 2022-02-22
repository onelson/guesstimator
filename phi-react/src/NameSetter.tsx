import React, { SyntheticEvent } from 'react';

type Props = {
  initialValue: string;
  onSubmit: (name: string) => void;
};

export function NameSetter(props: Props) {
  const handleSubmit = (event: SyntheticEvent) => {
    event.preventDefault();
    const playerName = (event as any).currentTarget.playerName.value;
    props.onSubmit(playerName);
  };

  return (
    <form onSubmit={handleSubmit}>
      <label>Name:</label>
      <input name="playerName" defaultValue={props.initialValue} />
    </form>
  );
}
