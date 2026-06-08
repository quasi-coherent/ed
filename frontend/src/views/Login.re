[@mel.module "./Login.module.css"] external styles: Js.t({..}) = "default";

type provider = {
  name: string,
  login_url: string,
};

type status =
  | Loading
  | Ready(list(provider))
  | Failed(string);

[@react.component]
let make = () => {
  let (status, setStatus) = React.useState(() => Loading);

  React.useEffect0(() => {
    setStatus(_ => Failed("asdf"));
    None;
  });

  let body =
    switch (status) {
    | Loading => <p> {React.string("Loading providers")} </p>
    | Failed(msg) => <p> {React.string("Error: " ++ msg)} </p>
    | Ready(providers) =>
      <ul>
        {providers
         |> List.map(p =>
              <li key={p.name}>
                <a href={p.login_url}>
                  {React.string("Log in with " ++ p.name)}
                </a>
              </li>
            )
         |> Array.of_list
         |> React.array}
      </ul>
    };

  <section className={styles##login}>
    <h1> {React.string("Sign in")} </h1>
    body
  </section>;
};
