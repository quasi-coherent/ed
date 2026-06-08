/* Centralised URL <-> route mapping. */

type route =
  | Login
  | Upload
  | Profile
  | Messages
  | Generate
  | History
  | NotFound;

let toPath = (r: route): string =>
  switch (r) {
  | Login => "/login"
  | Upload => "/upload"
  | Profile => "/profile"
  | Messages => "/messages"
  | Generate => "/generate"
  | History => "/history"
  | NotFound => "/upload"
  };

let ofUrl = (url: ReasonReactRouter.url): route =>
  switch (url.path) {
  | ["login"] => Login
  | []
  | ["upload"] => Upload
  | ["profile"] => Profile
  | ["messages"] => Messages
  | ["generate"] => Generate
  | ["history"] => History
  | _ => NotFound
  };

let push = (r: route): unit => ReasonReactRouter.push(toPath(r));
