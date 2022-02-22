import React from 'react';
import ReactDOM from 'react-dom';
import App from './App';
import './index.css';
import reportWebVitals from './reportWebVitals';
import {
  ApolloProvider,
  ApolloClient,
  InMemoryCache,
  HttpLink,
  // split,
} from '@apollo/client';
// import { WebSocketLink } from '@apollo/client/link/ws';
// import { getMainDefinition } from '@apollo/client/utilities';

function getClient() {
  const httpLink = new HttpLink({
    uri: '/gql',
  });

  // const proto = window.location.protocol === 'https:' ? 'wss:' : 'ws:';

  // const wsLink = new WebSocketLink({
  //   uri: `${proto}//${window.location.host}/gql`,
  //   options: {
  //     reconnect: true,
  //   },
  // });

  // const splitLink = split(
  //   ({ query }) => {
  //     const definition = getMainDefinition(query);
  //     return (
  //       definition.kind === 'OperationDefinition' &&
  //       definition.operation === 'subscription'
  //     );
  //   },
  //   wsLink,
  //   httpLink
  // );

  return new ApolloClient({
    cache: new InMemoryCache(),
    link: httpLink,
  });
}

ReactDOM.render(
  <React.StrictMode>
    <ApolloProvider client={getClient()}>
      <App />
    </ApolloProvider>
  </React.StrictMode>,
  document.getElementById('root')
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
