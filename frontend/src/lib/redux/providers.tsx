"use client";
import { Provider } from "react-redux";
import { reduxStore, useSelector } from "./store";

export const Providers = (props: React.PropsWithChildren) => {
  return <Provider store={reduxStore}>{props.children}</Provider>;
};
