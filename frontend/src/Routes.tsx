import { FC, ReactNode, useContext } from "react";
import { Route, BrowserRouter, Routes } from "react-router-dom";
import { useRecoilValue } from "recoil";
import { authAtom } from "./atoms/auth";

const Router: FC<{ children: ReactNode }> = ({ children }) => {
  return (
    <BrowserRouter>
      <Routes>{children}</Routes>
    </BrowserRouter>
  );
};

export const PrivateRoute: FC<{ children: ReactNode }> = ({ children }) => {
  const auth = useRecoilValue(authAtom)
  if (!auth) {
    return <div>Loading</div>;
  }
  if (!auth.isLogin) {
    return <div>Not logged in</div>;
  }

  return children as JSX.Element;
};

export default Router;
