[@mel.module "./Nav.module.css"] external styles: Js.t({..}) = "default";

let cx = (parts: list(string)) =>
  String.concat(" ", List.filter(s => String.length(s) > 0, parts));

[@react.component]
let make = (~current: Router.route) => {
  let corpus = CorpusContext.useCorpus();
  let enabled = CorpusContext.hasCorpus(corpus);

  let item = (~label: string, ~route: Router.route, ~always: bool=false, ()) => {
    let isActive = current == route;
    let isDisabled = !always && !enabled;
    let className =
      cx([
        styles##link,
        isActive ? styles##active : "",
        isDisabled ? styles##disabled : "",
      ]);
    <button
      key=label
      className
      disabled=isDisabled
      onClick={_ =>
        if (!isDisabled) {
          Router.push(route);
        }
      }>
      {React.string(label)}
    </button>;
  };

  <nav className={styles##nav}>
    <span className={styles##brand}>
      {React.string("Tone & Style Simulator")}
    </span>
    {item(~label="Upload", ~route=Upload, ~always=true, ())}
    {item(~label="Style Profile", ~route=Profile, ())}
    {item(~label="Messages", ~route=Messages, ())}
    {item(~label="Generate", ~route=Generate, ())}
    {item(~label="History", ~route=History, ())}
  </nav>;
};
