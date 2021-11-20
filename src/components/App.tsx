import React, {useRef} from "react";
import {FocusStyleManager,} from "@blueprintjs/core";
import {Handler, SpecialRelativity} from "./SpecialRelativity";

FocusStyleManager.onlyShowFocusOnTabs();

export const App: React.FC = () => {
  const rsRef = useRef(null as Handler | null);
  return (
    <SpecialRelativity ref={rsRef} />
  );
};
