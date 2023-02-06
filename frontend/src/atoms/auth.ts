import { atom } from "recoil";

export const authAtom = atom({
  key: "auth",
  default: {
    isLogin: false,
    name: "",
    user_id: "",
  },
});
