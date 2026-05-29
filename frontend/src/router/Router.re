/* Centralised URL <-> route mapping. */

type route =
  | Upload
  | Profile
  | Messages
  | Generate
  | History
  | NotFound;

let toPath = (r: route): string =>
  switch (r) {
  | Upload => "/upload"
  | Profile => "/profile"
  | Messages => "/messages"
  | Generate => "/generate"
  | History => "/history"
  | NotFound => "/upload"
  };

let ofUrl = (url: ReasonReactRouter.url): route =>
  switch (url.path) {
  | []
  | ["upload"] => Upload
  | ["profile"] => Profile
  | ["messages"] => Messages
  | ["generate"] => Generate
  | ["history"] => History
  | _ => NotFound
  };

let push = (r: route): unit => ReasonReactRouter.push(toPath(r));
