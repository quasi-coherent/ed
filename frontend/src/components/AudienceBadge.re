[@mel.module "./AudienceBadge.module.css"]
external styles: Js.t({..}) = "default";

[@react.component]
let make = (~audience: Api.audience) => {
  let cls =
    switch (audience) {
    | `Email => styles##email
    | `Slack => styles##slack
    | `Sms => styles##sms
    | `Unknown => styles##unknown
    };
  <span className={styles##badge ++ " " ++ cls}>
    {React.string(Api.audienceLabel(audience))}
  </span>;
};
